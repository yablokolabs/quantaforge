use rand::Rng;

#[derive(Debug, Clone)]
pub struct Tableau {
    n: usize,
    x: Vec<Vec<bool>>,
    z: Vec<Vec<bool>>,
    r: Vec<bool>,
}

impl Tableau {
    /// Create a new tableau for the |0...0⟩ state of `n` qubits.
    pub fn new(n: usize) -> Self {
        let rows = 2 * n;
        let mut x = vec![vec![false; n]; rows];
        let mut z = vec![vec![false; n]; rows];
        let r = vec![false; rows];

        // Destabilizer i (row i): X on qubit i
        for (i, row) in x.iter_mut().enumerate().take(n) {
            row[i] = true;
        }
        // Stabilizer i (row n+i): Z on qubit i
        for (i, row) in z.iter_mut().enumerate().skip(n) {
            row[i - n] = true;
        }

        Self { n, x, z, r }
    }

    pub fn num_qubits(&self) -> usize {
        self.n
    }

    /// Access X bits (for measurement module).
    pub(crate) fn x(&self) -> &Vec<Vec<bool>> {
        &self.x
    }

    pub fn hadamard(&mut self, q: usize) {
        for i in 0..2 * self.n {
            self.r[i] ^= self.x[i][q] & self.z[i][q];
            std::mem::swap(&mut self.x[i][q], &mut self.z[i][q]);
        }
    }

    pub fn phase_gate(&mut self, q: usize) {
        for i in 0..2 * self.n {
            self.r[i] ^= self.x[i][q] & self.z[i][q];
            self.z[i][q] ^= self.x[i][q];
        }
    }

    pub fn cnot(&mut self, control: usize, target: usize) {
        for i in 0..2 * self.n {
            self.r[i] ^= self.x[i][control]
                & self.z[i][target]
                & (self.x[i][target] ^ self.z[i][control] ^ true);
            self.x[i][target] ^= self.x[i][control];
            self.z[i][control] ^= self.z[i][target];
        }
    }

    pub fn pauli_x(&mut self, q: usize) {
        for i in 0..2 * self.n {
            self.r[i] ^= self.z[i][q];
        }
    }

    pub fn pauli_y(&mut self, q: usize) {
        for i in 0..2 * self.n {
            self.r[i] ^= self.x[i][q] ^ self.z[i][q];
        }
    }

    pub fn pauli_z(&mut self, q: usize) {
        for i in 0..2 * self.n {
            self.r[i] ^= self.x[i][q];
        }
    }

    pub fn cz(&mut self, q1: usize, q2: usize) {
        self.hadamard(q2);
        self.cnot(q1, q2);
        self.hadamard(q2);
    }

    pub fn measure(&mut self, qubit: usize, rng: &mut impl Rng) -> bool {
        let n = self.n;

        // Find a stabilizer row p (in n..2n-1) with x[p][qubit] = 1
        let p = (n..2 * n).find(|&row| self.x[row][qubit]);

        if let Some(p) = p {
            // Random outcome
            // For all rows i ≠ p where x[i][qubit] = 1, rowmult(i, p)
            for i in 0..2 * n {
                if i != p && self.x[i][qubit] {
                    rowmult(self, i, p);
                }
            }
            // Set destabilizer[p-n] = stabilizer[p]
            let dest = p - n;
            self.x[dest] = self.x[p].clone();
            self.z[dest] = self.z[p].clone();
            self.r[dest] = self.r[p];

            // Set stabilizer[p] to all zeros except z[p][qubit] = 1
            self.x[p] = vec![false; n];
            self.z[p] = vec![false; n];
            self.z[p][qubit] = true;
            self.r[p] = rng.gen_bool(0.5);

            self.r[p]
        } else {
            // Deterministic outcome — use scratch row
            let mut scratch_x = vec![false; n];
            let mut scratch_z = vec![false; n];
            let mut scratch_r = false;

            // For each destabilizer row i (0..n-1) where x[i][qubit] = 1,
            // multiply scratch by stabilizer row (i+n)
            for i in 0..n {
                if self.x[i][qubit] {
                    // Multiply scratch by row i+n
                    let src = i + n;
                    let phase = rowmult_phase(
                        &self.x[src],
                        &self.z[src],
                        self.r[src],
                        &scratch_x,
                        &scratch_z,
                        scratch_r,
                    );
                    for j in 0..n {
                        scratch_x[j] ^= self.x[src][j];
                        scratch_z[j] ^= self.z[src][j];
                    }
                    scratch_r = phase;
                }
            }

            scratch_r
        }
    }
}

/// g function: phase correction when multiplying two single-qubit Paulis.
fn g(x1: bool, z1: bool, x2: bool, z2: bool) -> i32 {
    match (x1, z1) {
        (false, false) => 0, // I
        (true, true) => {
            // Y
            z2 as i32 - x2 as i32
        }
        (true, false) => {
            // X
            z2 as i32 * (2 * x2 as i32 - 1)
        }
        (false, true) => {
            // Z
            x2 as i32 * (1 - 2 * z2 as i32)
        }
    }
}

/// Compute the resulting phase bit after multiplying source row into target row.
fn rowmult_phase(
    src_x: &[bool],
    src_z: &[bool],
    src_r: bool,
    tgt_x: &[bool],
    tgt_z: &[bool],
    tgt_r: bool,
) -> bool {
    let n = src_x.len();
    let mut phase: i32 = 2 * tgt_r as i32 + 2 * src_r as i32;
    for j in 0..n {
        phase += g(src_x[j], src_z[j], tgt_x[j], tgt_z[j]);
    }
    // r = (phase % 4 == 2)
    phase.rem_euclid(4) == 2
}

/// Multiply Pauli at row `target` by Pauli at row `source` (in-place on target).
fn rowmult(tableau: &mut Tableau, target: usize, source: usize) {
    let n = tableau.n;
    let new_r = rowmult_phase(
        &tableau.x[source],
        &tableau.z[source],
        tableau.r[source],
        &tableau.x[target],
        &tableau.z[target],
        tableau.r[target],
    );
    for j in 0..n {
        tableau.x[target][j] ^= tableau.x[source][j];
        tableau.z[target][j] ^= tableau.z[source][j];
    }
    tableau.r[target] = new_r;
}
