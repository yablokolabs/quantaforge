#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = qf_api::create_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("QuantaForge API listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
