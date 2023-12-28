use std::io::Read;
use error_mapper::{create_new_error, TheResult};
use mysql_async::{Conn, Pool};
use mysql_async::prelude::Queryable;
use crate::configurations::environment::Environment;

pub struct Data;

impl Data {
    pub async fn get_conn() -> TheResult<Conn> {
        let addr = Environment::instance().get_db_addr().await;
        let pool = Pool::new(addr.as_str());
        pool.get_conn().await.map_err(|e| create_new_error!(e))
    }

    pub async fn reset_database() -> TheResult<()> {
        let mut conn = Self::get_conn().await?;

        let (schema_reset, data_reset) = Self::load_schema_reset().await?;

        conn.query_drop(schema_reset).await.map_err(|e| create_new_error!(e))?;
        conn.query_drop(data_reset).await.map_err(|e| create_new_error!(e))?;

        Ok(())
    }

    async fn load_schema_reset() -> TheResult<(String, String)> {

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open("sql/schema_reset.sql")
            .map_err(|e| create_new_error!(e))?;

        let mut schema_reset = String::new();
        file.read_to_string(&mut schema_reset).map_err(|e| create_new_error!(e))?;

        let mut data_reset = String::new();
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open("sql/data_reset.sql")
            .map_err(|e| create_new_error!(e))?;
        file.read_to_string(&mut data_reset).map_err(|e| create_new_error!(e))?;

        Ok((schema_reset, data_reset))

    }
}
