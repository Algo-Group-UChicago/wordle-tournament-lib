mod types;
mod client;

// Re-export public functions from client
pub use client::{send_end, send_guesses, send_start};

// Optionally re-export types if needed elsewhere
pub use types::{EndRequest, EndResponse, GuessRequest, GuessResponse, StartRequest, StartResponse};
