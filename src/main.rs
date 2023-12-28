
use error_mapper::{create_new_error, TheResult};
use mysql_async::prelude::Queryable;
use crate::configurations::data::Data;


mod modules;
mod configurations;

#[tokio::main]
async fn main() {

    match Data::reset_database().await {
        Ok(_) => println!("Database reset successfully"),
        Err(e) => println!("Error resetting database: {}", e)
    };

}
