use crate::models::google_models::{
    AppState, GeminiInstance, GeminiMessage, GeminiRequest, GeminiResponse,
};
use crate::services::google_cloud_authentication::GoogleCloudAuthentication;
use actix_web::{web, Result};

async fn generate_response(
    state: web::Data<AppState>,
    query: &str,
    context: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://us-central1-aiplatform.googleapis.com/v1/projects/{}/locations/us-central1/publishers/google/models/gemini-pro:predict", state.project_id);
    let client = GoogleCloudAuthentication::get_authenticated_client()
        .await
        .unwrap();
    let request = GeminiRequest {
        instances: vec![GeminiInstance {
            context: context.to_string(),
            examples: vec![],
            messages: vec![GeminiMessage {
                author: "user".to_string(),
                content: query.to_string(),
            }],
        }],
    };

    let response = client.post(&url).json(&request).send().await?;

    let gemini_response: GeminiResponse = response.json().await?;
    Ok(gemini_response.predictions[0].candidates[0].content.clone())
}
