use the_logger::{log_error, log_info, TheLogger};
use crate::api::StopMethod;
use crate::configurations::data::Data;


mod modules;
mod configurations;
mod api;

#[tokio::main]
async fn main() {

    let logger = TheLogger::instance();

    match Data::reset_database().await {
        Ok(_) => log_info!(logger, "Database reset successfully"),
        Err(e) => log_error!(logger, "Error resetting database schema: {}", e)
    };

    let (sender, receiver) = tokio::sync::broadcast::channel::<StopMethod>(4);

    match api::start_api(
        sender,
        receiver
    ).await {
        Ok(_) => log_info!(logger, "Api shut down successfully"),
        Err(e) => log_error!(logger, "Error shutting down api: {}", e)
    };

}
