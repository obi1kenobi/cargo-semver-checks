use std::{collections::BTreeMap, sync::Arc};

use anyhow::Result;
use itertools::{Either, Itertools};
use trustfall::FieldValue;

use crate::witness_gen::{SingleWitnessCheckExtraInfo, SingleWitnessCheckInfo};

/// Contains data about the result of running a specific witness check for all lint results. Includes
/// additional data if this lint runs with [`LintLogic::UseWitness`](crate::query::LintLogic::UseWitness)
/// logic, specific to the witness logic employed.
#[derive(Debug)]
pub(crate) struct WitnessCheckResult {
    pub check_results: WitnessChecksResultKind,

    /// Indicates if any check errored, or failed without being repurposed.
    pub has_errored_check: bool,

    /// Indicates if a breaking change was found using the witness check.
    pub breaking_change_found: bool,
}

impl WitnessCheckResult {
    pub fn is_standard_logic(&self) -> bool {
        match &self.check_results {
            WitnessChecksResultKind::Standard(_) => true,
            WitnessChecksResultKind::WitnessLogic(_) => false,
        }
    }

    /// Partitions the contained data to create a [`WitnessFailures`] report.
    pub fn get_errors_and_failures(&self) -> WitnessFailures<'_> {
        // Partition into witness runs that were run and collected a status, and those that never ran
        // due to an early error.
        let (statuses, errors): (Vec<_>, Vec<_>) = match &self.check_results {
            WitnessChecksResultKind::Standard(results) => {
                results.iter().map(Result::as_ref).partition_result()
            }
            WitnessChecksResultKind::WitnessLogic(WitnessLogicKinds::InjectedAdditionalValues(
                results,
            )) => results
                .iter()
                .map(|result| result.as_ref().map(|(status, _)| status))
                .partition_result(),
        };

        // Partition into witness statuses that failed (no breaking change) and those that errored.
        // Successes are discarded.
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

/// Report about a particular lint's witness failures. Contains vectors with errors and diagnostic
/// data.
pub(crate) struct WitnessFailures<'a> {
    pub errors: Vec<&'a anyhow::Error>,
    pub failed_status: Vec<&'a SingleWitnessCheckInfo>,
    pub errored_status: Vec<&'a SingleWitnessCheckExtraInfo>,
}

/// Contains data about the check statuses for all runs of a witness. Also contains additional data
/// as necessary about the results of witness logic that was run.
#[derive(Debug)]
pub(crate) enum WitnessChecksResultKind {
    Standard(Vec<Result<WitnessCheckStatus>>),
    WitnessLogic(WitnessLogicKinds),
}

/// Data about witness logic runs. May contain any form of additional data on top of the check status.
#[derive(Debug)]
pub(crate) enum WitnessLogicKinds {
    /// For any case where the only additional witness logic is the injection of one or more values into
    /// the outputted lint values
    InjectedAdditionalValues(WitnessLogicKind<BTreeMap<Arc<str>, FieldValue>>),
}

type WitnessLogicKind<T> = Vec<Result<(WitnessCheckStatus, T)>>;

/// The resulting status of a successfully run witness check. Specifically, indicates if a breaking change
/// was discovered or not.
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

    pub(crate) fn is_errored(&self) -> bool {
        matches!(self, WitnessCheckStatus::ErroredCase { .. })
    }
}
