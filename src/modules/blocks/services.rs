use actix_web::{HttpResponse, post};
use the_logger::{log_error, TheLogger};
use crate::modules::blocks::Block;

#[post("/mine")]
async fn mine() -> HttpResponse {

    match Block::mine().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            log_error!(TheLogger::instance(), "Error mining block: {}: ", e);
            HttpResponse::InternalServerError().finish()
        }
    }

}
