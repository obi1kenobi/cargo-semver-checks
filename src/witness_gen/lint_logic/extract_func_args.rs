//! Logic for [`WitnessLogic::ExtractFuncArgs`](crate::query::WitnessLogic::ExtractFuncArgs)

use std::{collections::BTreeMap, sync::Arc};

use anyhow::Result;
use trustfall::FieldValue;

use crate::witness_gen::WitnessRustdocPaths;

pub(super) fn extract_func_args(
    witness_results: BTreeMap<Arc<str>, FieldValue>,
    rustdoc_paths: &WitnessRustdocPaths,
) -> Result<BTreeMap<Arc<str>, FieldValue>> {
    anyhow::bail!(
        "Failed intentionally: {:?}",
        witness_results.get("old_signature")
    )
}
