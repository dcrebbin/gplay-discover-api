use cloud_storage::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::google_cloud_authentication::GoogleCloudAuthentication;

#[derive(Deserialize)]
pub struct Query {
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub text: String,
    pub embedding: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbeddingResponse {
    pub predictions: Vec<EmbeddingPrediction>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbeddingPrediction {
    pub embeddings: Embeddings,
}

#[derive(Serialize, Deserialize)]
pub struct Embeddings {
    pub values: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct GeminiRequest {
    pub instances: Vec<GeminiInstance>,
}

#[derive(Serialize, Deserialize)]
pub struct GeminiInstance {
    pub context: String,
    pub examples: Vec<String>,
    pub messages: Vec<GeminiMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct GeminiMessage {
    pub author: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct GeminiResponse {
    pub predictions: Vec<GeminiPrediction>,
}

#[derive(Serialize, Deserialize)]
pub struct GeminiPrediction {
    pub candidates: Vec<GeminiCandidate>,
}

#[derive(Serialize, Deserialize)]
pub struct GeminiCandidate {
    pub content: String,
}

pub struct AppState {
    pub storage_client: Arc<Client>,
    pub http_client: reqwest::Client,
    pub project_id: String,
    pub bucket_name: String,
}
