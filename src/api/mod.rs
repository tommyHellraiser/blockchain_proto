mod services;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::{ServerHandle};
use actix_web::http::StatusCode;
use error_mapper::{create_new_error, TheResult};
use serde::Serialize;
use the_logger::{log_error, log_info, TheLogger};
use tokio::sync::broadcast::{Receiver, Sender};
use crate::configurations::environment::Environment;
use crate::modules::balances;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum StopMethod {
    Graceful,
    Force
}

#[derive(Clone, Debug)]
pub struct AppData {
    pub sender: Sender<StopMethod>
}

pub async fn start_api(
    sender: Sender<StopMethod>,
    receiver: Receiver<StopMethod>
) -> TheResult<()> {

    let (api_base_addr, api_port, api_workers) = Environment::instance().get_api_config().await;

    let server = HttpServer::new(move ||
        {
            App::new().service(
                web::scope("/api")
                    .service(
                        web::scope("/services").configure(services::services)
                    )
                    .service(
                        web::scope("/balances").configure(balances::services)
                    )
            ).app_data(web::Data::new(AppData { sender: sender.clone() }))
        }).bind((api_base_addr, api_port)).map_err(|e| create_new_error!(e))?
        .workers(api_workers)
        .run();

    tokio::spawn(stop_app(receiver, server.handle()));

    log_info!(TheLogger::instance(), "Starting Http server...");

    server.await.map_err(|e| create_new_error!(e))?;

    Ok(())
}

async fn stop_app(mut receiver: Receiver<StopMethod>, server: ServerHandle) {

    let logger = TheLogger::instance();

    match receiver.recv().await {
        Ok(StopMethod::Graceful) => {
            log_info!(logger, "Http server graceful shutdown started");
            server.stop(true).await;
        },
        Ok(StopMethod::Force) => {
            log_info!(logger, "Http server forced shutdown started");
            server.stop(false).await;
        },
        Err(e) => {
            log_error!(logger, "Error in broadcast channel receiver: {}", e);
        }
    }
}

pub fn json_response<T: Serialize>(code: StatusCode, body: T) -> HttpResponse {
    HttpResponse::build(code).json(body)
}
