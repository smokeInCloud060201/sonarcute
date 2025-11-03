pub mod server;
pub mod middleware;
pub mod handlers;

use actix_web::web;
use handlers::{reviews, ai_models, integrations, templates, health};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check
        .route("/health", web::get().to(health::health_check))
        
        // Review endpoints
        .service(
            web::scope("/api/reviews")
                .route("/local", web::post().to(reviews::create_local_review))
                .route("/pr", web::post().to(reviews::create_pr_review))
                .route("", web::get().to(reviews::list_reviews))
                .route("/{id}", web::get().to(reviews::get_review))
                .route("/{id}", web::delete().to(reviews::delete_review))
                .route("/{id}/comments", web::get().to(reviews::get_review_comments))
                .route("/{id}/retry", web::post().to(reviews::retry_review))
        )
        
        // AI Model endpoints
        .service(
            web::scope("/api/ai-models")
                .route("", web::get().to(ai_models::list_models))
                .route("", web::post().to(ai_models::create_model))
                .route("/{id}", web::get().to(ai_models::get_model))
                .route("/{id}", web::put().to(ai_models::update_model))
                .route("/{id}", web::delete().to(ai_models::delete_model))
                .route("/{id}/test", web::post().to(ai_models::test_model))
                .route("/{id}/default", web::post().to(ai_models::set_default_model))
        )
        
        // Integration endpoints
        .service(
            web::scope("/api/integrations")
                .route("/github", web::get().to(integrations::get_github_config))
                .route("/github", web::post().to(integrations::configure_github))
                .route("/jira", web::get().to(integrations::get_jira_config))
                .route("/jira", web::post().to(integrations::configure_jira))
                .route("/{type}/test", web::post().to(integrations::test_integration))
        )
        
        // Template endpoints
        .service(
            web::scope("/api/templates")
                .route("", web::get().to(templates::list_templates))
                .route("", web::post().to(templates::create_template))
                .route("/{id}", web::get().to(templates::get_template))
                .route("/{id}", web::put().to(templates::update_template))
                .route("/{id}", web::delete().to(templates::delete_template))
        );
}

