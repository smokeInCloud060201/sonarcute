mod config;
mod web;
mod services;
mod ai;
mod integrations;
mod database;
mod models;
mod utils;

use actix_web::{App, HttpServer, middleware::Logger};
use tracing_subscriber;
use dotenvy::dotenv;

use config::AppConfig;
use web::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("actix_web=info".parse().unwrap())
                .add_directive("sonar_mochi=debug".parse().unwrap()),
        )
        .init();

    // Load configuration
    let config = AppConfig::from_env().expect("Failed to load configuration");

    tracing::info!(
        "Starting Sonar-Mochi server on {}:{}",
        config.server.host,
        config.server.port
    );

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(configure_routes)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

