
//! Seismic middlware for encrypting transactions and decrypting responses
use crate::{Provider, ProviderLayer};
use alloy_network::Network;
use alloy_transport::Transport;

use crate::layers::seismic::provider::SeismicProvider;

/// Seismic layer
#[derive(Debug, Clone)]
pub (crate) struct SeismicLayer {}

impl<P, T, N> ProviderLayer<P, T, N> for SeismicLayer
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    type Provider = SeismicProvider<P, T, N>;

    fn layer(&self, inner: P) -> Self::Provider {
        SeismicProvider::new(inner)
    }
}
