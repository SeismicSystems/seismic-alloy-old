//! A provider layer that uses for filling sesimic transactions
use alloy_consensus::transaction::TxSeismic;
use alloy_primitives::{Address, Bytes, TxKind};
use alloy_rpc_types_eth::{TransactionInput, TransactionRequest};

pub mod layer;
pub mod provider;

#[cfg(feature = "ws")]
pub mod ws;
#[cfg(feature = "ws")]
pub use ws::SeismicUnsignedWsProvider;

#[cfg(feature = "reqwest")]
pub mod http;
#[cfg(feature = "reqwest")]
pub use http::{SeismicSignedProvider, SeismicUnsignedProvider};

/// Get a seismic transaction builder
pub fn build_seismic_tx(plaintext: Bytes, to: TxKind, from: Address) -> TransactionRequest {
    TransactionRequest {
        from: Some(from),
        to: Some(to),
        input: TransactionInput { input: Some(plaintext), data: None },
        transaction_type: Some(TxSeismic::TX_TYPE),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builder::ProviderBuilder, layers::seismic::layer::SeismicLayer, provider::Provider,
    };
    use alloy_network::Ethereum;

    #[tokio::test]
    async fn test_get_tee_pubkey() {
        let provider =
            ProviderBuilder::new().network::<Ethereum>().layer(SeismicLayer {}).on_anvil();
        let tee_pubkey = provider.get_tee_pubkey().await.unwrap();
        println!("test_get_tee_pubkey: tee_pubkey: {:?}", tee_pubkey);
    }
}
