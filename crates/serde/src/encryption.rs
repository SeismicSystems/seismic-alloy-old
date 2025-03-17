use alloy_primitives::FixedBytes;
use seismic_enclave::{constants, PublicKey};
use serde::{de::Error, Deserialize, Deserializer};

/// Deserializes a primitive number from a "quantity" hex string.
pub fn pubkey_with_prefix_deserialize<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes = FixedBytes::<{ constants::PUBLIC_KEY_SIZE }>::deserialize(deserializer)?;
    println!("pubkey bytes: {:?}", bytes);
    PublicKey::from_slice(bytes.as_slice()).map_err(D::Error::custom)
}
