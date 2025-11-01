use std::{collections::BTreeMap, sync::Arc};

use anyhow::Result;
use itertools::{Either, Itertools};
use trustfall::FieldValue;

use crate::witness_gen::{SingleWitnessCheckExtraInfo, SingleWitnessCheckInfo};

#[derive(Debug)]
pub(crate) struct WitnessCheckResult {
    pub check_results: WitnessChecksResultKind,
    pub has_errored_check: bool,
    pub breaking_change_found: bool,
}

impl WitnessCheckResult {
    pub fn is_standard_logic(&self) -> bool {
        match &self.check_results {
            WitnessChecksResultKind::Standard(_) => true,
            WitnessChecksResultKind::WitnessLogic(_) => false,
        }
    }

    pub fn get_errors_and_failures(&self) -> WitnessFailures<'_> {
        let (statuses, errors): (Vec<_>, Vec<_>) = match &self.check_results {
            WitnessChecksResultKind::Standard(results) => {
                results.iter().map(Result::as_ref).partition_result()
            }
            WitnessChecksResultKind::WitnessLogic(WitnessLogicKinds::ExtractFuncArgs(results)) => {
                results
                    .iter()
                    .map(|result| result.as_ref().map(|(status, _)| status))
                    .partition_result()
            }
        };

        let (failed_status, errored_status): (Vec<_>, Vec<_>) = statuses
            .iter()
            .filter_map(|status| match status {
                WitnessCheckStatus::BreakingChange => None,
                WitnessCheckStatus::NoBreakingChange(info) => Some(Either::Left(info)),
                WitnessCheckStatus::ErroredCase(info) => Some(Either::Right(info)),
            })
            .partition_map(|either| either);

        WitnessFailures {
            errors,
            failed_status,
            errored_status,
        }
    }
}

pub(crate) struct WitnessFailures<'a> {
    pub errors: Vec<&'a anyhow::Error>,
    pub failed_status: Vec<&'a SingleWitnessCheckInfo>,
    pub errored_status: Vec<&'a SingleWitnessCheckExtraInfo>,
}

#[derive(Debug)]
pub(crate) enum WitnessChecksResultKind {
    Standard(Vec<Result<WitnessCheckStatus>>),
    WitnessLogic(WitnessLogicKinds),
}

#[derive(Debug)]
pub(crate) enum WitnessLogicKinds {
    ExtractFuncArgs(WitnessLogicKind<BTreeMap<Arc<str>, FieldValue>>),
}

type WitnessLogicKind<T> = Vec<Result<(WitnessCheckStatus, T)>>;

#[derive(Debug)]
pub(crate) enum WitnessCheckStatus {
    /// Indicates that `baseline` checked but `current` did not, which is the expected result
    BreakingChange,

    /// Indicated that the witness always checked successfully.
    ///
    /// Includes some diagnostic information.
    NoBreakingChange(SingleWitnessCheckInfo),

    /// Indicates that the witness never checked successfully, or broke in an inverted manner, both of which are errors.
    ///
    /// Includes some diagnostic information.
    ErroredCase(SingleWitnessCheckExtraInfo),
}

impl WitnessCheckStatus {
    pub(crate) fn is_breaking(&self) -> bool {
        matches!(self, WitnessCheckStatus::BreakingChange)
    }

    pub(crate) fn errored(&self) -> bool {
        matches!(self, WitnessCheckStatus::ErroredCase { .. })
    }
}
