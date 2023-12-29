use the_logger::{log_error, log_info, TheLogger};
use crate::api::StopMethod;
use crate::configurations::data::Data;
use crate::configurations::environment::Environment;


mod modules;
mod configurations;
mod api;
mod utilities;

#[tokio::main]
async fn main() {

    let logger = TheLogger::instance();

    if Environment::instance().reset_schema().await {
        match Data::reset_database().await {
            Ok(_) => log_info!(logger, "Database reset successfully"),
            Err(e) => log_error!(logger, "Error resetting database schema: {}", e)
        };
    }
    //  TODO add block mine cron
    //   when block is mined, all transactions that are pending need to be saved into that block with the hash and stuff
    //   

    let (sender, receiver) = tokio::sync::broadcast::channel::<StopMethod>(4);

    match api::start_api(
        sender,
        receiver
    ).await {
        Ok(_) => log_info!(logger, "Api shut down successfully"),
        Err(e) => log_error!(logger, "Error shutting down api: {}", e)
    };

}
