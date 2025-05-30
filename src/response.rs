use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub items: Vec<T>,
}
