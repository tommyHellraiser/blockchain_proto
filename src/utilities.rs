
pub type WalletIdType = u16;
pub type AliasType = String;
pub type KeyType = String;
pub type BlockIdType = u64;
pub type BlockHashType = String;
pub type TransactionHashType = String;

#[macro_export]
macro_rules! data_from_row {
    ($row: ident, $col: literal, $datatype: ty) => {
        $row.get::<$datatype, _>($col).unwrap_or_else(|| {
            panic!("Element {} was not found", $col);
        })
    };
}

#[macro_export]
macro_rules! data_from_row_enum {
    ($row: ident, $col: literal, $datatype: ty) => {
        {
            let value = $row.get::<String, _>($col).unwrap_or_else(|| {
                panic!("Element {} was not found", $col);
            });
            <$datatype>::from(value)
        }
    };
}

#[macro_export]
macro_rules! data_from_row_datetime {
    ($row: ident, $col: literal, $format: ident) => {
        {
            let value = $row.get::<String, _>($col).unwrap_or_else(|| {
                panic!("Element {} was not found", $col);
            });
            NaiveDateTime::parse_from_str(&value, $format).unwrap_or_else(|_| {
                panic!("Could not parse Datetime from row value");
            })
        }
    };
}
