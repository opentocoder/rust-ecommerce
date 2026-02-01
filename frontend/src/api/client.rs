use gloo_net::http::{Request, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};
use shared::ApiError;

const API_BASE: &str = "http://localhost:3000/api";

fn add_auth_header(builder: RequestBuilder) -> RequestBuilder {
    if let Some(token) = super::get_token() {
        builder.header("Authorization", &format!("Bearer {}", token))
    } else {
        builder
    }
}

pub async fn get<T: DeserializeOwned>(endpoint: &str) -> Result<T, ApiError> {
    let url = format!("{}{}", API_BASE, endpoint);

    let request = add_auth_header(Request::get(&url));

    let response = request.send().await.map_err(|e| ApiError::internal_error(e.to_string()))?;

    if response.ok() {
        response
            .json()
            .await
            .map_err(|e| ApiError::internal_error(e.to_string()))
    } else {
        let error: ApiError = response
            .json()
            .await
            .unwrap_or_else(|_| ApiError::internal_error("Unknown error"));
        Err(error)
    }
}

pub async fn post<T: DeserializeOwned, B: Serialize>(endpoint: &str, body: &B) -> Result<T, ApiError> {
    let url = format!("{}{}", API_BASE, endpoint);

    let request = add_auth_header(
        Request::post(&url)
            .header("Content-Type", "application/json")
    )
    .json(body)
    .map_err(|e| ApiError::internal_error(e.to_string()))?;

    let response = request.send().await.map_err(|e| ApiError::internal_error(e.to_string()))?;

    if response.ok() {
        response
            .json()
            .await
            .map_err(|e| ApiError::internal_error(e.to_string()))
    } else {
        let error: ApiError = response
            .json()
            .await
            .unwrap_or_else(|_| ApiError::internal_error("Unknown error"));
        Err(error)
    }
}

pub async fn put<T: DeserializeOwned, B: Serialize>(endpoint: &str, body: &B) -> Result<T, ApiError> {
    let url = format!("{}{}", API_BASE, endpoint);

    let request = add_auth_header(
        Request::put(&url)
            .header("Content-Type", "application/json")
    )
    .json(body)
    .map_err(|e| ApiError::internal_error(e.to_string()))?;

    let response = request.send().await.map_err(|e| ApiError::internal_error(e.to_string()))?;

    if response.ok() {
        response
            .json()
            .await
            .map_err(|e| ApiError::internal_error(e.to_string()))
    } else {
        let error: ApiError = response
            .json()
            .await
            .unwrap_or_else(|_| ApiError::internal_error("Unknown error"));
        Err(error)
    }
}

pub async fn delete<T: DeserializeOwned>(endpoint: &str) -> Result<T, ApiError> {
    let url = format!("{}{}", API_BASE, endpoint);

    let request = add_auth_header(Request::delete(&url));

    let response = request.send().await.map_err(|e| ApiError::internal_error(e.to_string()))?;

    if response.ok() {
        response
            .json()
            .await
            .map_err(|e| ApiError::internal_error(e.to_string()))
    } else {
        let error: ApiError = response
            .json()
            .await
            .unwrap_or_else(|_| ApiError::internal_error("Unknown error"));
        Err(error)
    }
}
