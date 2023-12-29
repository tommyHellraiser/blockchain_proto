use actix_web::web;
use error_mapper::{create_new_error, TheResult};
use mysql_async::prelude::Queryable;
use rust_decimal::Decimal;
use crate::configurations::data::Data;
use crate::modules::wallets::Wallet;
use crate::utilities::{BlockIdType, KeyType, WalletIdType};

mod services;
mod data;

pub(in super::super) fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(services::post_transaction);
}

pub(in super::super) struct Transaction {
    block_id: Option<BlockIdType>,
    origin_wallet: Option<KeyType>,
    origin_wallet_id: Option<WalletIdType>,
    destination_wallet: KeyType,
    destination_wallet_id: WalletIdType,
    amount: Decimal
}

impl Transaction {
    pub(super) fn new(
        origin_wallet: Option<Wallet>,
        destination_wallet: Wallet,
        amount: Decimal
    ) -> Self {

        let (origin_wallet_key, origin_wallet_id) = if let Some(wallet) = origin_wallet {
            (Some(wallet.get_public_key()), Some(wallet.get_id()))
        } else {
            (None, None)
        };

        Self {
            block_id: None,
            origin_wallet: origin_wallet_key,
            origin_wallet_id,
            destination_wallet: destination_wallet.get_public_key(),
            destination_wallet_id: destination_wallet.get_id(),
            amount
        }
    }

    pub(super) async fn insert(
        &self
    ) -> TheResult<bool> {

        let mut conn = Data::get_conn().await?;

        let mut stmt = String::from("INSERT INTO transactions (");
        let mut values = String::from("VALUES (");
        let mut params: Vec<String> = vec![];

        if let Some(origin_wallet) = &self.origin_wallet {
            stmt.push_str("origin_wallet, ");
            values.push_str("?, ");
            params.push(origin_wallet.clone());
        }

        if let Some(origin_id) = &self.origin_wallet_id {
            stmt.push_str("origin_wallet_ID, ");
            values.push_str("?, ");
            params.push(origin_id.to_string());
        }

        stmt.push_str("destination_wallet, destination_wallet_ID, amount) ");
        values.push_str("?, ?, ?);");
        params.push(self.destination_wallet.clone());
        params.push(self.destination_wallet_id.to_string());
        params.push(self.amount.to_string());

        stmt.push_str(&values);

        conn.exec_drop(stmt, params).await.map_err(|e| create_new_error!(e))?;

        Ok(conn.affected_rows() > 0)

    }
}
