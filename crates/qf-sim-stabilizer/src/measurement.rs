use crate::tableau::Tableau;

/// Check if the measurement outcome of `qubit` is deterministic.
///
/// Returns true iff no stabilizer row has x[row][qubit] = 1.
pub fn is_deterministic(tableau: &Tableau, qubit: usize) -> bool {
    let n = tableau.num_qubits();
    for row in n..2 * n {
        if tableau.x()[row][qubit] {
            return false;
        }
    }
    true
}
