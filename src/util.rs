use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::Context as _;

pub(crate) const SCOPE: &str = "semver-checks";

/// Write a file through a temporary file in the destination directory.
///
/// This is useful when multiple processes may produce the same output path, or when multiple
/// separate write operations are required to finalize the file, and subsequent readers should not
/// observe half-written files. The destination file itself is not opened until after the closure has
/// successfully produced complete contents in a temporary file.
///
/// The operation has three phases:
///
/// 1. Create a temporary file next to the destination. Creating it in the same directory keeps the
///    temp file on the same filesystem as the destination, which is required for atomic renames on
///    Unix-like systems.
/// 2. Wrap that temp file in a buffered writer and give the writer to the closure. If the closure
///    returns an error, the destination is left untouched and the temp file is removed.
/// 3. Flush and close the buffered temp file, then move it to the destination path. On Unix,
///    `rename` replaces the destination atomically when source and destination are on the same
///    filesystem: other processes see either the old file or the new file, not a partially-written
///    mixture.
///
/// Some platforms and filesystems reject that final rename, especially when replacing an existing
/// destination. If the move fails, this falls back to opening the destination path directly,
/// truncating it, and copying the completed temp-file contents into it. That fallback is not atomic,
/// but it preserves the caller's request to write the file when the platform cannot provide the
/// stronger move operation.
///
/// This helper is about preventing partial contents from being published by normal successful
/// writes. It does not `fsync` the file or directory, so it does not claim crash-durability after an
/// OS or power failure.
///
/// The destination's parent directory must already exist and is canonicalized before use. The
/// temporary file name includes random bits and is opened with `create_new`, so an existing path is
/// never followed or reused. On Unix, the temporary file is created with mode `0o600` so only the
/// current user can read or write it while it is still unpublished.
pub(crate) fn atomic_write(
    destination: impl AsRef<Path>,
    write_contents: impl FnOnce(&mut dyn Write) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let destination = destination.as_ref();
    let file_name = destination
        .file_name()
        .with_context(|| format!("destination is not a file path: {}", destination.display()))?
        .to_string_lossy();
    let parent = destination
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .with_context(|| {
            format!(
                "destination has no parent directory: {}",
                destination.display()
            )
        })?;
    let parent = fs_err::canonicalize(parent)
        .with_context(|| format!("failed to canonicalize {}", parent.display()))?;

    // Try a bounded number of random names. A collision is already extremely unlikely, but
    // `create_new` makes the check race-free: the OS creates the file only if the path does not
    // already exist, so a concurrent writer cannot cause us to open its file by mistake.
    let mut created_temp_file = None;
    for _ in 0..4 {
        let temp_path = parent.join(format!(
            ".{file_name}.tmp-{}-{:032x}",
            std::process::id(),
            rand::random::<u128>(),
        ));
        let mut options = fs_err::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            // The temp file may contain complete rustdoc JSON, manifests, or other cache data
            // before it is renamed into place. Keep that unpublished file private even if the
            // process umask would otherwise allow broader access.
            use fs_err::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }

        match options.open(&temp_path) {
            Ok(file) => {
                created_temp_file = Some((temp_path, file));
                break;
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("failed to create {}", temp_path.display()));
            }
        }
    }
    let (temp_path, temp_file) = created_temp_file.with_context(|| {
        format!(
            "failed to create a unique temporary file for {}",
            destination.display()
        )
    })?;
    let mut temp_file = BufWriter::new(temp_file);

    // The closure can validate and serialize the entire output before the destination path is
    // touched. If it fails, remove the temp file and return the original error so callers see the
    // serialization/build failure rather than cleanup noise.
    if let Err(error) = write_contents(&mut temp_file) {
        drop(temp_file);
        let _ = fs_err::remove_file(&temp_path);
        return Err(error);
    }

    // Flush the userspace buffer before publishing the temp file. This is not an fsync: it pushes
    // data through Rust/std buffering to the OS, but does not guarantee persistence after a crash.
    if let Err(error) = temp_file.flush() {
        drop(temp_file);
        let _ = fs_err::remove_file(&temp_path);
        return Err(error).with_context(|| format!("failed to flush {}", temp_path.display()));
    }
    let temp_file = match temp_file.into_inner() {
        Ok(file) => file,
        Err(error) => {
            let error = error.into_error();
            let _ = fs_err::remove_file(&temp_path);
            return Err(error).with_context(|| format!("failed to flush {}", temp_path.display()));
        }
    };
    drop(temp_file);

    match fs_err::rename(&temp_path, destination) {
        Ok(()) => Ok(()),
        Err(rename_error) => {
            // `rename` is the atomic path, but it is not uniformly capable of replacing an
            // existing file across all supported platforms. Windows' standard rename semantics are
            // stricter than Unix's, and some filesystems can reject replacement-style renames too.
            // At this point the temp file contains complete output, so fall back to publishing those
            // bytes by truncating and writing the destination directly.
            let fallback_result = (|| -> anyhow::Result<()> {
                let mut temp_file = fs_err::File::open(&temp_path).with_context(|| {
                    format!(
                        "failed to reopen temporary file {} after atomic rename failed",
                        temp_path.display()
                    )
                })?;
                let destination_file = fs_err::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(destination)
                    .with_context(|| {
                        format!(
                            "failed to open {} for direct write fallback",
                            destination.display()
                        )
                    })?;
                let mut destination_file = BufWriter::new(destination_file);
                // `fs_err::copy` is a path-to-path helper, so using it here would reopen the
                // destination itself and skip the buffered fallback writer we just configured.
                // `fs_err` does not wrap stream-to-stream copies, so use `std::io::copy` and add
                // the path context at this call site.
                std::io::copy(&mut temp_file, &mut destination_file).with_context(|| {
                    format!(
                        "failed to copy {} into {}",
                        temp_path.display(),
                        destination.display()
                    )
                })?;
                destination_file
                    .flush()
                    .with_context(|| format!("failed to flush {}", destination.display()))?;
                Ok(())
            })();

            // Do not let temp-file cleanup hide the result of the write fallback. If the fallback
            // failed, that is the actionable error; if it succeeded, leaving a temp file behind is
            // inconvenient but should not make an otherwise-successful write fail.
            let result = fallback_result.with_context(|| {
                format!(
                    "atomic rename from {} to {} failed: {rename_error}",
                    temp_path.display(),
                    destination.display()
                )
            });
            let _ = fs_err::remove_file(&temp_path);
            result
        }
    }
}

pub(crate) fn slugify(value: &str) -> String {
    value
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::atomic_write;

    struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "cargo-semver-checks-{name}-{}-{:032x}",
                std::process::id(),
                rand::random::<u128>(),
            ));
            fs_err::create_dir(&path).expect("failed to create test temp dir");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs_err::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn atomic_write_commits_successful_closure() -> anyhow::Result<()> {
        let temp_dir = TestDir::new("atomic-write-success");
        let destination = temp_dir.path().join("output.txt");

        atomic_write(&destination, |writer| {
            writer.write_all(b"existing contents")?;
            Ok(())
        })?;
        atomic_write(&destination, |writer| {
            writer.write_all(b"complete contents")?;
            Ok(())
        })?;

        assert_eq!(fs_err::read_to_string(&destination)?, "complete contents");
        Ok(())
    }

    #[test]
    fn atomic_write_aborts_when_closure_errors() -> anyhow::Result<()> {
        let temp_dir = TestDir::new("atomic-write-error");
        let destination = temp_dir.path().join("output.txt");
        atomic_write(&destination, |writer| {
            writer.write_all(b"existing contents")?;
            Ok(())
        })?;

        let error = atomic_write(&destination, |writer| {
            writer.write_all(b"partial contents")?;
            anyhow::bail!("closure failed");
        })
        .expect_err("closure error should abort the write");

        assert_eq!(error.to_string(), "closure failed");
        assert_eq!(fs_err::read_to_string(&destination)?, "existing contents");
        assert_eq!(
            fs_err::read_dir(temp_dir.path())?.count(),
            1,
            "temporary file should be removed after closure error"
        );
        Ok(())
    }
}
