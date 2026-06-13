use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

#[derive(Debug, Deserialize)]
struct FilterParams {
    grup: Option<String>,
    tag: Option<String>,
}

fn url_encode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            _ => {
                let mut buf = [0u8; 4];
                c.encode_utf8(&mut buf)
                    .bytes()
                    .map(|b| format!("%{:02X}", b))
                    .collect::<String>()
            }
        })
        .collect()
}

/// GET /api/doa?grup=...&tag=...
async fn list_doa(Query(params): Query<FilterParams>) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut url = "https://equran.id/api/doa".to_string();
    let mut parts = vec![];

    if let Some(grup) = &params.grup {
        parts.push(format!("grup={}", url_encode(grup)));
    }
    if let Some(tag) = &params.tag {
        parts.push(format!("tag={}", url_encode(tag)));
    }
    if !parts.is_empty() {
        url.push('?');
        url.push_str(&parts.join("&"));
    }

    match reqwest::get(&url).await {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(data) => Ok(Json(data)),
            Err(_) => Err(StatusCode::BAD_GATEWAY),
        },
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}

/// GET /api/doa/:id
async fn get_doa(Path(id): Path<u32>) -> Result<Json<serde_json::Value>, StatusCode> {
    let url = format!("https://equran.id/api/doa/{}", id);

    match reqwest::get(&url).await {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(data) => Ok(Json(data)),
            Err(_) => Err(StatusCode::BAD_GATEWAY),
        },
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}

/// GET /api/health
async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "buku-doa-backend"
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/doa", get(list_doa))
        .route("/api/doa/{id}", get(get_doa))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Backend running on http://localhost:3001");
    axum::serve(listener, app).await.unwrap();
}
