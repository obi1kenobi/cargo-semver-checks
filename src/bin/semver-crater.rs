use clap::{Parser, ValueEnum};
use crates_io_api::{CratesQuery, Sort, Version};
use itertools::Itertools;
use std::time::Duration;

#[derive(ValueEnum, Clone, Debug)]
enum ReleaseFilter {
    CheckAllReleases,
    SkipAdjacentPatches,
    SkipAdjacentMinor,
}

impl ReleaseFilter {
    fn filter(&self, versions: Vec<Version>) -> Vec<(Version, Version)> {
        let versions = {
            let mut filtered: Vec<Version> = vec![versions
                .first()
                .expect("received empty list of versions")
                .clone()];
            filtered.extend(versions.windows(3).filter_map(|vec| {
                let mut iter = vec.iter();
                let version_v0 = semver::Version::parse(&iter.next().unwrap().num).unwrap();
                let v1 = iter.next().unwrap();
                let version_v2 = semver::Version::parse(&iter.next().unwrap().num).unwrap();
                assert!(iter.next().is_none());

                let keep_middle = match self {
                    Self::CheckAllReleases => true,
                    Self::SkipAdjacentPatches => {
                        version_v0.major != version_v2.major || version_v0.minor != version_v2.minor
                    }
                    Self::SkipAdjacentMinor => version_v0.major != version_v2.major,
                };
                match keep_middle {
                    true => Some(v1.clone()),
                    false => None,
                }
            }));
            filtered.push(
                versions
                    .last()
                    .expect("received empty list of versions")
                    .clone(),
            );
            filtered
        };
        versions.into_iter().tuple_windows().collect()
    }
}

#[derive(Parser, Debug)]
#[command(author)]
struct Args {
    #[arg(value_enum, default_value_t=ReleaseFilter::SkipAdjacentPatches)]
    release_filter: ReleaseFilter,

    #[arg(short, long)]
    crates: Vec<String>,
}

fn check_crate(
    client: &crates_io_api::SyncClient,
    release_filter: &ReleaseFilter,
    name: &str,
) -> anyhow::Result<()> {
    let versions = client.get_crate(name)?.versions;
    let versions: Vec<Version> = versions.into_iter().filter(|v| !v.yanked).collect();
    for (version_current, version_baseline) in release_filter.filter(versions) {
        println!(
            "{} {} -> {}",
            name, version_baseline.num, version_current.num
        );

        let report = cargo_semver_checks::Check::new(cargo_semver_checks::Rustdoc::from_registry(
            &version_current.num,
        ))
        .with_baseline(cargo_semver_checks::Rustdoc::from_registry(
            &version_baseline.num,
        ))
        .with_packages(vec![name.to_string()])
        .check_release()?;
        let _ = report;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let client = crates_io_api::SyncClient::new(
        // The format of the string and the duration
        // is documented in https://crates.io/policies#crawlers
        "crates_io_api: cargo-semver-checks (https://github.com/obi1kenobi/cargo-semver-checks/)",
        Duration::from_millis(1000),
    )?;

    match args.crates.is_empty() {
        true => {
            let mut query = CratesQuery::builder()
                .page_size(100)
                .sort(Sort::Downloads)
                .build();
            query.set_page(1);
            for crate_info in client.crates(query)?.crates.into_iter() {
                check_crate(&client, &args.release_filter, &crate_info.name)?;
            }
        }
        false => {
            for crate_name in args.crates {
                check_crate(&client, &args.release_filter, &crate_name)?;
            }
        }
    }

    Ok(())
}
