use pyo3::PyErr;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::common::{API_GUESSES_ENDPOINT, DUMMY_GUESS, WORD_LENGTH};
use crate::hint::WordleHint;

#[derive(Serialize)]
struct GuessRequest {
    team_id: String,
    guesses: Vec<String>,
}

#[derive(Deserialize)]
struct GuessResponse {
    hints: Vec<String>,
}

/// Submit a round of guesses to server and return the corresponding hints based on answer key
pub fn submit_guesses(team_id: &str, guesses: &[String]) -> Result<Vec<WordleHint>, PyErr> {
    let client = Client::new();

    let request_body = GuessRequest {
        team_id: team_id.to_string(),
        guesses: guesses.to_vec(),
    };

    let response = client
        .post(API_GUESSES_ENDPOINT)
        .json(&request_body)
        .send()
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to send request to server: {}",
                e
            ))
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response
            .text()
            .unwrap_or_else(|_| "Unable to read error message".to_string());
        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Server returned error status {}: {}",
            status, error_body
        )));
    }

    let guess_response: GuessResponse = response.json().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to parse server response: {}",
            e
        ))
    })?;

    guesses
        .iter()
        .zip(guess_response.hints.iter())
        .map(|(word, hint_str)| {
            if word == DUMMY_GUESS {
                Ok(WordleHint::new_all_correct("-".repeat(WORD_LENGTH)))
            } else {
                WordleHint::new_hint(word.clone(), hint_str.clone())
            }
        })
        .collect::<Result<Vec<WordleHint>, PyErr>>()
}

/// Send start signal to server to start tournament evaluation
pub fn send_start_signal(team_id: &str) -> Result<(), PyErr> {
    // This will probably start some kind of timer
    println!("Sending mock start signal to server for team {}", team_id);
    // TODO: Implement actual API call
    Ok(())
}

/// Send end signal to server to end tournament evaluation and return score
pub fn send_end_signal(team_id: &str) -> Result<f64, PyErr> {
    // This will probably end some kind of timer, record the user's final score,
    // shuffle the user's answer key for the next run etc.
    println!("Sending mock end signal to server for team {}", team_id);
    // TODO: Implement actual API call
    // Should return the weighted server score
    Ok(0.0)
}
