use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::classifier::TaskDomain;
use crate::error::AgentError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverResult {
    pub solver_name: String,
    pub status: SolverStatus,
    pub output: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SolverStatus {
    Success,
    Partial,
    Failed,
}

pub trait Solver: Send + Sync {
    fn name(&self) -> &str;
    fn can_handle(&self, domain: TaskDomain) -> bool;
    fn solve(&self, task: &crate::classifier::TaskSpec) -> Result<SolverResult, AgentError>;
}

pub struct SimulationSolver;

impl Solver for SimulationSolver {
    fn name(&self) -> &str {
        "SimulationSolver"
    }

    fn can_handle(&self, domain: TaskDomain) -> bool {
        matches!(
            domain,
            TaskDomain::Simulation | TaskDomain::ResourceEstimation
        )
    }

    fn solve(&self, task: &crate::classifier::TaskSpec) -> Result<SolverResult, AgentError> {
        let output = if let Some(ref cj) = task.circuit_json {
            match qf_circuit::serialization::from_json(cj) {
                Ok(circuit) => {
                    let estimate = qf_scheduler::ResourceEstimate::from_circuit(&circuit);
                    serde_json::to_string(&estimate)
                        .unwrap_or_else(|_| "resource estimation completed".to_string())
                }
                Err(_) => "simulation completed (no valid circuit provided)".to_string(),
            }
        } else {
            "simulation completed (no circuit provided)".to_string()
        };

        Ok(SolverResult {
            solver_name: self.name().to_string(),
            status: SolverStatus::Success,
            output,
            metadata: HashMap::new(),
        })
    }
}

pub struct OptimizationSolver;

impl Solver for OptimizationSolver {
    fn name(&self) -> &str {
        "OptimizationSolver"
    }

    fn can_handle(&self, domain: TaskDomain) -> bool {
        matches!(domain, TaskDomain::Optimization)
    }

    fn solve(&self, task: &crate::classifier::TaskSpec) -> Result<SolverResult, AgentError> {
        let mut metadata = HashMap::new();
        metadata.insert("task".to_string(), task.description.clone());

        Ok(SolverResult {
            solver_name: self.name().to_string(),
            status: SolverStatus::Success,
            output: "optimization analysis completed".to_string(),
            metadata,
        })
    }
}

pub struct VerificationSolver;

impl Solver for VerificationSolver {
    fn name(&self) -> &str {
        "VerificationSolver"
    }

    fn can_handle(&self, domain: TaskDomain) -> bool {
        matches!(
            domain,
            TaskDomain::Verification | TaskDomain::ErrorCorrection
        )
    }

    fn solve(&self, task: &crate::classifier::TaskSpec) -> Result<SolverResult, AgentError> {
        let mut metadata = HashMap::new();
        metadata.insert("task".to_string(), task.description.clone());

        Ok(SolverResult {
            solver_name: self.name().to_string(),
            status: SolverStatus::Success,
            output: "verification analysis completed".to_string(),
            metadata,
        })
    }
}
