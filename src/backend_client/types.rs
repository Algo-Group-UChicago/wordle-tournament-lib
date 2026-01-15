use serde::{Deserialize, Serialize};

// Request and response types for communicating with the backend API.
// These types also serve as this library's understanding of the backend interface.

#[derive(Serialize)]
pub struct StartRequest {
    pub team_id: String,
}

#[derive(Deserialize)]
pub struct StartResponse {
    pub run_id: String,
}

#[derive(Serialize)]
pub struct GuessRequest {
    pub team_id: String,
    pub run_id: String,
    pub guesses: Vec<String>,
}

#[derive(Deserialize)]
pub struct GuessResponse {
    pub hints: Vec<String>,
}

#[derive(Serialize)]
pub struct EndRequest {
    pub team_id: String,
    pub run_id: String,
}

#[derive(Deserialize)]
pub struct EndResponse {
    pub score: f64,
    pub average_guesses: f64,
    pub solved: bool,
}
