//! Seismic provider for websocket requests
use crate::{
    fillers::{
        FillProvider, JoinFill, NonceFiller, RecommendedFillers, SimpleNonceManager, WalletFiller,
    },
    Identity, PendingTransactionBuilder, Provider, ProviderBuilder, ProviderLayer, RootProvider,
    SendableTx,
};
use alloy_consensus::{transaction::TxSeismicElements, TxSeismic};
use alloy_network::{Ethereum, EthereumWallet, Network, TransactionBuilder};
use alloy_primitives::{Address, Bytes, TxKind};
use alloy_rpc_types_eth::{TransactionInput, TransactionRequest};
use alloy_transport::{Transport, TransportErrorKind, TransportResult};
use std::{marker::PhantomData, ops::Deref};

/// Seismic unsigned websocket provider
pub type SeismicUnsignedWsProviderInner = RootProvider<alloy_transport::BoxTransport>;

/// Seismic unsigned websocket provider
#[derive(Debug, Clone)]
pub struct SeismicUnsignedWsProvider(pub SeismicUnsignedWsProviderInner);

#[cfg(feature = "ws")]
impl SeismicUnsignedWsProvider {
    /// creates a new websocket provider for a client
    pub async fn new(url: impl Into<String>) -> Result<Self, alloy_transport::TransportError> {
        let provider = ProviderBuilder::new().on_builtin(&url.into()).await?;
        Ok(Self(provider))
    }

    /// Get the inner provider
    pub fn inner(&self) -> &SeismicUnsignedWsProviderInner {
        &self.0
    }
}
