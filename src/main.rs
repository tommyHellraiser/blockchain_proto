
use error_mapper::{create_new_error, TheResult};
use mysql_async::prelude::Queryable;
use crate::configurations::environment::Environment;


mod modules;
mod configurations;

#[tokio::main]
async fn main() {

    Environment::instance().print_content().await;


}

#[allow(dead_code)]
async fn schema_reset() -> TheResult<()> {

    let pool = mysql_async::Pool::new("mysql://root@127.0.0.1:3306/blockchain");
    let mut conn = pool.get_conn().await.map_err(|e| create_new_error!(e))?;

    let query = r#"INSERT INTO transactions(origin_wallet, destination_wallet, destination_wallet_ID, amount)
VALUES('6969lmao', '420_blaze_it', 2, 50);"#;

    conn.query_drop(query).await.map_err(|e| create_new_error!(e))?;

    Ok(())
}


