use pyo3::PyErr;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::common::{API_END_PATH, API_GUESSES_PATH, API_START_PATH, BACKEND_API};
use crate::hint::WordleHint;

use super::types::{
    EndRequest, EndResponse, GuessRequest, GuessResponse, StartRequest, StartResponse,
};

/// Generic helper to make POST requests to the backend API
fn post_request<Req: Serialize, Resp: for<'de> Deserialize<'de>>(
    endpoint_path: &str,
    request_body: Req,
) -> Result<Resp, PyErr> {
    let client = Client::new();
    let endpoint = format!("{}{}", BACKEND_API, endpoint_path);

    let response = client
        .post(&endpoint)
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

    response.json().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to parse server response: {}",
            e
        ))
    })
}

/// Send start signal to server to start tournament evaluation
pub fn send_start(team_id: &str) -> Result<String, PyErr> {
    let response: StartResponse = post_request(
        API_START_PATH,
        StartRequest {
            team_id: team_id.to_string(),
        },
    )?;
    Ok(response.run_id)
}

/// Send a round of guesses to server and return the corresponding hints based on answer key
pub fn send_guesses(
    team_id: &str,
    run_id: &str,
    guesses: &[String],
) -> Result<Vec<WordleHint>, PyErr> {
    let response: GuessResponse = post_request(
        API_GUESSES_PATH,
        GuessRequest {
            team_id: team_id.to_string(),
            run_id: run_id.to_string(),
            guesses: guesses.to_vec(),
        },
    )?;

    guesses
        .iter()
        .zip(response.hints.iter())
        .map(|(word, hint_str)| {
            if hint_str == "OOOOO" {
                Ok(WordleHint::new_all_correct(word.clone()))
            } else {
                WordleHint::new_hint(word.clone(), hint_str.clone())
            }
        })
        .collect::<Result<Vec<WordleHint>, PyErr>>()
}

/// Send end signal to server to end tournament evaluation and return score
pub fn send_end(team_id: &str, run_id: &str) -> Result<f64, PyErr> {
    let response: EndResponse = post_request(
        API_END_PATH,
        EndRequest {
            team_id: team_id.to_string(),
            run_id: run_id.to_string(),
        },
    )?;
    Ok(response.score)
}
