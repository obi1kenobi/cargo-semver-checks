use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use trustfall_core::ir::TransparentValue;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) enum RequiredSemverUpdate {
    Major,
    Minor,
}

/// A query that can be executed on a pair of rustdoc output files,
/// returning instances of a particular kind of semver violation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SemverQuery {
    pub(crate) id: String,

    pub(crate) human_readable_name: String,

    pub(crate) description: String,

    pub(crate) required_update: RequiredSemverUpdate,

    pub(crate) query: String,

    #[serde(default)]
    pub(crate) arguments: BTreeMap<String, TransparentValue>,

    /// The top-level error describing the semver violation that was detected.
    /// Even if multiple instances of this semver issue are found, this error
    /// message is displayed only at most once.
    pub(crate) error_message: String,

    /// Optional template that can be combined with each query output to produce
    /// a human-readable description of the specific semver violation that was discovered.
    #[serde(default)]
    pub(crate) per_result_error_template: Option<String>,
}

impl SemverQuery {
    pub(crate) fn all_queries() -> BTreeMap<String, SemverQuery> {
        let mut queries = BTreeMap::default();

        let query_text_contents = [include_str!("./queries/struct_missing.ron")];
        for query_text in query_text_contents {
            let query: SemverQuery = ron::from_str(query_text).expect("query failed to parse");
            let id_conflict = queries.insert(query.id.clone(), query);
            assert!(id_conflict.is_none(), "{:?}", id_conflict);
        }

        queries
    }
}

#[cfg(test)]
mod tests {
    use trustfall_core::frontend::parse;

    use crate::adapter::RustdocAdapter;

    use super::SemverQuery;

    #[test]
    fn all_queries_parse_correctly() {
        let schema = RustdocAdapter::schema();
        for semver_query in SemverQuery::all_queries().into_values() {
            let _ = parse(&schema, &semver_query.query).expect("not a valid query");
        }
    }
}
