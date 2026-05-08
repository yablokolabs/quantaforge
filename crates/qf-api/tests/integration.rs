use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn json_request(method: &str, uri: &str, body: Option<serde_json::Value>) -> Request<Body> {
    let mut builder = Request::builder().method(method).uri(uri);
    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    let body = match body {
        Some(v) => Body::from(serde_json::to_vec(&v).unwrap()),
        None => Body::empty(),
    };
    builder.body(body).unwrap()
}

async fn body_json(response: axum::http::Response<Body>) -> serde_json::Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn test_health() {
    let app = qf_api::create_router();
    let response = app
        .oneshot(json_request("GET", "/health", None))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json = body_json(response).await;
    assert_eq!(json["status"], "ok");
    assert!(json["version"].is_string());
}

#[tokio::test]
async fn test_create_circuit() {
    let app = qf_api::create_router();
    let body = serde_json::json!({
        "num_qubits": 2,
        "gates": [
            {"gate": "H", "qubits": [0]},
            {"gate": "CNOT", "qubits": [0, 1]},
            {"gate": "MEASURE", "qubits": [0]},
            {"gate": "MEASURE", "qubits": [1]}
        ]
    });
    let response = app
        .oneshot(json_request("POST", "/circuits", Some(body)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json = body_json(response).await;
    assert!(json["id"].is_string());
    assert_eq!(json["num_qubits"], 2);
    assert_eq!(json["gate_count"], 4);
    assert!(json["circuit_json"].is_string());
}

#[tokio::test]
async fn test_get_circuit() {
    // First create a circuit, then retrieve it
    let app = qf_api::create_router();
    let create_body = serde_json::json!({
        "num_qubits": 1,
        "gates": [{"gate": "H", "qubits": [0]}]
    });
    let resp = app
        .clone()
        .oneshot(json_request("POST", "/circuits", Some(create_body)))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let created = body_json(resp).await;
    let id = created["id"].as_str().unwrap();

    let resp2 = app
        .oneshot(json_request("GET", &format!("/circuits/{id}"), None))
        .await
        .unwrap();
    assert_eq!(resp2.status(), StatusCode::OK);
    let fetched = body_json(resp2).await;
    assert_eq!(fetched["id"], id);
    assert_eq!(fetched["num_qubits"], 1);
}

#[tokio::test]
async fn test_get_circuit_not_found() {
    let app = qf_api::create_router();
    let resp = app
        .oneshot(json_request("GET", "/circuits/nonexistent-id", None))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    let json = body_json(resp).await;
    assert_eq!(json["error"], "Circuit not found");
}

#[tokio::test]
async fn test_submit_and_get_job() {
    let app = qf_api::create_router();

    // Create circuit with measurements
    let create_body = serde_json::json!({
        "num_qubits": 2,
        "gates": [
            {"gate": "H", "qubits": [0]},
            {"gate": "CNOT", "qubits": [0, 1]},
            {"gate": "MEASURE", "qubits": [0]},
            {"gate": "MEASURE", "qubits": [1]}
        ]
    });
    let resp = app
        .clone()
        .oneshot(json_request("POST", "/circuits", Some(create_body)))
        .await
        .unwrap();
    let circuit = body_json(resp).await;
    let circuit_id = circuit["id"].as_str().unwrap();

    // Submit job
    let job_body = serde_json::json!({
        "circuit_id": circuit_id,
        "shots": 100,
    });
    let resp = app
        .clone()
        .oneshot(json_request("POST", "/jobs", Some(job_body)))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let job = body_json(resp).await;
    assert_eq!(job["status"], "completed");
    let job_id = job["id"].as_str().unwrap();
    assert!(job["result"].is_object());
    assert_eq!(job["result"]["num_shots"], 100);
    assert_eq!(job["result"]["num_qubits"], 2);

    // Get job by id
    let resp = app
        .oneshot(json_request("GET", &format!("/jobs/{job_id}"), None))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let fetched_job = body_json(resp).await;
    assert_eq!(fetched_job["id"], job_id);
    assert_eq!(fetched_job["status"], "completed");
}

#[tokio::test]
async fn test_submit_job_circuit_not_found() {
    let app = qf_api::create_router();
    let job_body = serde_json::json!({
        "circuit_id": "nonexistent",
        "shots": 10,
    });
    let resp = app
        .oneshot(json_request("POST", "/jobs", Some(job_body)))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
