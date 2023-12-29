use error_mapper::{create_new_error, TheResult};
use mysql_async::Conn;
use mysql_async::prelude::Queryable;
use rust_decimal::Decimal;
use crate::configurations::data::Data;
use crate::modules::transactions::services::NewTransaction;
use crate::modules::wallets::Wallet;

pub(super) async fn process_transaction(transaction: NewTransaction) -> TheResult<()> {

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

    let executed = insert_transaction(
        &mut conn,
        origin_wallet,
        destination_wallet,
        transaction.amount
    ).await?;

    if !executed {
        return Err(create_new_error!("Transaction could not be inserted into database"))
    }

    Ok(())
}

async fn insert_transaction(
    conn: &mut Conn,
    origin_wallet: Option<Wallet>,
    destination_wallet: Wallet,
    amount: Decimal
) -> TheResult<bool> {

    let mut stmt = String::from("INSERT INTO transactions (");
    let mut values = String::from("VALUES (");
    let mut params = vec![];

    if let Some(origin_wallet) = origin_wallet {
        stmt.push_str("origin_wallet, origin_wallet_ID, ");
        values.push_str("?, ?, ");
        params.push(origin_wallet.get_public_key());
        params.push(origin_wallet.get_id().to_string());
    }

    stmt.push_str("destination_wallet, destination_wallet_ID, amount) ");
    values.push_str("?, ?, ?);");
    params.push(destination_wallet.get_public_key());
    params.push(destination_wallet.get_id().to_string());
    params.push(amount.to_string());

    stmt.push_str(&values);

    conn.exec_drop(stmt, params).await.map_err(|e| create_new_error!(e))?;

    Ok(conn.affected_rows() > 0)

}
