//! Types and structs associated with light client state
use ark_ed_on_bn254::EdwardsConfig as Config;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::SeedableRng;
use ethers::types::U256;
use jf_primitives::signatures::schnorr;
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Concrete type for light client state
pub type LightClientState = hotshot_types::light_client::LightClientState<ark_ed_on_bn254::Fq>;
/// Signature scheme
pub type StateSignatureScheme =
    jf_primitives::signatures::schnorr::SchnorrSignatureScheme<ark_ed_on_bn254::EdwardsConfig>;
/// Signatures
pub type StateSignature = schnorr::Signature<Config>;
/// Verification key for verifying state signatures
pub type StateVerKey = schnorr::VerKey<Config>;
/// Signing key for signing a light client state
pub type StateSignKey = schnorr::SignKey<ark_ed_on_bn254::Fr>;
/// Key pairs for signing/verifying a light client state
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct StateKeyPair(schnorr::KeyPair<Config>);

impl std::ops::Deref for StateKeyPair {
    type Target = schnorr::KeyPair<Config>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StateKeyPair {
    /// Generate key pairs from `thread_rng()`
    #[must_use]
    pub fn generate() -> StateKeyPair {
        schnorr::KeyPair::generate(&mut ark_std::rand::thread_rng()).into()
    }

    /// Generate key pairs from seed
    #[must_use]
    pub fn generate_from_seed(seed: [u8; 32]) -> StateKeyPair {
        schnorr::KeyPair::generate(&mut ChaCha20Rng::from_seed(seed)).into()
    }

    /// Generate key pairs from an index and a seed
    #[must_use]
    pub fn generate_from_seed_indexed(seed: [u8; 32], index: u64) -> StateKeyPair {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&seed);
        hasher.update(&index.to_le_bytes());
        let new_seed = *hasher.finalize().as_bytes();
        Self::generate_from_seed(new_seed)
    }
}

impl From<schnorr::KeyPair<Config>> for StateKeyPair {
    fn from(value: schnorr::KeyPair<Config>) -> Self {
        StateKeyPair(value)
    }
}

/// Request body to send to the state relay server
#[derive(Clone, Debug, CanonicalSerialize, CanonicalDeserialize, Serialize, Deserialize)]
pub struct StateSignatureRequestBody {
    pub key: StateVerKey,
    pub state: LightClientState,
    pub signature: StateSignature,
}

/// The state signatures bundle is a light client state and its signatures collected
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateSignaturesBundle {
    /// The state for this signatures bundle
    pub state: LightClientState,
    /// The collected signatures
    pub signatures: HashMap<StateVerKey, StateSignature>,

    /// Total stakes associated with the signer
    pub accumulated_weight: U256,
}