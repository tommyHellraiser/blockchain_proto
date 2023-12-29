use std::fmt::{Display, Formatter};
use actix_web::web;
use error_mapper::{create_new_error, TheResult};
use mysql_async::prelude::{FromRow, Queryable};
use mysql_async::{FromRowError, Row};
use rust_decimal::Decimal;
use serde::Serialize;
use crate::configurations::data::Data;
use crate::{data_from_row, data_from_row_enum};
use crate::modules::wallets::Wallet;
use crate::utilities::{BlockIdType, KeyType, TransactionHashType, WalletIdType};

mod services;
mod data;
pub mod pending;

pub(in super::super) fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(services::post_transaction)
        .service(services::pending_transactions);
}

#[derive(Clone, Debug, Serialize)]
pub(in super::super) struct Transaction {
    block_id: Option<BlockIdType>,
    status: TransactionStatus,
    origin_wallet: Option<KeyType>,
    origin_wallet_id: Option<WalletIdType>,
    destination_wallet: KeyType,
    destination_wallet_id: WalletIdType,
    amount: Decimal,
    hash: TransactionHashType
}

#[derive(Clone, Debug, Serialize, Default)]
pub(in super::super) enum TransactionStatus{
    #[default]
    Pending,
    Confirmed,
    Error,
    Unknown
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
            status: TransactionStatus::default(),
            origin_wallet: origin_wallet_key.clone(),
            origin_wallet_id,
            destination_wallet: destination_wallet.get_public_key(),
            destination_wallet_id: destination_wallet.get_id(),
            amount,
            hash: sha256::digest(
                format!(
                    "{}{}{}{}{}{}",
                     TransactionStatus::default(),
                     origin_wallet_key.unwrap_or_default(),
                     origin_wallet_id.unwrap_or_default(),
                     destination_wallet.get_public_key(),
                     destination_wallet.get_id(),
                     amount
                )
            )
        }
    }

    pub(super) async fn insert(
        &self
    ) -> TheResult<bool> {

        let mut conn = Data::get_conn().await?;

        let mut stmt = String::from("INSERT INTO transactions (status, ");
        let mut values = String::from("VALUES (?, ");
        let mut params: Vec<String> = vec!["Pending".to_string()];

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

        stmt.push_str("destination_wallet, destination_wallet_ID, amount, hash) ");
        values.push_str("?, ?, ?, ?);");
        params.push(self.destination_wallet.clone());
        params.push(self.destination_wallet_id.to_string());
        params.push(self.amount.to_string());
        params.push(self.hash.clone());

        stmt.push_str(&values);

        conn.exec_drop(stmt, params).await.map_err(|e| create_new_error!(e))?;

        Ok(conn.affected_rows() > 0)

    }

    pub(super) async fn select_pending() -> TheResult<Vec<Transaction>> {

        let mut conn = Data::get_conn().await?;

        let stmt = "SELECT block_ID, status, origin_wallet, origin_wallet_ID, destination_wallet, destination_wallet_ID, \
        amount FROM transactions WHERE status = 'Pending' ORDER BY ID";

        conn.query::<Transaction, _>(stmt).await.map_err(|e| create_new_error!(e))

    }


    pub(super) async fn select_pending_hash() -> TheResult<Vec<TransactionHashType>> {

        let mut conn = Data::get_conn().await?;

        let stmt = "SELECT hash FROM transactions WHERE status = 'Pending' ORDER BY ID";

        conn.query::<TransactionHashType, _>(stmt).await.map_err(|e| create_new_error!(e))

    }

    pub(super) async fn link_pending_to_last_block() -> TheResult<bool> {

        let mut conn = Data::get_conn().await?;

        let stmt = "SELECT ID FROM blocks ORDER BY ID DESC LIMIT 1";

        let block_id = conn.query_first::<BlockIdType, _>(stmt).await.map_err(|e| create_new_error!(e))?;

        if let Some(id) = block_id {
            let stmt = format!(
                "UPDATE transactions SET block_ID = {}, status = 'Confirmed' WHERE status = 'Pending'",
                id
            );

            conn.query_drop(stmt).await.map_err(|e| create_new_error!(e))?;
            Ok(conn.affected_rows() > 0)

        } else {
            Err(create_new_error!("No blocks found in database!"))
        }

    }
}

impl FromRow for Transaction {
    fn from_row(row: Row) -> Self where Self: Sized {
        Self {
            block_id: data_from_row!(row, "block_ID", Option<BlockIdType>),
            status: data_from_row_enum!(row, "status", TransactionStatus),
            origin_wallet: data_from_row!(row, "origin_wallet", Option<KeyType>),
            origin_wallet_id: data_from_row!(row, "origin_wallet_ID", Option<WalletIdType>),
            destination_wallet: data_from_row!(row, "destination_wallet", KeyType),
            destination_wallet_id: data_from_row!(row, "destination_wallet_ID", WalletIdType),
            amount: data_from_row!(row, "amount", Decimal),
            hash: data_from_row!(row, "hash", TransactionHashType)
        }
    }
    fn from_row_opt(_row: Row) -> Result<Self, FromRowError> where Self: Sized {
        unimplemented!()
    }
}

impl From<String> for TransactionStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Pending" => Self::Pending,
            "Confirmed" => Self::Confirmed,
            "Error" => Self::Error,
            "Unknown" => Self::Unknown,
            _ => Self::Unknown
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}{}{}",
               self.status,
               self.origin_wallet.clone().unwrap_or_default(),
               self.origin_wallet_id.unwrap_or_default(),
               self.destination_wallet,
               self.destination_wallet_id,
               self.amount
        )
    }
}

impl Display for TransactionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "Pending"),
            Self::Confirmed => write!(f, "Confirmed"),
            Self::Error => write!(f, "Error"),
            Self::Unknown => write!(f, "Unknown")
        }
    }
}
