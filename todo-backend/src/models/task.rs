use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Task {
    #[serde(default)]
    pub id: i64,
    pub description: String,
    pub done: bool,
}

#[derive(Deserialize, Serialize)]
pub struct NewTask {
    pub description: String,
    #[serde(default)]
    pub done: bool,
}
