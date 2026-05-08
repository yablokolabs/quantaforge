use crate::aggregator::{AggregatedResult, ResultAggregator};
use crate::classifier::{TaskSpec, WorkloadClassifier};
use crate::error::AgentError;
use crate::router::TaskRouter;
use crate::solver::SolverResult;

pub struct OrchestrationEngine {
    classifier: WorkloadClassifier,
    router: TaskRouter,
    aggregator: ResultAggregator,
}

impl OrchestrationEngine {
    pub fn new() -> Self {
        Self {
            classifier: WorkloadClassifier::new(),
            router: TaskRouter::new(),
            aggregator: ResultAggregator::new(),
        }
    }

    pub fn process_task(&self, task: TaskSpec) -> Result<AggregatedResult, AgentError> {
        let domain = self.classifier.classify(&task);

        let solver = self
            .router
            .route(domain)
            .ok_or_else(|| AgentError::UnknownDomain(format!("{:?}", domain)))?;

        let result: SolverResult = solver.solve(&task)?;
        Ok(self.aggregator.aggregate(vec![result]))
    }

    pub fn process_workflow(&self, tasks: Vec<TaskSpec>) -> Result<AggregatedResult, AgentError> {
        let mut results = Vec::new();

        for task in tasks {
            let domain = self.classifier.classify(&task);

            let solver = self
                .router
                .route(domain)
                .ok_or_else(|| AgentError::UnknownDomain(format!("{:?}", domain)))?;

            match solver.solve(&task) {
                Ok(result) => results.push(result),
                Err(e) => {
                    return Err(AgentError::WorkflowStepFailed(e.to_string()));
                }
            }
        }

        Ok(self.aggregator.aggregate(results))
    }
}

impl Default for OrchestrationEngine {
    fn default() -> Self {
        Self::new()
    }
}
