use crate::error::TopologyError;
use crate::graph::QubitGraph;

/// Build a square lattice of `rows × cols` qubits with nearest-neighbor connectivity.
/// Qubit id = row * cols + col.
pub fn square_lattice(rows: usize, cols: usize) -> Result<QubitGraph, TopologyError> {
    if rows == 0 || cols == 0 {
        return Err(TopologyError::InvalidDimensions { rows, cols });
    }

    let mut g = QubitGraph::new();

    for r in 0..rows {
        for c in 0..cols {
            let id = r * cols + c;
            g.add_qubit(id, r, c);
        }
    }

    for r in 0..rows {
        for c in 0..cols {
            let id = r * cols + c;
            // East neighbor
            if c + 1 < cols {
                g.add_connection(id, id + 1)?;
            }
            // South neighbor
            if r + 1 < rows {
                g.add_connection(id, id + cols)?;
            }
        }
    }

    Ok(g)
}

/// Build a simplified heavy-hex lattice.
///
/// Heavy-hex lattices are used in IBM quantum processors. We model them as:
/// - Data qubits arranged on a hex-like grid
/// - Bridge (syndrome) qubits inserted on every other horizontal edge
///
/// For simplicity we lay out data qubits on a `rows × cols` grid and add
/// bridge qubits between horizontally adjacent pairs on alternating rows.
pub fn heavy_hex_lattice(rows: usize, cols: usize) -> Result<QubitGraph, TopologyError> {
    if rows == 0 || cols == 0 {
        return Err(TopologyError::InvalidDimensions { rows, cols });
    }

    let mut g = QubitGraph::new();

    // Phase 1: add data qubits (ids 0 .. rows*cols - 1)
    for r in 0..rows {
        for c in 0..cols {
            let id = r * cols + c;
            g.add_qubit(id, r, c);
        }
    }

    let mut bridge_id = rows * cols;

    // Phase 2: horizontal connections with bridge qubits on even rows
    for r in 0..rows {
        for c in 0..cols.saturating_sub(1) {
            let left = r * cols + c;
            let right = r * cols + c + 1;
            if r % 2 == 0 {
                // Insert a bridge qubit between left and right
                g.add_qubit(bridge_id, r, c * 2 + 1); // synthetic coordinate
                g.add_connection(left, bridge_id)?;
                g.add_connection(bridge_id, right)?;
                bridge_id += 1;
            } else {
                // Direct connection on odd rows
                g.add_connection(left, right)?;
            }
        }
    }

    // Phase 3: vertical connections (only on alternating columns for hex-like pattern)
    for r in 0..rows.saturating_sub(1) {
        for c in 0..cols {
            if (r + c) % 2 == 0 {
                let top = r * cols + c;
                let bottom = (r + 1) * cols + c;
                g.add_connection(top, bottom)?;
            }
        }
    }

    Ok(g)
}
