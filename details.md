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