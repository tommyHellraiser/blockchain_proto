
UPDATE blocks
SET previous_block_ID = 1
WHERE ID = 1;

INSERT INTO wallets(alias, public_key)
VALUES('tommy', '6969lmao');

INSERT INTO wallets(alias, public_key)
VALUES('dest_tommy', '420_blaze_it');

INSERT INTO wallets_balances(wallet_ID, balance)
VALUES(1, 100), (2, 100);

INSERT INTO transactions(origin_wallet, destination_wallet, destination_wallet_ID, amount)
VALUES('6969lmao', '420_blaze_it', 2, 50);
