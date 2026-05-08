use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskDomain {
    Simulation,
    Optimization,
    Verification,
    ErrorCorrection,
    ResourceEstimation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSpec {
    pub description: String,
    pub domain: Option<TaskDomain>,
    pub circuit_json: Option<String>,
    pub parameters: HashMap<String, String>,
}

pub struct WorkloadClassifier;

impl WorkloadClassifier {
    pub fn new() -> Self {
        Self
    }

    pub fn classify(&self, task: &TaskSpec) -> TaskDomain {
        if let Some(domain) = task.domain {
            return domain;
        }

        let desc = task.description.to_lowercase();

        if desc.contains("simulate") || desc.contains("run") || desc.contains("execute") {
            TaskDomain::Simulation
        } else if desc.contains("optimize") || desc.contains("reduce") || desc.contains("minimize")
        {
            TaskDomain::Optimization
        } else if desc.contains("verify") || desc.contains("prove") || desc.contains("check") {
            TaskDomain::Verification
        } else if desc.contains("error")
            || desc.contains("correct")
            || desc.contains("surface")
            || desc.contains("qec")
        {
            TaskDomain::ErrorCorrection
        } else if desc.contains("resource") || desc.contains("estimate") || desc.contains("cost") {
            TaskDomain::ResourceEstimation
        } else {
            TaskDomain::Simulation
        }
    }
}

impl Default for WorkloadClassifier {
    fn default() -> Self {
        Self::new()
    }
}
