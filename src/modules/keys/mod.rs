use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use error_mapper::{create_new_error, TheResult};
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use secp256k1::ecdsa::Signature;
use secp256k1::hashes::sha256;
use secp256k1::rand::rngs::OsRng;

pub(in super::super) fn generate_key_pair() -> (String, String) {

    //  Secp engine
    let secp = Secp256k1::new();
    //  Generate key pair
    let (secret, public) = secp.generate_keypair(&mut OsRng);

    (
        secret.display_secret().to_string(),
        public.to_string()
    )

}

pub(in super::super) fn validate_message(
    public: &str,
    message: &str
) -> TheResult<bool> {
    //  Load secp engine
    let secp = Secp256k1::new();

    //  Create keys from str
    let public = PublicKey::from_str(public).map_err(|e| create_new_error!(e))?;

    let mut hasher = DefaultHasher::new();
    message.hash(&mut hasher);
    let hashed = hasher.finish().to_string();
    let msg = hashed.as_bytes();
    let signature = Signature::from_str(&message).map_err(|e| create_new_error!(e))?;
    let message = Message::from_hashed_data::<sha256::Hash>(msg);
    //  Hashed message, signed message and public keys needed
    Ok(secp.verify_ecdsa(&message, &signature, &public).is_ok())
}

pub(in super::super) fn sign_message(
    secret: &str,
    message: &str
) -> TheResult<String> {

    //  Instance the secp engine
    let secp = Secp256k1::new();
    //  Create secret key from string
    let secret = SecretKey::from_str(secret).map_err(|e| create_new_error!(e))?;

    //  Hash the string message
    let mut hasher = DefaultHasher::new();
    message.hash(&mut hasher);
    let hashed = hasher.finish().to_string();
    let msg = hashed.as_bytes();

    //  Create the message object and the signature
    let message = Message::from_hashed_data::<sha256::Hash>(msg);
    let signature = secp.sign_ecdsa(&message, &secret);
    
    Ok(signature.to_string())
}