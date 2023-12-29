use actix_web::{HttpResponse, put, web};
use actix_web::http::StatusCode;
use the_logger::{log_error, log_info, TheLogger};
use crate::api::{AppData, json_response, StopMethod};

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(stop);
}

#[put("/stop")]
async fn stop(api_data: web::Data<AppData>) -> HttpResponse {

    let logger = TheLogger::instance();

    match api_data.sender.send(StopMethod::Graceful) {
        Ok(_) => {
            log_info!(logger, "Shutdown request received");
            json_response(StatusCode::OK, "Stopping http server")
        },
        Err(e) => {
            log_error!(logger, "Error sending shutdown request: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

}
