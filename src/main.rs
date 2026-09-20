use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
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
        .route("/api/test_key", post(test_key_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8992")
        .await
        .expect("Failed to bind TCP listener on port 8992");

    println!("\n=======================================================");
    println!("  ⚡ JEV SALES RADAR // LIVE TELEPROMPTER ACTIVE");
    println!("  🚀 Server running at: http://localhost:8992");
    println!("  🎯 Sales Psychology RAG: 10 Core Battlecards Loaded");
    println!("  ⏱️ Sub-25ms TypeSafe Jev System One Gateway Connected");
    println!("=======================================================\n");

    axum::serve(listener, app)
        .await
        .expect("Server failed to run");
}

async fn index_handler() -> impl IntoResponse {
    let html = include_str!("../static/index.html");
    Html(html)
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

#[derive(Deserialize)]
struct TestKeyRequest {
    api_key: String,
}

#[derive(Serialize)]
struct TestKeyResponse {
    status: String,
    online: bool,
    latency_ms: u64,
    model: String,
    message: String,
}

async fn test_key_handler(Json(req): Json<TestKeyRequest>) -> impl IntoResponse {
    let start = std::time::Instant::now();
    let url = std::env::var("TYPESAFE_BASE_URL")
        .unwrap_or_else(|_| "https://api.typesafe.ai/v1/systemone".to_string());

    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "model": "jev-latest",
        "state": "Sales Radar live connection ping.",
        "questions": {
            "health": {
                "type": "choice",
                "instructions": "Verify connectivity.",
                "criteria": {"ONLINE": "Operational"}
            }
        }
    });

    match client
        .post(&url)
        .header("Authorization", format!("Bearer {}", req.api_key.trim()))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => {
            let latency = start.elapsed().as_millis() as u64;
            (
                StatusCode::OK,
                Json(TestKeyResponse {
                    status: "success".to_string(),
                    online: true,
                    latency_ms: latency,
                    model: "jev-system-one".to_string(),
                    message: format!("Connected to TypeSafe Jev System One in {}ms!", latency),
                }),
            )
        }
        Ok(resp) => {
            let code = resp.status();
            (
                StatusCode::BAD_REQUEST,
                Json(TestKeyResponse {
                    status: "error".to_string(),
                    online: false,
                    latency_ms: start.elapsed().as_millis() as u64,
                    model: "none".to_string(),
                    message: format!("HTTP {}: Invalid credentials or endpoint.", code),
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(TestKeyResponse {
                status: "error".to_string(),
                online: false,
                latency_ms: start.elapsed().as_millis() as u64,
                model: "none".to_string(),
                message: format!("Connection error: {}", e),
            }),
        ),
    }
}
