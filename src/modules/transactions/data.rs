use error_mapper::{create_new_error, TheResult};
use mysql_async::Conn;
use mysql_async::prelude::Queryable;
use rust_decimal::Decimal;
use crate::configurations::data::Data;
use crate::modules::transactions::services::NewTransactionRequest;
use crate::modules::transactions::Transaction;
use crate::modules::wallets::Wallet;

pub(super) async fn process_transaction(transaction: NewTransactionRequest) -> TheResult<()> {

    let mut conn = Data::get_conn().await?;

    let origin_wallet = if let Some(origin) = transaction.origin {
        match Wallet::select_by_key(&mut conn, &origin).await? {
            Some(wallet) => Some(wallet),
            None => return Err(create_new_error!("No wallet found for requested origin key"))
        }
    } else {
        None
    };


    let destination_wallet = match Wallet::select_by_key(&mut conn, &transaction.destination).await? {
        Some(wallet) => wallet,
        None => return Err(create_new_error!("No wallet found for requested destination key"))
    };

    let transaction = Transaction::new(
        origin_wallet,
        destination_wallet,
        transaction.amount
    );

    let executed = transaction.insert().await?;

    if !executed {
        return Err(create_new_error!("Transaction could not be inserted into database"))
    }

    Ok(())
}
