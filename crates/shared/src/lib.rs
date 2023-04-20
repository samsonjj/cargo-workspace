use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Comment {
    pub body: String,
    pub author: String,
}
