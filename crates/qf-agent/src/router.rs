use crate::classifier::TaskDomain;
use crate::solver::{OptimizationSolver, SimulationSolver, Solver, VerificationSolver};

pub struct TaskRouter {
    solvers: Vec<Box<dyn Solver>>,
}

impl TaskRouter {
    pub fn new() -> Self {
        Self {
            solvers: vec![
                Box::new(SimulationSolver),
                Box::new(OptimizationSolver),
                Box::new(VerificationSolver),
            ],
        }
    }

    pub fn register_solver(&mut self, solver: Box<dyn Solver>) {
        self.solvers.push(solver);
    }

    pub fn route(&self, domain: TaskDomain) -> Option<&dyn Solver> {
        self.solvers
            .iter()
            .find(|s| s.can_handle(domain))
            .map(|s| s.as_ref())
    }
}

impl Default for TaskRouter {
    fn default() -> Self {
        Self::new()
    }
}
