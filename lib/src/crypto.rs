use ecdsa::{
    signature::{rand_core::OsRng, Signer, Verifier},
    Signature as ECDSASignature, SigningKey, VerifyingKey,
};
use k256::Secp256k1;
use serde::{Deserialize, Serialize};

use crate::sha256::Hash;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature(pub ECDSASignature<Secp256k1>);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PublicKey(pub VerifyingKey<Secp256k1>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateKey(#[serde(with = "signkey_serde")] pub SigningKey<Secp256k1>);

mod signkey_serde {
    use serde::Deserialize;

    pub fn serialize<S>(
        key: &super::SigningKey<super::Secp256k1>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&key.to_bytes())
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<super::SigningKey<super::Secp256k1>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        Ok(super::SigningKey::from_slice(&bytes).unwrap())
    }
}

impl PrivateKey {
    pub fn new_key() -> Self {
        // PrivateKey(SigningKey::random(&mut rand::thread_rng()));
        PrivateKey(SigningKey::random(&mut OsRng)) // incase result is not as expected
    }
    pub fn public_key(&self) -> PublicKey {
        PublicKey(*self.0.verifying_key())
    }
}

impl Signature {
    pub fn sign_output(output_hash: &Hash, private_key: &PrivateKey) -> Self {
        let signing_key = &private_key.0;
        let signature = signing_key.sign(&output_hash.as_bytes());
        Signature(signature)
    }

    pub fn verify(&self, output_hash: &Hash, public_key: &PublicKey) -> bool {
        public_key
            .0
            .verify(&output_hash.as_bytes(), &self.0)
            .is_ok()
    }
}
