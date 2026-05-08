use std::collections::HashMap;

use qf_agent::{
    OrchestrationEngine, SolverStatus, TaskDomain, TaskRouter, TaskSpec, WorkloadClassifier,
};

fn make_task(description: &str) -> TaskSpec {
    TaskSpec {
        description: description.to_string(),
        domain: None,
        circuit_json: None,
        parameters: HashMap::new(),
    }
}

fn make_task_with_domain(description: &str, domain: TaskDomain) -> TaskSpec {
    TaskSpec {
        description: description.to_string(),
        domain: Some(domain),
        circuit_json: None,
        parameters: HashMap::new(),
    }
}

#[test]
fn classifier_simulate() {
    let classifier = WorkloadClassifier::new();
    let task = make_task("simulate this circuit");
    assert_eq!(classifier.classify(&task), TaskDomain::Simulation);
}

#[test]
fn classifier_optimize() {
    let classifier = WorkloadClassifier::new();
    let task = make_task("optimize gate count");
    assert_eq!(classifier.classify(&task), TaskDomain::Optimization);
}

#[test]
fn classifier_verify() {
    let classifier = WorkloadClassifier::new();
    let task = make_task("verify correctness");
    assert_eq!(classifier.classify(&task), TaskDomain::Verification);
}

#[test]
fn classifier_error_correction() {
    let classifier = WorkloadClassifier::new();
    let task = make_task("error correction experiment");
    assert_eq!(classifier.classify(&task), TaskDomain::ErrorCorrection);
}

#[test]
fn classifier_explicit_domain_overrides() {
    let classifier = WorkloadClassifier::new();
    let task = make_task_with_domain("optimize gate count", TaskDomain::Verification);
    assert_eq!(classifier.classify(&task), TaskDomain::Verification);
}

#[test]
fn router_simulation_to_simulation_solver() {
    let router = TaskRouter::new();
    let solver = router.route(TaskDomain::Simulation).unwrap();
    assert_eq!(solver.name(), "SimulationSolver");
}

#[test]
fn engine_process_task_success() {
    let engine = OrchestrationEngine::new();
    let task = make_task("simulate this circuit");
    let result = engine.process_task(task).unwrap();
    assert_eq!(result.total_steps, 1);
    assert_eq!(result.successful_steps, 1);
    assert_eq!(result.failed_steps, 0);
    assert_eq!(result.outputs.len(), 1);
    assert_eq!(result.outputs[0].status, SolverStatus::Success);
}

#[test]
fn engine_process_workflow_three_tasks() {
    let engine = OrchestrationEngine::new();
    let tasks = vec![
        make_task("simulate this circuit"),
        make_task("optimize gate count"),
        make_task("verify correctness"),
    ];
    let result = engine.process_workflow(tasks).unwrap();
    assert_eq!(result.total_steps, 3);
    assert_eq!(result.successful_steps, 3);
    assert_eq!(result.outputs.len(), 3);
}

#[test]
fn engine_unknown_domain_falls_back_to_simulation() {
    let engine = OrchestrationEngine::new();
    let task = make_task("do something unrelated");
    let result = engine.process_task(task).unwrap();
    assert_eq!(result.total_steps, 1);
    assert_eq!(result.successful_steps, 1);
    assert_eq!(result.outputs[0].solver_name, "SimulationSolver");
}
