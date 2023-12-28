
UPDATE blocks
SET previous_block_ID = 1
WHERE ID = 1;

INSERT INTO wallets(alias, public_key)
VALUES('tommy', '031f187a3e242b39bf7f51c7b2aedb3355595f9cea0b39065289284ff19f43c46d');

INSERT INTO wallets(alias, public_key)
VALUES('hellraiser', '027296dca1fb1e3e03d20c695db0565c51eefb0e8b853a30b274d8464d55ff6c29');

INSERT INTO wallets_balances(wallet_ID, balance)
VALUES(1, 100), (2, 100);

INSERT INTO transactions(destination_wallet, destination_wallet_ID, amount)
VALUES('031f187a3e242b39bf7f51c7b2aedb3355595f9cea0b39065289284ff19f43c46d', 1, 100);

INSERT INTO transactions(destination_wallet, destination_wallet_ID, amount)
VALUES('027296dca1fb1e3e03d20c695db0565c51eefb0e8b853a30b274d8464d55ff6c29', 2, 100);

INSERT INTO wallets_keys(wallet_ID, private, public)
VALUES
(1, 'a27fb6e1e73530b237ec8baf7ac8774bf821d462b2502874aa10c546033ca139', '031f187a3e242b39bf7f51c7b2aedb3355595f9cea0b39065289284ff19f43c46d'),
(2, 'fa5e976782a9c59ba5a6bdbe0c193135de3962ea8b1ecaf3b438c43b3259d552', '027296dca1fb1e3e03d20c695db0565c51eefb0e8b853a30b274d8464d55ff6c29');