use actix_web::{web, Result};
use actix_web::{HttpResponse, Responder};
use serde_json::json;

async fn rag_query(state: web::Data<AppState>, query: web::Json<Query>) -> impl Responder {
    match query_embeddings(state.clone(), &query.text).await {
        Ok(relevant_doc) => {
            match generate_response(state.clone(), &query.text, &relevant_doc).await {
                Ok(response) => HttpResponse::Ok().json(serde_json::json!({
                    "query": query.text,
                    "relevant_document": relevant_doc,
                    "generated_response": response
                })),
                Err(e) => HttpResponse::InternalServerError()
                    .body(format!("Error generating response: {}", e)),
            }
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error querying embeddings: {}", e))
        }
    }
}
