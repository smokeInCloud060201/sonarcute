pub mod review;
pub mod review_comment;
pub mod ai_model;
pub mod integration;
pub mod review_template;
pub mod review_history;

pub use review::Entity as Review;
pub use review_comment::Entity as ReviewComment;
pub use ai_model::Entity as AiModelConfig;
pub use integration::Entity as IntegrationConfig;
pub use review_template::Entity as ReviewTemplate;
pub use review_history::Entity as ReviewHistory;
