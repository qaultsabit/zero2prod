use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Parameter {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(_parameters))]
pub async fn confirm(_parameters: web::Query<Parameter>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
