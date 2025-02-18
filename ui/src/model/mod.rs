use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ArticleCard {
    pub url: String,
    pub title: String,
    pub description: String,
    pub image: String,
    pub comments: i32,
    pub created_at: DateTime<Utc>,
}

impl ArticleCard {
    pub fn new(url: &str, title: String, description: String, image: String) -> Self {
        Self {
            url: url.to_string(),
            title,
            description,
            image,
            comments: 0,
            created_at: Utc::now(),
        }
    }
}
