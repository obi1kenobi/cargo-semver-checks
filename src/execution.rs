use std::{collections::BTreeMap, sync::Arc, iter::Peekable, cell::RefCell, rc::Rc};

use anyhow::Context;
use trustfall_core::{frontend::parse, ir::{indexed::IndexedQuery, FieldValue}, schema::Schema, interpreter::execution::interpret_ir};

use crate::query::SemverQuery;

type QueryResultItem = BTreeMap<Arc<str>, FieldValue>;

fn get_parsed_query_and_args(schema: &Schema, semver_query: &SemverQuery) -> (Arc<IndexedQuery>, Arc<QueryResultItem>) {
    let parsed_query = parse(schema, &semver_query.query)
        .expect("not a valid query, should have been caught in tests");
    let args = Arc::new(
        semver_query
            .arguments
            .iter()
            .map(|(k, v)| (Arc::from(k.clone()), v.clone().into()))
            .collect(),
    );

    (parsed_query, args)
}

pub(crate) trait Queriable<'a> {
    fn run_query(&self, semver_query: &SemverQuery) -> anyhow::Result<Peekable<Box<dyn Iterator<Item = QueryResultItem> + 'a>>>;
}

pub(crate) struct QueriableRustdocV18<'a> {
    schema: Schema,
    adapter: Rc<RefCell<crate::rustdoc_v18::adapter::RustdocAdapter<'a>>>,
}

impl<'a> QueriableRustdocV18<'a> {
    pub(crate) fn new(schema: Schema, adapter: Rc<RefCell<crate::rustdoc_v18::adapter::RustdocAdapter<'a>>>) -> Self {
        Self {
            schema,
            adapter,
        }
    }
}

impl<'a> Queriable<'a> for QueriableRustdocV18<'a> {
    fn run_query(&self, semver_query: &SemverQuery) -> anyhow::Result<Peekable<Box<dyn Iterator<Item = QueryResultItem> + 'a>>> {
        let (parsed_query, args) = get_parsed_query_and_args(&self.schema, semver_query);

        let results_iter = interpret_ir(self.adapter.clone(), parsed_query, args)
            .with_context(|| "Query execution error.")?
            .peekable();

        Ok(results_iter)
    }
}

pub(crate) struct QueriableRustdocV21<'a> {
    schema: Schema,
    adapter: Rc<RefCell<crate::rustdoc_v21::adapter::RustdocAdapter<'a>>>,
}

impl<'a> QueriableRustdocV21<'a> {
    pub(crate) fn new(schema: Schema, adapter: Rc<RefCell<crate::rustdoc_v21::adapter::RustdocAdapter<'a>>>) -> Self {
        Self {
            schema,
            adapter,
        }
    }
}

impl<'a> Queriable<'a> for QueriableRustdocV21<'a> {
    fn run_query(&self, semver_query: &SemverQuery) -> anyhow::Result<Peekable<Box<dyn Iterator<Item = QueryResultItem> + 'a>>> {
        let (parsed_query, args) = get_parsed_query_and_args(&self.schema, semver_query);

        let results_iter = interpret_ir(self.adapter.clone(), parsed_query, args)
            .with_context(|| "Query execution error.")?
            .peekable();

        Ok(results_iter)
    }
}
