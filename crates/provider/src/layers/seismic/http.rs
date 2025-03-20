//! Seismic provider for HTTP requests
use crate::{
    fillers::{FillProvider, JoinFill, RecommendedFillers, WalletFiller},
    Identity, ProviderBuilder, RootProvider,
};
use alloy_network::{Ethereum, EthereumWallet};
use std::ops::Deref;

use crate::layers::seismic::{layer::SeismicLayer, provider::SeismicProvider};

/// Seismic provider
pub type SeismicSignedProviderInner = SeismicProvider<
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
>;

/// Seismic signed provider
#[derive(Debug, Clone)]
pub struct SeismicSignedProvider(SeismicSignedProviderInner);

impl SeismicSignedProvider {
    /// Creates a new seismic signed provider
    pub fn new(wallet: EthereumWallet, url: reqwest::Url) -> Self {
        // Create wallet layer with recommended fillers
        let tx_filler_layer =
            JoinFill::new(Ethereum::recommended_fillers(), WalletFiller::new(wallet.clone()));

        // Build and return the provider
        let inner = ProviderBuilder::new()
            .network::<Ethereum>()
            .layer(SeismicLayer {})
            .layer(tx_filler_layer)
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
pub type SeismicUnsignedProviderInner = SeismicProvider<
    FillProvider<
        JoinFill<<Ethereum as RecommendedFillers>::RecommendedFillers, Identity>,
        RootProvider<alloy_transport_http::Http<alloy_transport_http::Client>, Ethereum>,
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
        let tx_filler_layer = JoinFill::new(Ethereum::recommended_fillers(), Identity);

        let inner = ProviderBuilder::new()
            .network::<Ethereum>()
            .layer(SeismicLayer {})
            .layer(tx_filler_layer)
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
mod tests {
    use alloy_network::{Ethereum, EthereumWallet, TransactionBuilder};
    use alloy_node_bindings::{Anvil, AnvilInstance};
    use alloy_primitives::{Address, Bytes, TxKind};
    use alloy_rpc_types_eth::TransactionRequest;
    use alloy_signer_local::PrivateKeySigner;

    use crate::{
        layers::seismic::{layer::SeismicLayer, test_utils::ContractTestContext},
        Provider, ProviderBuilder, SeismicSignedProvider, SeismicUnsignedProvider, SendableTx,
    };

    #[tokio::test]
    async fn test_seismic_signed_call() {
        let plaintext = ContractTestContext::get_deploy_input_plaintext();
        let anvil = Anvil::new().spawn();
        let wallet = get_wallet(&anvil);
        let provider = SeismicSignedProvider::new(
            wallet.clone(),
            reqwest::Url::parse("http://localhost:8545").unwrap(),
        );

        let tx = TransactionRequest::default().with_input(plaintext).with_kind(TxKind::Create);

        let res = provider.seismic_call(SendableTx::Builder(tx)).await.unwrap();

        assert_eq!(res, ContractTestContext::get_code());
    }

    #[tokio::test]
    async fn test_seismic_unsigned_call() {
        let plaintext = ContractTestContext::get_deploy_input_plaintext();
        let anvil = Anvil::new().spawn();
        let from = get_wallet(&anvil).default_signer().address();
        let unsigned_provider =
            SeismicUnsignedProvider::new(reqwest::Url::parse("http://localhost:8545").unwrap());

        let tx = TransactionRequest::default()
            .with_input(plaintext)
            .with_kind(TxKind::Create)
            .with_from(from);

        let res = unsigned_provider.seismic_call(SendableTx::Builder(tx)).await.unwrap();
        assert_eq!(res, ContractTestContext::get_code());
    }

    #[tokio::test]
    async fn test_send_transaction() {
        let plaintext = ContractTestContext::get_deploy_input_plaintext();
        let anvil = Anvil::new().spawn();
        let wallet = get_wallet(&anvil);
        let provider = SeismicSignedProvider::new(wallet.clone(), anvil.endpoint_url());

        // testing send transaction
        let tx = TransactionRequest::default().with_input(plaintext).with_kind(TxKind::Create);

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

        let tx = TransactionRequest::default().with_input(plaintext).with_to(Address::ZERO);

        let res = provider.send_transaction(tx).await.unwrap();
        let receipt = res.get_receipt().await.unwrap();
        assert_eq!(receipt.status(), true);
    }

    #[tokio::test]
    async fn test_get_tee_pubkey() {
        let provider =
            ProviderBuilder::new().network::<Ethereum>().layer(SeismicLayer {}).on_anvil();
        let tee_pubkey = provider.get_tee_pubkey().await.unwrap();
        println!("test_get_tee_pubkey: tee_pubkey: {:?}", tee_pubkey);
    }

    fn get_wallet(anvil: &AnvilInstance) -> EthereumWallet {
        let bob: PrivateKeySigner = anvil.keys()[1].clone().into();
        let wallet = EthereumWallet::from(bob.clone());
        wallet
    }
}
