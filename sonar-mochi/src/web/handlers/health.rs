use actix_web::{HttpResponse, Result};

pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "sonar-mochi",
        "version": env!("CARGO_PKG_VERSION")
    })))
}

