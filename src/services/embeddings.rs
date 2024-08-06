use actix_web::{web, Result};

use crate::models::google_models::{AppState, Document, EmbeddingResponse};
use crate::services::google_cloud_authentication::GoogleCloudAuthentication;

async fn create_embedding(
    state: web::Data<AppState>,
    text: &str,
) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let url = format!("https://us-central1-aiplatform.googleapis.com/v1/projects/{}/locations/us-central1/publishers/google/models/textembedding-gecko:predict", state.project_id);

    let client = GoogleCloudAuthentication::get_authenticated_client()
        .await
        .unwrap();

    let response = client
        .post(&url)
        .json(&serde_json::json!({
            "instances": [{"content": text}]
        }))
        .send()
        .await?;

    let embedding_response: EmbeddingResponse = response.json().await?;
    Ok(embedding_response.predictions[0].embeddings.values.clone())
}

async fn store_embedding(
    state: web::Data<AppState>,
    doc: &Document,
    index: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let blob_name = format!("embedding_{}.json", index);
    let content = serde_json::to_vec(&doc)?;

    // do something
    Ok(())
}

async fn query_embeddings(
    state: web::Data<AppState>,
    query: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let query_embedding = create_embedding(state.clone(), query).await?;

    let embeddings: Vec<Vec<f32>> = Vec::new();
    let objects = state.storage_client.as_ref().bucket().list().await?;
    //do something
    Ok("Do something".to_string())
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot_product / (magnitude_a * magnitude_b)
}
