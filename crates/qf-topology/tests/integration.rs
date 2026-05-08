use qf_topology::{
    adjacency_matrix, is_connected, max_degree, route_gate, square_lattice, validate_two_qubit_gate,
};

#[test]
fn square_5x5_has_25_nodes() {
    let g = square_lattice(5, 5).unwrap();
    assert_eq!(g.num_qubits(), 25);
}

#[test]
fn square_5x5_interior_degree_4() {
    let g = square_lattice(5, 5).unwrap();
    // Interior nodes: row 1..4, col 1..4
    for r in 1..4 {
        for c in 1..4 {
            let id = r * 5 + c;
            assert_eq!(
                g.degree(id).unwrap(),
                4,
                "interior qubit {id} should have degree 4"
            );
        }
    }
}

#[test]
fn square_5x5_corner_degree_2() {
    let g = square_lattice(5, 5).unwrap();
    let corners = [0, 4, 20, 24];
    for &id in &corners {
        assert_eq!(
            g.degree(id).unwrap(),
            2,
            "corner qubit {id} should have degree 2"
        );
    }
}

#[test]
fn square_5x5_edge_degree_3() {
    let g = square_lattice(5, 5).unwrap();
    // Top edge non-corner: cols 1..4
    for c in 1..4 {
        assert_eq!(
            g.degree(c).unwrap(),
            3,
            "top-edge qubit {c} should have degree 3"
        );
    }
    // Left edge non-corner: rows 1..4
    for r in 1..4 {
        let id = r * 5;
        assert_eq!(
            g.degree(id).unwrap(),
            3,
            "left-edge qubit {id} should have degree 3"
        );
    }
}

#[test]
fn square_5x5_are_connected_adjacent() {
    let g = square_lattice(5, 5).unwrap();
    assert!(g.are_connected(0, 1), "0 and 1 should be connected");
}

#[test]
fn square_5x5_are_not_connected_non_adjacent() {
    let g = square_lattice(5, 5).unwrap();
    assert!(!g.are_connected(0, 2), "0 and 2 should NOT be connected");
}

#[test]
fn square_5x5_shortest_path() {
    let g = square_lattice(5, 5).unwrap();
    let path = g.shortest_path(0, 24).unwrap();
    // Must start at 0 and end at 24
    assert_eq!(*path.first().unwrap(), 0);
    assert_eq!(*path.last().unwrap(), 24);
    // Manhattan distance is 8, so path length = 9
    assert_eq!(path.len(), 9);
    // Every consecutive pair must be connected
    for w in path.windows(2) {
        assert!(g.are_connected(w[0], w[1]));
    }
}

#[test]
fn validate_gate_connected() {
    let g = square_lattice(5, 5).unwrap();
    assert!(validate_two_qubit_gate(&g, 0, 1).is_ok());
}

#[test]
fn validate_gate_disconnected() {
    let g = square_lattice(5, 5).unwrap();
    assert!(validate_two_qubit_gate(&g, 0, 2).is_err());
}

#[test]
fn square_lattice_is_connected() {
    let g = square_lattice(5, 5).unwrap();
    assert!(is_connected(&g));
}

#[test]
fn adjacency_matrix_dimensions() {
    let g = square_lattice(5, 5).unwrap();
    let mat = adjacency_matrix(&g);
    assert_eq!(mat.len(), 25);
    for row in &mat {
        assert_eq!(row.len(), 25);
    }
}

#[test]
fn adjacency_matrix_symmetric() {
    let g = square_lattice(3, 3).unwrap();
    let mat = adjacency_matrix(&g);
    for i in 0..mat.len() {
        for j in 0..mat.len() {
            assert_eq!(mat[i][j], mat[j][i]);
        }
    }
}

#[test]
fn max_degree_square_5x5() {
    let g = square_lattice(5, 5).unwrap();
    assert_eq!(max_degree(&g), 4);
}

#[test]
fn route_gate_adjacent_no_swaps() {
    let g = square_lattice(3, 3).unwrap();
    // Identity mapping: logical i -> physical i
    let mapping: Vec<usize> = (0..9).collect();
    let route = route_gate(&g, 0, 1, &mapping).unwrap();
    assert!(route.swaps.is_empty());
}

#[test]
fn route_gate_non_adjacent_produces_swaps() {
    let g = square_lattice(3, 3).unwrap();
    let mapping: Vec<usize> = (0..9).collect();
    // 0 and 2 are not adjacent in a 3×3 grid
    let route = route_gate(&g, 0, 2, &mapping).unwrap();
    assert!(!route.swaps.is_empty());
    // After routing, the logical qubits should be on adjacent physical qubits
    let p1 = route.final_mapping[0];
    let p2 = route.final_mapping[2];
    assert!(g.are_connected(p1, p2));
}

#[test]
fn invalid_dimensions() {
    assert!(square_lattice(0, 5).is_err());
    assert!(square_lattice(5, 0).is_err());
}

#[test]
fn heavy_hex_builds() {
    let g = qf_topology::heavy_hex_lattice(3, 3).unwrap();
    assert!(g.num_qubits() > 9); // has bridge qubits
    assert!(is_connected(&g));
}
