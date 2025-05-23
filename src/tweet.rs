use actix_web::web::{Json, Path};
use actix_web::{HttpResponse, delete, get, post};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::APPLICATION_JSON;
use crate::like::Like;
use crate::response::Response;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
}

impl Tweet {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
            likes: vec![],
        }
    }
}

pub type Tweets = Response<Tweet>;

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    pub message: Option<String>,
}

impl TweetRequest {
    pub fn to_tweet(&self) -> Option<Tweet> {
        match &self.message {
            Some(message) => Some(Tweet::new(message.to_string())),
            None => None,
        }
    }
}

#[get("/tweets")]
pub async fn get_all() -> HttpResponse {
    // TODO: fetch the last 50 tweets
    let tweets = Tweets { items: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweets)
}

#[post("/tweets")]
pub async fn create(tweet_request: Json<TweetRequest>) -> HttpResponse {
    println!("message: {}", tweet_request.message.as_ref().unwrap());
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(tweet_request.to_tweet())
}

#[get("/tweets/{id}")]
pub async fn get_by_id(_: Path<(String,)>) -> HttpResponse {
    // TODO: find tweet by ID
    let found_tweet: Option<Tweet> = None;

    match found_tweet {
        Some(tweet) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(tweet),
        None => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

#[delete("/tweets/{id}")]
pub async fn delete(_: Path<(String,)>) -> HttpResponse {
    // TODO: delete tweet by ID
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
