use actix_web::{get, HttpResponse, post, web};
use actix_web::http::StatusCode;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use the_logger::{log_error, log_info, TheLogger};
use crate::api::json_response;
use crate::modules::transactions::{data, Transaction};

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

#[get("/pending_transactions")]
async fn pending_transactions() -> HttpResponse {

    #[derive(Serialize, Clone, Debug)]
    struct PendingTransactionsResponse {
        transactions: Vec<Transaction>
    }

    match Transaction::select_pending().await {
        Ok(transactions) => {
            json_response(StatusCode::OK, PendingTransactionsResponse { transactions })
        },
        Err(e) => {
            log_error!(TheLogger::instance(), "Error selecting pending transactions: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
