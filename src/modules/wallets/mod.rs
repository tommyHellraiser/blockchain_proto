mod services;

use actix_web::web::ServiceConfig;
use error_mapper::{create_new_error, TheResult};
use mysql_async::prelude::{FromRow, Queryable};
use mysql_async::{Conn, FromRowError, Row};
use serde::{Deserialize, Serialize};
use crate::utilities::{AliasType, KeyType, WalletIdType};
use crate::data_from_row;

pub(in super::super) fn services(cfg: &mut ServiceConfig) {
    cfg.service(services::get_wallet);
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct GetWalletRequest {
    alias: AliasType
}

#[derive(Clone, Debug, Serialize)]
pub(in super::super) struct Wallet {
    id: WalletIdType,
    alias: AliasType,
    public_key: KeyType
}

impl Wallet {
    pub(in super::super) async fn select_by_id(conn: &mut Conn, id: WalletIdType) -> TheResult<Option<Self>> {

        let stmt = format!(
            "SELECT ID, alias, public_key FROM wallets WHERE ID = {}",
            id
        );

        let wallet = conn.query_first::<Self, _>(
            stmt
        ).await.map_err(|e| create_new_error!(e))?;

        Ok(wallet)

    }

    pub(in super::super) async fn select_by_key(conn: &mut Conn, key: &str) -> TheResult<Option<Self>> {

        let stmt = format!(
            "SELECT ID, alias, public_key FROM wallets WHERE public_key = '{}'",
            key
        );

        let wallet = conn.query_first::<Self, _>(
            stmt
        ).await.map_err(|e| create_new_error!(e))?;

        Ok(wallet)
    }

    pub(super) async fn select_by_alias(conn: &mut Conn, alias: &str) -> TheResult<Option<Self>> {

        let stmt = format!(
            "SELECT ID, alias, public_key FROM wallets WHERE alias = '{}'",
            alias
        );

        let wallet = conn.query_first::<Self, _>(stmt).await.map_err(|e| create_new_error!(e))?;

        Ok(wallet)
    }

    pub fn get_id(&self) -> WalletIdType {
        self.id
    }

    pub fn get_alias(&self) -> AliasType {
        self.alias.clone()
    }

    pub fn get_public_key(&self) -> KeyType {
        self.public_key.clone()
    }
}

impl FromRow for Wallet {
    fn from_row(row: Row) -> Self where Self: Sized {
        Self {
            id: data_from_row!(row, "ID", WalletIdType),
            alias: data_from_row!(row, "alias", AliasType),
            public_key: data_from_row!(row, "public_key", KeyType)
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError> where Self: Sized {
        unimplemented!()
    }
}
