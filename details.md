Transaction content (simplified):
-origin_addr
-dest_adddr
-amount

Block content (in database):
-Transactions
-Merkle root
-Timestamp
-Previous block hash
-Previous block height
-Height

Block chain should be stored in files, with all content hashed and signed. For hashing we'll use SHA256
For signing we'll use the openssl library

The block message will have a structure similar to this one:
https://learnmeabitcoin.com/technical/blkdat



database tables:
-transactions
	id
	block_id
	origin_address
	destination_address
	amount
	creation_date
-blocks
	id (height)
	hash
	previous_block_hash
	merkle_root
	timestamp
wallets
	id
	public_key
	balance
	creation_date
	deletion_date
keys (for testing only)
	wallet_id
	public_ley
	private_key

api details:
- need to create transactions
- need to create wallets with automatic generation of user alias, keys and shit (request to an external site, or smth)
- need to mine blocks (now sure it needs to be an api feature)
- need to check balances when requested
- need to get details for a wallet


general features:
- cron to mine blocks every 1 minute or so
- need to fail transaction if balance is lower than requested for transaction (check balance before executing transaction)
- add all transactions from pending to block once a block has been mined

modules needed for now:
- balances
- keys
- transactions
- wallets