use qf_qec_ldpc::{
    gf2_rank, hamming_code, repetition_code, BitFlipDecoder, CssCode, Decoder, LdpcError,
    ParityCheckMatrix,
};

#[test]
fn repetition_code_dimensions() {
    let n = 5;
    let h = repetition_code(n);
    assert_eq!(h.rows(), n - 1);
    assert_eq!(h.cols(), n);
}

#[test]
fn repetition_code_single_bit_error_nonzero_syndrome() {
    let n = 5;
    let h = repetition_code(n);
    let mut error = vec![0u8; n];
    error[2] = 1;
    let syn = h.syndrome(&error).unwrap();
    assert!(syn.iter().any(|&s| s != 0));
}

#[test]
fn repetition_code_no_error_zero_syndrome() {
    let n = 5;
    let h = repetition_code(n);
    let error = vec![0u8; n];
    let syn = h.syndrome(&error).unwrap();
    assert!(syn.iter().all(|&s| s == 0));
}

#[test]
fn hamming_code_7_4_dimensions() {
    let h = hamming_code(3);
    assert_eq!(h.rows(), 3);
    assert_eq!(h.cols(), 7);
}

#[test]
fn bit_flip_decoder_corrects_single_error_repetition() {
    let n = 7;
    let h = repetition_code(n);
    let decoder = BitFlipDecoder::new(50);

    // Single-bit error at position 3
    let mut error = vec![0u8; n];
    error[3] = 1;
    let syn = h.syndrome(&error).unwrap();

    let correction = decoder.decode(&h, &syn).unwrap();
    // After applying correction, syndrome should be zero
    let corrected_syn = h.syndrome(&correction).unwrap();
    assert_eq!(corrected_syn, syn);
}

#[test]
fn css_orthogonality_valid() {
    // Construct a simple CSS code: use repetition code Hx and its dual for Hz
    // For a valid CSS code, we need Hx * Hz^T = 0 (mod 2)
    // Simplest example: Hx = Hz = repetition code won't work since H*H^T != 0 in general.
    // Use identity-like construction: Hx = [[1,1,0,0],[0,0,1,1]], Hz = [[1,0,1,0],[0,1,0,1]]
    // Check: Hx * Hz^T:
    //   [1,1,0,0] . [1,0] = 1, [1,1,0,0] . [0,1] = 1  => [1,1]  -- not zero
    // Instead: Hx = [[1,1,1,1]], Hz = [[1,1,0,0],[0,0,1,1]]
    // Hx * Hz^T = [1,1,1,1] * [[1,0],[1,0],[0,1],[0,1]] = [2, 2] mod 2 = [0,0] ✓
    let hx = ParityCheckMatrix::from_dense(vec![vec![1, 1, 1, 1]]).unwrap();
    let hz = ParityCheckMatrix::from_dense(vec![vec![1, 1, 0, 0], vec![0, 0, 1, 1]]).unwrap();
    let css = CssCode::new(hx, hz, "test-css");
    assert!(css.is_ok());
}

#[test]
fn css_orthogonality_violated() {
    let hx = ParityCheckMatrix::from_dense(vec![vec![1, 1, 0], vec![0, 1, 1]]).unwrap();
    let hz = ParityCheckMatrix::from_dense(vec![vec![1, 0, 1]]).unwrap();
    // Hx * Hz^T = [[1,1,0]*[1,0,1]^T, [0,1,1]*[1,0,1]^T] = [1, 1] mod 2 -- not zero
    let css = CssCode::new(hx, hz, "bad-css");
    assert!(matches!(css, Err(LdpcError::CssOrthogonalityViolated)));
}

#[test]
fn css_num_physical_qubits() {
    let hx = ParityCheckMatrix::from_dense(vec![vec![1, 1, 1, 1]]).unwrap();
    let hz = ParityCheckMatrix::from_dense(vec![vec![1, 1, 0, 0], vec![0, 0, 1, 1]]).unwrap();
    let css = CssCode::new(hx, hz, "test-css").unwrap();
    assert_eq!(css.num_physical_qubits(), 4);
}

#[test]
fn gf2_rank_identity() {
    let n = 4;
    let mut data = vec![vec![0u8; n]; n];
    for i in 0..n {
        data[i][i] = 1;
    }
    let m = ParityCheckMatrix::from_dense(data).unwrap();
    assert_eq!(gf2_rank(&m), n);
}

#[test]
fn parity_check_row_col_weights() {
    let h =
        ParityCheckMatrix::from_dense(vec![vec![1, 1, 0, 0], vec![0, 1, 1, 0], vec![0, 0, 1, 1]])
            .unwrap();
    assert_eq!(h.row_weight(0), 2);
    assert_eq!(h.row_weight(1), 2);
    assert_eq!(h.row_weight(2), 2);
    assert_eq!(h.col_weight(0), 1);
    assert_eq!(h.col_weight(1), 2);
    assert_eq!(h.col_weight(2), 2);
    assert_eq!(h.col_weight(3), 1);
}

#[test]
fn is_ldpc_check() {
    let h =
        ParityCheckMatrix::from_dense(vec![vec![1, 1, 0, 0], vec![0, 1, 1, 0], vec![0, 0, 1, 1]])
            .unwrap();
    assert!(h.is_ldpc(2));
    assert!(!h.is_ldpc(1));
}

#[test]
fn parity_check_serialization_roundtrip() {
    let h = repetition_code(5);
    let json = serde_json::to_string(&h).unwrap();
    let h2: ParityCheckMatrix = serde_json::from_str(&json).unwrap();
    assert_eq!(h, h2);
}

#[test]
fn invalid_dimensions_zero_rows() {
    let result = ParityCheckMatrix::new(0, 5);
    assert!(matches!(
        result,
        Err(LdpcError::InvalidDimensions { rows: 0, cols: 5 })
    ));
}
