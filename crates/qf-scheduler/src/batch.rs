use crate::estimator::ResourceEstimate;
use crate::planner::{JobPlanner, JobStatus};
use qf_circuit::serialization::from_json;

pub struct BatchExecutor;

impl BatchExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute_batch(&self, planner: &mut JobPlanner) -> Vec<String> {
        let queued_ids: Vec<String> = planner
            .pending_jobs()
            .iter()
            .map(|j| j.id.clone())
            .collect();

        let mut completed = Vec::new();

        for id in queued_ids {
            if let Some(job) = planner.get_job_mut(&id) {
                job.status = JobStatus::Running;

                let result_json = match from_json(&job.circuit_json) {
                    Ok(circuit) => {
                        let estimate = ResourceEstimate::from_circuit(&circuit);
                        serde_json::to_string(&estimate)
                            .unwrap_or_else(|_| r#"{"error":"serialization failed"}"#.to_string())
                    }
                    Err(e) => {
                        job.status = JobStatus::Failed;
                        job.result = Some(format!(r#"{{"error":"{}"}}"#, e));
                        continue;
                    }
                };

                job.status = JobStatus::Completed;
                job.result = Some(result_json);
                completed.push(id);
            }
        }

        completed
    }
}

impl Default for BatchExecutor {
    fn default() -> Self {
        Self::new()
    }
}
