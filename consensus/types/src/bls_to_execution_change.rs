use crate::test_utils::TestRandom;
use crate::*;
use bls::PublicKeyBytes;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use test_random_derive::TestRandom;
use tree_hash_derive::TreeHash;

#[derive(
    arbitrary::Arbitrary,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Clone,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    TreeHash,
    TestRandom,
)]
pub struct BlsToExecutionChange {
    #[serde(with = "serde_utils::quoted_u64")]
    pub validator_index: u64,
    pub from_bls_pubkey: PublicKeyBytes,
    pub to_execution_address: Address,
}

impl SignedRoot for BlsToExecutionChange {}

impl BlsToExecutionChange {
    pub fn sign(
        self,
        secret_key: &SecretKey,
        genesis_validators_root: Hash256,
        spec: &ChainSpec,
    ) -> SignedBlsToExecutionChange {
        let domain = spec.compute_domain(
            Domain::BlsToExecutionChange,
            spec.genesis_fork_version,
            genesis_validators_root,
        );
        let message = self.signing_root(domain);
        SignedBlsToExecutionChange {
            message: self,
            signature: secret_key.sign(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    ssz_and_tree_hash_tests!(BlsToExecutionChange);
}
