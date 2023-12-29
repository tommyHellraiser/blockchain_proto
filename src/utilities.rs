pub type WalletIdType = u16;
pub type AliasType = String;
pub type KeyType = String;

#[macro_export]
macro_rules! data_from_row {
    ($row: ident, $col: literal, $datatype: ty) => {
        $row.get::<$datatype, _>($col).unwrap_or_else(|| {
            panic!("Element {} was not found", $col);
        })
    };
}