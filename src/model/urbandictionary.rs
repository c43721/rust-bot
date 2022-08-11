use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchStringResponse {
    pub list: Vec<Definition>,
}

#[derive(Serialize, Deserialize)]
pub struct Definition {
    pub definition: String,
    pub permalink: String,
    pub thumbs_up: u32,
    pub author: String,
    pub word: String,
    pub defid: u32,
    pub example: String,
    pub thumbs_down: u32,
}
