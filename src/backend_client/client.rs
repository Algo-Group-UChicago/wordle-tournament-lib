use pyo3::PyErr;
use reqwest::blocking::Client;

use crate::common::{API_END_PATH, API_GUESSES_PATH, API_START_PATH, BACKEND_API};
use crate::hint::WordleHint;

use super::types::{EndRequest, EndResponse, GuessRequest, GuessResponse, StartRequest, StartResponse};

/// Send start signal to server to start tournament evaluation
pub fn send_start(team_id: &str) -> Result<String, PyErr> {
    let client = Client::new();

    let request_body = StartRequest {
        team_id: team_id.to_string(),
    };

    let endpoint = format!("{}{}", BACKEND_API, API_START_PATH);
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

    let start_response: StartResponse = response.json().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to parse server response: {}",
            e
        ))
    })?;

    Ok(start_response.run_id)
}

/// Send a round of guesses to server and return the corresponding hints based on answer key
pub fn send_guesses(team_id: &str, run_id: &str, guesses: &[String]) -> Result<Vec<WordleHint>, PyErr> {
    let client = Client::new();

    let request_body = GuessRequest {
        team_id: team_id.to_string(),
        run_id: run_id.to_string(),
        guesses: guesses.to_vec(),
    };

    let endpoint = format!("{}{}", BACKEND_API, API_GUESSES_PATH);
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
    let client = Client::new();

    let request_body = EndRequest {
        team_id: team_id.to_string(),
        run_id: run_id.to_string(),
    };

    let endpoint = format!("{}{}", BACKEND_API, API_END_PATH);
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

    let end_response: EndResponse = response.json().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to parse server response: {}",
            e
        ))
    })?;

    Ok(end_response.score)
}
