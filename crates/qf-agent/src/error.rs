use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("unknown task domain: {0}")]
    UnknownDomain(String),
    #[error("solver {0} failed: {1}")]
    SolverFailure(String, String),
    #[error("workflow step {0} failed")]
    WorkflowStepFailed(String),
    #[error(transparent)]
    SchedulerError(#[from] qf_scheduler::error::SchedulerError),
}
