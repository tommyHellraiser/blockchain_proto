DROP DATABASE if EXISTS blockchain;
CREATE DATABASE if NOT EXISTS blockchain;
USE blockchain;

DROP TABLE if EXISTS blocks;
CREATE TABLE blocks (
	ID INT(10) PRIMARY KEY AUTO_INCREMENT,
	block_hash VARCHAR(70) NOT NULL,
	previous_block_hash VARCHAR(70) NOT NULL,
	previous_block_ID INT(10) NULL,
	merkle_root VARCHAR(70) NOT NULL,
	transaction_datetime DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP(),
	CONSTRAINT blocks_previous_block_ID FOREIGN KEY (previous_block_ID) REFERENCES blocks (ID)
);

DROP TABLE if EXISTS wallets;
CREATE TABLE wallets (
	ID INT(10) PRIMARY KEY NOT NULL AUTO_INCREMENT,
	alias VARCHAR(30) NOT NULL,
	public_key VARCHAR(100) NOT NULL,
	creation_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP(),
	deletion_date DATETIME NULL
);

DROP TABLE if EXISTS transactions;
CREATE TABLE transactions (
	ID INT(10) PRIMARY KEY NOT NULL AUTO_INCREMENT,
	block_ID INT(10) NULL,
	origin_wallet VARCHAR(100) NULL,
	origin_wallet_ID INT(10) NULL,
	destination_wallet VARCHAR(100) NOT NULL,
	destination_wallet_ID INT(10) NOT NULL,
	amount DOUBLE NOT NULL,
	creation_date DATETIME DEFAULT CURRENT_TIMESTAMP(),
	CONSTRAINT transactions_block_ID FOREIGN KEY (block_ID) REFERENCES blocks (ID),
	CONSTRAINT transactions_destination_wallet_ID FOREIGN KEY (destination_wallet_ID) REFERENCES wallets (ID)
);

DROP TABLE if EXISTS wallets_balances;
CREATE TABLE wallets_balances (
	wallet_ID INT(10) NOT NULL,
	balance DOUBLE NOT NULL DEFAULT 0,
	balance_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP(),
	CONSTRAINT wallets_balances FOREIGN KEY (wallet_ID) REFERENCES wallets (ID)
);

INSERT INTO blocks (block_hash, previous_block_hash, merkle_root)
VALUES('asd', 'qwe', 'zcasdc');
