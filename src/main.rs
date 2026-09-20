use axum::{
    extract::State,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod engine;
mod rag;

use engine::{AnalysisRequest, JevEngine};
use rag::BATTLECARDS;

struct AppState {
    engine: JevEngine,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        engine: JevEngine::new(),
    });

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/api/analyze", post(analyze_handler))
        .route("/api/playbooks", get(playbooks_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8992")
        .await
        .expect("Failed to bind TCP listener on port 8992");

    println!("\n=======================================================");
    println!("  ⚡ JEV SALES RADAR // ELEVENLABS SLEEK TELEPROMPTER");
    println!("  🚀 Server running at: http://localhost:8992");
    println!("  🎯 Sales Psychology RAG: 12 General Battlecards Active");
    println!("  ⏱️ Sub-Millisecond SIMD In-Memory Decision Engine");
    println!("=======================================================\n");

    axum::serve(listener, app)
        .await
        .expect("Server failed to run");
}

async fn index_handler() -> impl IntoResponse {
    match tokio::fs::read_to_string("static/index.html").await {
        Ok(html) => Html(html),
        Err(_) => Html(include_str!("../static/index.html").to_string()),
    }
}

async fn analyze_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AnalysisRequest>,
) -> impl IntoResponse {
    let hint = state.engine.analyze(&req).await;
    Json(hint)
}

async fn playbooks_handler() -> impl IntoResponse {
    Json(BATTLECARDS)
}
