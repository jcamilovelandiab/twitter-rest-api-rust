use actix_web::{HttpResponse, delete, get, post, web::Path};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{constants::APPLICATION_JSON, response::Response};

pub type Likes = Response<Like>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Like {
    id: String,
    created_at: DateTime<Utc>,
}

impl Like {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
        }
    }
}

#[get("/tweets/{id}/likes")]
pub async fn get_all_likes_by_tweet_id(_: Path<(String,)>) -> HttpResponse {
    // TODO: find tweet by ID
    let likes = Likes { items: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(likes)
}

#[post("/tweets/{id}/likes")]
pub async fn add_like(_: Path<(String,)>) -> HttpResponse {
    // TODO: Add one like to a tweet
    let like = Like::new();

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(like)
}

#[delete("/tweets/{id}/likes")]
pub async fn remove_like(_: Path<(String,)>) -> HttpResponse {
    // TODO: Remove one like from a tweet
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
