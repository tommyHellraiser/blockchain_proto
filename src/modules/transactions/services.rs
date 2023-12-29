use actix_web::{HttpResponse, post, web};
use rust_decimal::Decimal;
use serde::Deserialize;
use the_logger::{log_error, log_info, TheLogger};
use crate::modules::transactions::data;

#[derive(Deserialize)]
pub(super) struct NewTransactionRequest {
    pub(super) origin: Option<String>,
    pub(super) destination: String,
    pub(super) amount: Decimal
}

#[post("/new_transaction")]
pub(super) async fn post_transaction(body: web::Json<NewTransactionRequest>) -> HttpResponse {

    let logger = TheLogger::instance();

    match data::process_transaction(body.into_inner()).await {
        Ok(_) => {
            log_info!(logger, "Transaction processed successfully");
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log_error!(logger, "Error processing transaction: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
