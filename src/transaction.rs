use serde::{Serialize, Deserialize};
use ring::{digest, signature};
use ring::signature::{Ed25519KeyPair, KeyPair, Signature, UnparsedPublicKey};
use crate::crypto::hash::Hashable;
use crate::crypto::hash::H256;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Transaction {
    input: Vec<Input>,
    output: Vec<Output>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Input {
    // Add fields for input here, such as an address or transaction id
    pub address: String,
    pub transaction_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Output {
    // Add fields for output here, such as an address and amount
    pub address: String,
    pub amount: u64,
}

/// Create digital signature of a transaction
pub fn sign(t: &Transaction, key: &Ed25519KeyPair) -> Signature {
    let data = bincode::serialize(t).unwrap();
    key.sign(&data)
}

/// Verify digital signature of a transaction, using public key instead of secret key
pub fn verify(t: &Transaction, public_key: &<Ed25519KeyPair as KeyPair>::PublicKey, signature: &Signature) -> bool {
    match ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public_key.as_ref()).verify(
    bincode::serialize(&t).unwrap().as_ref(), signature.as_ref()) {
        Result::Ok(()) => true,
        Result::Err(_) => false,
    }
}

impl Hashable for Transaction {
    fn hash(&self) -> H256 {
        let bytes = bincode::serialize(&self).unwrap();
        ring::digest::digest(&ring::digest::SHA256, &bytes).into()
    }
}

#[cfg(any(test, test_utilities))]
mod tests {
    use super::*;
    use crate::crypto::key_pair::{self, random};

    pub fn generate_random_transaction() -> Transaction {
        let num = rand::random::<u8>()%2;
        let mut input = vec![];
        if(num > 0) {
            input = vec![Default::default()];
        }
        let output = vec![];
        Transaction { input, output }
    }

    #[test]
    fn sign_verify() {
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);
        assert!(verify(&t, &(key.public_key()), &signature));
    }
}