//! Seismic provider for HTTP requests
use crate::{
    fillers::{
        FillProvider, JoinFill, NonceFiller, RecommendedFillers, SimpleNonceManager, WalletFiller,
    },
    Identity, ProviderBuilder, RootProvider,
};
use alloy_network::{Ethereum, EthereumWallet};
use std::ops::Deref;

use crate::layers::seismic::layer::SeismicLayer;
use crate::layers::seismic::provider::SeismicProvider;

/// Seismic provider
pub type SeismicSignedProviderInner = FillProvider<
    JoinFill<Identity, NonceFiller>,
    SeismicProvider<
        FillProvider<
            JoinFill<
                <Ethereum as RecommendedFillers>::RecommendedFillers,
                WalletFiller<EthereumWallet>,
            >,
            RootProvider<alloy_transport_http::Http<alloy_transport_http::Client>, Ethereum>,
            alloy_transport_http::Http<alloy_transport_http::Client>,
            Ethereum,
        >,
        alloy_transport_http::Http<alloy_transport_http::Client>,
        Ethereum,
    >,
    alloy_transport_http::Http<alloy_transport_http::Client>,
    Ethereum,
>;

/// Seismic signed provider
#[derive(Debug, Clone)]
pub struct SeismicSignedProvider(SeismicSignedProviderInner);

impl SeismicSignedProvider {
    /// Creates a new seismic signed provider
    pub fn new(wallet: EthereumWallet, url: reqwest::Url) -> Self {
        // Create wallet layer with recommended fillers
        let wallet_layer =
            JoinFill::new(Ethereum::recommended_fillers(), WalletFiller::new(wallet.clone()));

        // Create nonce management layer
        let nonce_layer: JoinFill<Identity, NonceFiller<SimpleNonceManager>> =
            JoinFill::new(Identity, NonceFiller::default());

        // Build and return the provider
        let inner = ProviderBuilder::new()
            .network::<Ethereum>()
            .layer(nonce_layer)
            .layer(SeismicLayer {})
            .layer(wallet_layer)
            .on_http(url);
        Self(inner)
    }
}
impl Deref for SeismicSignedProvider {
    type Target = SeismicSignedProviderInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Seismic unsigned provider
pub type SeismicUnsignedProviderInner = FillProvider<
    JoinFill<Identity, NonceFiller>,
    SeismicProvider<
        FillProvider<
            JoinFill<<Ethereum as RecommendedFillers>::RecommendedFillers, Identity>,
            RootProvider<alloy_transport_http::Http<alloy_transport_http::Client>, Ethereum>,
            alloy_transport_http::Http<alloy_transport_http::Client>,
            Ethereum,
        >,
        alloy_transport_http::Http<alloy_transport_http::Client>,
        Ethereum,
    >,
    alloy_transport_http::Http<alloy_transport_http::Client>,
    Ethereum,
>;

/// Seismic unsigned provider
#[derive(Debug, Clone)]
pub struct SeismicUnsignedProvider(SeismicUnsignedProviderInner);

impl SeismicUnsignedProvider {
    /// Creates a new seismic unsigned provider
    pub fn new(url: reqwest::Url) -> Self {
        // Create wallet layer with recommended fillers
        let wallet_layer = JoinFill::new(Ethereum::recommended_fillers(), Identity);
        let nonce_layer: JoinFill<Identity, NonceFiller<SimpleNonceManager>> =
            JoinFill::new(Identity, NonceFiller::default());

        let inner = ProviderBuilder::new()
            .network::<Ethereum>()
            .layer(nonce_layer)
            .layer(SeismicLayer {})
            .layer(wallet_layer)
            .on_http(url);
        Self(inner)
    }
}

impl Deref for SeismicUnsignedProvider {
    type Target = SeismicUnsignedProviderInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
/// Utilities for testing seismic provider
pub mod test_utils {
    use alloy_primitives::{hex, Bytes};

    /// Test context for seismic provider
    #[derive(Debug)]
    pub struct ContractTestContext;
    impl ContractTestContext {
        // ==================== first block for encrypted transaction ====================
        // Contract deployed
        //     pragma solidity ^0.8.13;
        // contract SeismicCounter {
        //     suint256 number;
        //     constructor() payable {
        //         number = 0;
        //     }
        //     function setNumber(suint256 newNumber) public {
        //         number = newNumber;
        //     }
        //     function increment() public {
        //         number++;
        //     }
        //     function isOdd() public view returns (bool) {
        //         return number % 2 == 1;
        //     }
        // }
        /// Get the is odd input plaintext
        pub fn get_is_odd_input_plaintext() -> Bytes {
            Bytes::from_static(&hex!("43bd0d70"))
        }

        /// Get the set number input plaintext
        pub fn get_set_number_input_plaintext() -> Bytes {
            Bytes::from_static(&hex!(
                "24a7f0b70000000000000000000000000000000000000000000000000000000000000003"
            ))
        }

        /// Get the increment input plaintext
        pub fn get_increment_input_plaintext() -> Bytes {
            Bytes::from_static(&hex!("d09de08a"))
        }

        /// Get the deploy input plaintext
        pub fn get_deploy_input_plaintext() -> Bytes {
            Bytes::from_static(&hex!("60806040525f5f8190b150610285806100175f395ff3fe608060405234801561000f575f5ffd5b506004361061003f575f3560e01c806324a7f0b71461004357806343bd0d701461005f578063d09de08a1461007d575b5f5ffd5b61005d600480360381019061005891906100f6565b610087565b005b610067610090565b604051610074919061013b565b60405180910390f35b6100856100a7565b005b805f8190b15050565b5f600160025fb06100a19190610181565b14905090565b5f5f81b0809291906100b8906101de565b919050b150565b5f5ffd5b5f819050919050565b6100d5816100c3565b81146100df575f5ffd5b50565b5f813590506100f0816100cc565b92915050565b5f6020828403121561010b5761010a6100bf565b5b5f610118848285016100e2565b91505092915050565b5f8115159050919050565b61013581610121565b82525050565b5f60208201905061014e5f83018461012c565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f61018b826100c3565b9150610196836100c3565b9250826101a6576101a5610154565b5b828206905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6101e8826100c3565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361021a576102196101b1565b5b60018201905091905056fea2646970667358221220ea421d58b6748a9089335034d76eb2f01bceafe3dfac2e57d9d2e766852904df64736f6c63782c302e382e32382d646576656c6f702e323032342e31322e392b636f6d6d69742e39383863313261662e6d6f64005d"))
        }

        /// Results from solc compilation
        pub fn get_code() -> Bytes {
            Bytes::from_static(&hex!("608060405234801561000f575f5ffd5b506004361061003f575f3560e01c806324a7f0b71461004357806343bd0d701461005f578063d09de08a1461007d575b5f5ffd5b61005d600480360381019061005891906100f6565b610087565b005b610067610090565b604051610074919061013b565b60405180910390f35b6100856100a7565b005b805f8190b15050565b5f600160025fb06100a19190610181565b14905090565b5f5f81b0809291906100b8906101de565b919050b150565b5f5ffd5b5f819050919050565b6100d5816100c3565b81146100df575f5ffd5b50565b5f813590506100f0816100cc565b92915050565b5f6020828403121561010b5761010a6100bf565b5b5f610118848285016100e2565b91505092915050565b5f8115159050919050565b61013581610121565b82525050565b5f60208201905061014e5f83018461012c565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f61018b826100c3565b9150610196836100c3565b9250826101a6576101a5610154565b5b828206905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6101e8826100c3565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361021a576102196101b1565b5b60018201905091905056fea2646970667358221220ea421d58b6748a9089335034d76eb2f01bceafe3dfac2e57d9d2e766852904df64736f6c63782c302e382e32382d646576656c6f702e323032342e31322e392b636f6d6d69742e39383863313261662e6d6f64005d"))
        }
    }
}


#[cfg(test)]
mod tests {
    use alloy_network::{EthereumWallet, TransactionBuilder};
    use alloy_node_bindings::{Anvil, AnvilInstance};
    use alloy_primitives::{Address, Bytes, TxKind};
    use alloy_rpc_types_eth::{TransactionInput, TransactionRequest};
    use alloy_signer_local::PrivateKeySigner;

    use crate::{Provider, SeismicSignedProvider, SeismicUnsignedProvider};
    use crate::{layers::seismic::http::test_utils::ContractTestContext, SendableTx};

    #[tokio::test]
    async fn test_seismic_signed_call() {
        let plaintext = ContractTestContext::get_deploy_input_plaintext();
        let anvil = Anvil::new().spawn();
        let wallet = get_wallet(&anvil);
        let provider = SeismicSignedProvider::new(wallet.clone(), anvil.endpoint_url());

        let from = wallet.default_signer().address();
        let tx = TransactionRequest::default()
            .with_input(plaintext)
            .with_kind(TxKind::Create)
            .with_from(from);

        let res = provider.seismic_call(SendableTx::Builder(tx)).await.unwrap();

        assert_eq!(res, ContractTestContext::get_code());
    }

    #[tokio::test]
    async fn test_seismic_unsigned_call() {
        let plaintext = ContractTestContext::get_deploy_input_plaintext();
        let anvil = Anvil::new().spawn();

        let unsigned_provider = SeismicUnsignedProvider::new(anvil.endpoint_url());

        let tx = TransactionRequest::default()
            .with_input(plaintext)
            .with_from(Address::ZERO)
            .with_kind(TxKind::Create);

        let res = unsigned_provider.seismic_call(SendableTx::Builder(tx)).await.unwrap();
        assert_eq!(res, ContractTestContext::get_code());
    }

    #[tokio::test]
    async fn test_send_transaction() {
        let plaintext = ContractTestContext::get_deploy_input_plaintext();
        let anvil = Anvil::new().spawn();
        let wallet = get_wallet(&anvil);
        let provider = SeismicSignedProvider::new(wallet.clone(), anvil.endpoint_url());
        let from = wallet.default_signer().address();

        // testing send transaction
        let tx = TransactionRequest {
            input: TransactionInput { input: Some(plaintext), data: None },
            from: Some(from),
            to: Some(TxKind::Create),
            ..Default::default()
        };

        let contract_address = provider
            .send_transaction(tx)
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap()
            .contract_address
            .unwrap();

        let code = provider.get_code_at(contract_address).await.unwrap();
        assert_eq!(code, ContractTestContext::get_code());
    }

    #[tokio::test]
    async fn test_send_transaction_with_emtpy_input() {
        let plaintext = Bytes::new();
        let anvil = Anvil::new().spawn();
        let wallet = get_wallet(&anvil);
        let provider = SeismicSignedProvider::new(wallet.clone(), anvil.endpoint_url());
        let from = wallet.default_signer().address();

        let tx = TransactionRequest::default()
            .with_input(plaintext)
            .with_from(from)
            .with_to(Address::ZERO);

        let res = provider.send_transaction(tx).await.unwrap();
        let receipt = res.get_receipt().await.unwrap();
        assert_eq!(receipt.status(), true);
    }

    fn get_wallet(anvil: &AnvilInstance) -> EthereumWallet {
        let bob: PrivateKeySigner = anvil.keys()[1].clone().into();
        let wallet = EthereumWallet::from(bob.clone());
        wallet
    }

}
