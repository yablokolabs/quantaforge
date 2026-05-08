use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchedulerError {
    #[error("no suitable backend for job: {0}")]
    NoSuitableBackend(String),
    #[error("resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),
    #[error("job {0} not found")]
    JobNotFound(String),
}
