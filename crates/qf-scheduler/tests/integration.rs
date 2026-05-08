use qf_circuit::{Circuit, Gate, Instruction};
use qf_scheduler::{Backend, BatchExecutor, JobPlanner, JobStatus, ResourceEstimate, RuntimeClass};

fn make_circuit(num_qubits: usize, instructions: Vec<Instruction>) -> Circuit {
    let mut c = Circuit::new(num_qubits).unwrap();
    for instr in instructions {
        c.add_instruction(instr).unwrap();
    }
    c
}

fn circuit_to_json(circuit: &Circuit) -> String {
    qf_circuit::serialization::to_json(circuit).unwrap()
}

#[test]
fn resource_estimate_10_qubit_statevector() {
    let circuit = make_circuit(
        10,
        vec![
            Instruction::new(Gate::H, vec![0]).unwrap(),
            Instruction::new(Gate::Rx(1.0), vec![1]).unwrap(),
            Instruction::new(Gate::CNOT, vec![0, 1]).unwrap(),
        ],
    );
    let est = ResourceEstimate::from_circuit(&circuit);
    assert_eq!(est.recommended_backend, Backend::StateVector);
    assert_eq!(est.estimated_runtime_class, RuntimeClass::Instant);
    assert_eq!(est.num_qubits, 10);
    assert_eq!(est.gate_count, 3);
}

#[test]
fn resource_estimate_clifford_only_stabilizer() {
    let circuit = make_circuit(
        5,
        vec![
            Instruction::new(Gate::H, vec![0]).unwrap(),
            Instruction::new(Gate::S, vec![1]).unwrap(),
            Instruction::new(Gate::CNOT, vec![0, 1]).unwrap(),
            Instruction::new(Gate::X, vec![2]).unwrap(),
        ],
    );
    let est = ResourceEstimate::from_circuit(&circuit);
    assert_eq!(est.recommended_backend, Backend::Stabilizer);
}

#[test]
fn resource_estimate_31_qubit_non_clifford_infeasible() {
    let circuit = make_circuit(31, vec![Instruction::new(Gate::T, vec![0]).unwrap()]);
    let est = ResourceEstimate::from_circuit(&circuit);
    assert_eq!(est.estimated_runtime_class, RuntimeClass::Infeasible);
    assert_eq!(est.recommended_backend, Backend::TensorNetwork);
}

#[test]
fn job_creation_queued_with_id() {
    let job = qf_scheduler::Job::new("{}".to_string(), 1024);
    assert_eq!(job.status, JobStatus::Queued);
    assert!(!job.id.is_empty());
    assert_eq!(job.shots, 1024);
}

#[test]
fn job_planner_submit_and_pending() {
    let mut planner = JobPlanner::new();
    planner.submit("{}".to_string(), 100);
    planner.submit("{}".to_string(), 200);
    planner.submit("{}".to_string(), 300);
    assert_eq!(planner.pending_jobs().len(), 3);
    assert_eq!(planner.all_jobs().len(), 3);
}

#[test]
fn batch_executor_processes_all_queued() {
    let circuit = make_circuit(
        3,
        vec![
            Instruction::new(Gate::H, vec![0]).unwrap(),
            Instruction::new(Gate::CNOT, vec![0, 1]).unwrap(),
        ],
    );
    let json = circuit_to_json(&circuit);

    let mut planner = JobPlanner::new();
    let id1 = planner.submit(json.clone(), 100);
    let id2 = planner.submit(json.clone(), 200);
    let id3 = planner.submit(json, 300);

    let executor = BatchExecutor::new();
    let completed = executor.execute_batch(&mut planner);

    assert_eq!(completed.len(), 3);
    assert!(completed.contains(&id1));
    assert!(completed.contains(&id2));
    assert!(completed.contains(&id3));

    for id in &completed {
        let job = planner.get_job(id).unwrap();
        assert_eq!(job.status, JobStatus::Completed);
        assert!(job.result.is_some());
    }

    assert_eq!(planner.pending_jobs().len(), 0);
}
