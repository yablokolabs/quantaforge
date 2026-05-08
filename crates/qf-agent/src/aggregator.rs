use serde::{Deserialize, Serialize};

use crate::solver::{SolverResult, SolverStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedResult {
    pub total_steps: usize,
    pub successful_steps: usize,
    pub failed_steps: usize,
    pub outputs: Vec<SolverResult>,
    pub summary: String,
}

pub struct ResultAggregator;

impl ResultAggregator {
    pub fn new() -> Self {
        Self
    }

    pub fn aggregate(&self, results: Vec<SolverResult>) -> AggregatedResult {
        let total = results.len();
        let successful = results
            .iter()
            .filter(|r| r.status == SolverStatus::Success)
            .count();
        let failed = results
            .iter()
            .filter(|r| r.status == SolverStatus::Failed)
            .count();

        let summary = format!("{}/{} steps completed successfully", successful, total);

        AggregatedResult {
            total_steps: total,
            successful_steps: successful,
            failed_steps: failed,
            outputs: results,
            summary,
        }
    }
}

impl Default for ResultAggregator {
    fn default() -> Self {
        Self::new()
    }
}
