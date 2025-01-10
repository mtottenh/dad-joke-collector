use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DadJoke {
    pub id: String,
    pub joke: String,
    pub status: i32,
}
