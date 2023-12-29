use actix_web::{get, HttpResponse, web};
use actix_web::http::StatusCode;
use the_logger::{log_error, TheLogger};
use crate::api::json_response;
use crate::configurations::data::Data;
use crate::modules::wallets::{GetWalletRequest, Wallet};

#[get("get_wallet")]
async fn get_wallet(body: web::Json<GetWalletRequest>) -> HttpResponse {

    let logger = TheLogger::instance();

    let mut conn = match Data::get_conn().await {
        Ok(conn) => conn,
        Err(e) => {
            log_error!(logger, "Error getting database connection up!: {}", e);
            return HttpResponse::InternalServerError().finish()
        }
    };

    match Wallet::select_by_alias(&mut conn, &body.into_inner().alias).await {
        Ok(Some(wallet)) => json_response(StatusCode::OK, wallet),
        Ok(None) => json_response(StatusCode::BAD_REQUEST, "No wallet was found for requested alias"),
        Err(e) => {
            log_error!(logger, "Error fetching wallet from db: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


//  TODO add create wallet service