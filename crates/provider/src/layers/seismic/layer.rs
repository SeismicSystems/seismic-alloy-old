//! Seismic middlware for encrypting transactions and decrypting responses
use crate::{Provider, ProviderLayer};
use alloy_network::Network;

use crate::layers::seismic::provider::SeismicProvider;

/// Seismic layer
#[derive(Debug, Clone)]
pub(crate) struct SeismicLayer {}

impl<P, N> ProviderLayer<P, N> for SeismicLayer
where
    P: Provider<N>,
    N: Network,
{
    type Provider = SeismicProvider<P, N>;

    fn layer(&self, inner: P) -> Self::Provider {
        SeismicProvider::new(inner)
    }
}
