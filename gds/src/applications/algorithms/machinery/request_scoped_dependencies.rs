use crate::termination::TerminationFlag;

/// Request-scoped dependencies that are available throughout a request.
/// This is a core pattern in the Applications system for managing
/// dependencies that need to be shared across different components.
#[derive(Clone)]
pub struct RequestScopedDependencies {
    termination_flag: TerminationFlag,
}

impl RequestScopedDependencies {
    pub fn new(termination_flag: TerminationFlag) -> Self {
        Self { termination_flag }
    }

    pub fn termination_flag(&self) -> &TerminationFlag {
        &self.termination_flag
    }
}

impl Default for RequestScopedDependencies {
    fn default() -> Self {
        Self {
            termination_flag: TerminationFlag::running_true(),
        }
    }
}
