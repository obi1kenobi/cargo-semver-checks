#![no_std]

#![deny(missing_docs, rustdoc::broken_intra_doc_links)]

/// Rust doc comment with an unescaped HTML <tag> that rustdoc will complain about.
///
/// Also, a link that points to nowhere: [`Foo::bar`].
pub struct Foo;

pub struct Undocumented;  // fails the `missing_docs` lint
