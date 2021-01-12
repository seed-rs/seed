use cocoon::MiniCocoon;
use digest::Digest;
use generic_array::GenericArray;
use opaque_ke::{
    ciphersuite::CipherSuite, errors::InternalPakeError, hash::Hash, slow_hash::SlowHash,
};
use rand_core::{OsRng, RngCore};

pub use opaque_ke;
pub use rand_core;

pub struct DefaultCipherSuite;

impl CipherSuite for DefaultCipherSuite {
    type Group = curve25519_dalek::ristretto::RistrettoPoint;
    type KeyFormat = opaque_ke::keypair::X25519KeyPair;
    type KeyExchange = opaque_ke::key_exchange::tripledh::TripleDH;
    type Hash = sha2::Sha256;
    type SlowHash = ScryptHash;
}

pub struct ScryptHash;

impl<D: Hash> SlowHash<D> for ScryptHash {
    fn hash(
        input: GenericArray<u8, <D as Digest>::OutputSize>,
    ) -> Result<Vec<u8>, InternalPakeError> {
        // 256-bit derived key
        let mut derived_key = [0_u8; 32];
        scrypt::scrypt(
            input.as_slice(),
            b"salt",
            // `ScryptParams::recommended` is `15, 8, 1`
            &scrypt::ScryptParams::new(5, 8, 1).expect("new Scrypt params"),
            &mut derived_key,
        )
        .expect("32 bytes always satisfy output length requirements");
        Ok(derived_key.to_vec())
    }
}

#[must_use]
pub fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut seed = [0_u8; 32];
    OsRng.fill_bytes(&mut seed);

    let cocoon = MiniCocoon::from_key(key, &seed);
    cocoon.wrap(data).expect("encrypted data")
}

#[must_use]
pub fn decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut seed = [0_u8; 32];
    OsRng.fill_bytes(&mut seed);

    let cocoon = MiniCocoon::from_key(key, &seed);
    cocoon.unwrap(data).expect("decrypted data")
}
