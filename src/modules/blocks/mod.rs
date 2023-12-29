mod data;
mod services;

use actix_web::web;
use chrono::NaiveDateTime;
use error_mapper::{create_new_error, TheResult};
use mysql_async::prelude::{FromRow, Queryable};
use mysql_async::{FromRowError, Row};
use crate::configurations::data::Data;
use crate::{data_from_row, data_from_row_datetime};
use crate::modules::transactions::Transaction;
use crate::utilities::{BlockHashType, BlockIdType};

const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub(in super::super) fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(services::mine);
}

#[derive(Default, Debug, Clone)]
struct Block {
    block_hash: BlockHashType,
    previous_block_hash: Option<BlockHashType>,
    previous_block_id: Option<BlockIdType>,
    mine_datetime: NaiveDateTime
}

impl Block {
    async fn mine() -> TheResult<()> {

        let transactions = Transaction::select_pending_hash().await?;

        let mut tx_hashes = String::new();
        transactions.iter().for_each(|t| tx_hashes.push_str(t));
        let tx_hash = sha256::digest(tx_hashes);

        //  Ignoring the first 2 digits of the year
        let now = chrono::Utc::now().naive_utc();
        let now_hash = now.format("%Y%m%d%H%M%S").to_string()[2..].to_string();

        let last_block = match Self::select_last_block().await {
            Ok(Some(block)) => { block },
            Ok(None) => {
                //  If no previous block was found, this is the genesis block
                Block::default()
            },
            Err(e) => {
                return Err(e);
            }
        };

        //  For the hash we'll choose an arbitrary content. Let's say: amount of transactions for this block, the hash
        // for all transactions' hashes, timestamp, and previous block hash if there's any
        let content_to_hash = format!("{}{}{}{}", transactions.len(), tx_hash, now_hash, last_block.previous_block_hash.clone().unwrap_or_default());

        let block = Block {
            block_hash: sha256::digest(content_to_hash),
            previous_block_hash: last_block.previous_block_hash,
            previous_block_id: last_block.previous_block_id,
            mine_datetime: now
        };

        //  Insert mined block in database
        let executed = block.insert().await?;
        if !executed {
            return Err(create_new_error!("Mined block was not inserted in database!"));
        }

        //  Link pending transactions to this block
        let executed = Transaction::link_pending_to_last_block().await?;
        if !executed {
            return Err(create_new_error!("Pending transactions were not updated!"));
        }

        Ok(())
    }

    async fn insert(&self) -> TheResult<bool> {

        let mut conn = Data::get_conn().await?;

        let mut stmt = String::from("INSERT INTO blocks (block_hash");
        let mut values = String::from(" VALUES (?, ");
        let mut params = vec![self.block_hash.clone()];

        if let Some(previous_block_hash) = &self.previous_block_hash {
            stmt.push_str(", previous_block_hash");
            values.push_str("?, ");
            params.push(previous_block_hash.clone());
        }

        if let Some(previous_block_id) = &self.previous_block_id {
            stmt.push_str(", previous_block_id");
            values.push_str("?, ");
            params.push(previous_block_id.to_string());
        }

        stmt.push_str(", mine_datetime)");
        values.push_str("?);");
        params.push(self.mine_datetime.format(DATETIME_FORMAT).to_string());

        stmt.push_str(&values);

        conn.exec_drop(stmt, params).await.map_err(|e| create_new_error!(e))?;

        Ok(conn.affected_rows() > 0)
    }

    async fn select_last_block() -> TheResult<Option<Block>> {

        let mut conn = Data::get_conn().await?;

        let stmt = "SELECT ID, block_hash, previous_block_hash, previous_block_ID, mine_datetime FROM blocks ORDER BY ID DESC LIMIT 1";

        conn.query_first::<Block, _>(stmt).await.map_err(|e| create_new_error!(e))

    }
}

impl FromRow for Block {
    fn from_row(row: Row) -> Self where Self: Sized {
        Self {
            block_hash: data_from_row!(row, "block_hash", BlockHashType),
            previous_block_hash: data_from_row!(row, "previous_block_hash", Option<BlockHashType>),
            previous_block_id: data_from_row!(row, "previous_block_id", Option<BlockIdType>),
            mine_datetime: data_from_row_datetime!(row, "mine_datetime", DATETIME_FORMAT)
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError> where Self: Sized {
        unimplemented!()
    }
}
