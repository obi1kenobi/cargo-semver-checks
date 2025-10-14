mod extract_func_args;

use std::{collections::BTreeMap, sync::Arc};

use anyhow::Result;
use trustfall::FieldValue;
use trustfall_rustdoc::VersionedRustdocAdapter;

use crate::{
    SemverQuery,
    query::{LintLogic, WitnessLogic},
    witness_gen::WitnessRustdocPaths,
};

pub(crate) enum WitnessLogicResult {
    ExtractFuncArgs(BTreeMap<Arc<str>, FieldValue>),
}

/// Runs any extra queries according to the [`SemverQuery`]'s [`LintLogic`].
///
/// Anything other than [`LintLogic::UseWitness`] implies a no-op.
pub(super) fn run_extra_witness_queries(
    _adapter: &VersionedRustdocAdapter,
    semver_query: &SemverQuery,
    witness_results: BTreeMap<Arc<str>, FieldValue>,
    rustdoc_paths: &WitnessRustdocPaths,
) -> Result<(BTreeMap<Arc<str>, FieldValue>, Option<WitnessLogicResult>)> {
    match semver_query.lint_logic {
        LintLogic::UseWitness(WitnessLogic::ExtractFuncArgs) => {
            extract_func_args::extract_func_args(witness_results, rustdoc_paths).map(|data| {
                (
                    data.clone(),
                    Some(WitnessLogicResult::ExtractFuncArgs(data)),
                )
            })
        }

        // No-op if no additional query logic is required
        _ => Ok((witness_results, None)),
    }
}
