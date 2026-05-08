use crate::circuit::Circuit;
use crate::error::CircuitError;
use crate::gate::Gate;
use crate::instruction::Instruction;

pub fn parse_circuit(input: &str, num_qubits: usize) -> Result<Circuit, CircuitError> {
    let mut circuit = Circuit::new(num_qubits)?;

    for (line_num, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        let parse_err =
            |msg: String| CircuitError::ParseError(format!("line {}: {}", line_num + 1, msg));

        let gate_name = tokens[0].to_uppercase();
        let (gate, expected_qubits) = match gate_name.as_str() {
            "H" => (Gate::H, 1),
            "X" => (Gate::X, 1),
            "Y" => (Gate::Y, 1),
            "Z" => (Gate::Z, 1),
            "S" => (Gate::S, 1),
            "T" => (Gate::T, 1),
            "CNOT" | "CX" => (Gate::CNOT, 2),
            "CZ" => (Gate::CZ, 2),
            "MEASURE" => (Gate::Measure, 1),
            "RX" => {
                if tokens.len() < 3 {
                    return Err(parse_err("RX requires qubit and angle".into()));
                }
                let angle: f64 = tokens[2]
                    .parse()
                    .map_err(|_| parse_err(format!("invalid angle '{}'", tokens[2])))?;
                (Gate::Rx(angle), 1)
            }
            "RY" => {
                if tokens.len() < 3 {
                    return Err(parse_err("RY requires qubit and angle".into()));
                }
                let angle: f64 = tokens[2]
                    .parse()
                    .map_err(|_| parse_err(format!("invalid angle '{}'", tokens[2])))?;
                (Gate::Ry(angle), 1)
            }
            "RZ" => {
                if tokens.len() < 3 {
                    return Err(parse_err("RZ requires qubit and angle".into()));
                }
                let angle: f64 = tokens[2]
                    .parse()
                    .map_err(|_| parse_err(format!("invalid angle '{}'", tokens[2])))?;
                (Gate::Rz(angle), 1)
            }
            other => return Err(parse_err(format!("unknown gate '{}'", other))),
        };

        // For rotation gates, qubit indices are only at position 1 (angle is at position 2)
        let qubit_tokens = if gate.is_parameterized() {
            &tokens[1..2]
        } else {
            &tokens[1..1 + expected_qubits]
        };

        if qubit_tokens.len() != expected_qubits {
            return Err(parse_err(format!(
                "{} requires {} qubit(s), got {}",
                gate.name(),
                expected_qubits,
                qubit_tokens.len()
            )));
        }

        let qubits: Vec<usize> = qubit_tokens
            .iter()
            .map(|t| {
                t.parse::<usize>()
                    .map_err(|_| parse_err(format!("invalid qubit index '{}'", t)))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let instr = Instruction::new(gate, qubits)?;
        circuit.add_instruction(instr)?;
    }

    Ok(circuit)
}
