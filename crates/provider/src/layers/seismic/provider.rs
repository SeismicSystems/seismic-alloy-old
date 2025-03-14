//! Seismic provider for encrypting transactions and decrypting responses
use crate::{
    PendingTransactionBuilder, Provider,  RootProvider,
    SendableTx,
};
use alloy_consensus::transaction::TxSeismicElements;
use alloy_network::{Network, TransactionBuilder};
use alloy_primitives::Bytes;
use alloy_transport::{Transport, TransportErrorKind, TransportResult};
use std::marker::PhantomData;

/// Seismic middlware for encrypting transactions and decrypting responses
#[derive(Debug, Clone)]
pub struct SeismicProvider<P, T, N> {
    /// Inner provider.
    inner: P,
    /// Phantom data
    _pd: PhantomData<(T, N)>,
}

impl<P, T, N> SeismicProvider<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    /// Create a new seismic provider
    pub (crate) fn new(inner: P) -> Self {
        Self { inner, _pd: PhantomData }
    }

    /// Should encrypt input
    pub (crate) fn should_encrypt_input<B: TransactionBuilder<N>>(&self, tx: &B) -> bool {
        tx.input().map_or(false, |input| !input.is_empty()) && tx.nonce().is_some()
    }
}

/// Implement the Provider trait for the SeismicProvider
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<P, T, N> Provider<T, N> for SeismicProvider<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    fn root(&self) -> &RootProvider<T, N> {
        self.inner.root()
    }

    async fn seismic_call(&self, mut tx: SendableTx<N>) -> TransportResult<Bytes> {
        if let Some(builder) = tx.as_mut_builder() {
            if self.should_encrypt_input(builder) {
                let network_pk = self.inner.get_tee_pubkey().await.map_err(|e| {
                    TransportErrorKind::custom_str(&format!(
                        "Error getting tee pubkey from server: {:?}",
                        e
                    ))
                })?;
                let encryption_keypair = TxSeismicElements::get_rand_encryption_keypair();
                let seismic_elements = TxSeismicElements::default()
                    .with_encryption_pubkey(encryption_keypair.public_key())
                    .with_encryption_nonce(TxSeismicElements::get_rand_encryption_nonce());

                // Encrypt using recipient's public key and generated private key
                let plaintext_input = builder.input().unwrap();
                let encrypted_input = seismic_elements
                    .client_encrypt(&plaintext_input, &network_pk, &encryption_keypair.secret_key())
                    .map_err(|e| {
                        TransportErrorKind::custom_str(&format!("Error encrypting input: {:?}", e))
                    })?;

                builder.set_input(Bytes::from(encrypted_input));
                builder.set_seismic_elements(seismic_elements);

                // decrypting output
                return self
                    .inner
                    .seismic_call(SendableTx::Builder(builder.clone()))
                    .await
                    .and_then(|encrypted_output| {
                        // Decrypt the output using the encryption keypair
                        let decrypted_output = seismic_elements
                            .client_decrypt(
                                &encrypted_output,
                                &network_pk,
                                &encryption_keypair.secret_key(),
                            )
                            .map_err(|e| {
                                TransportErrorKind::custom_str(&format!(
                                    "Error decrypting output: {:?}",
                                    e
                                ))
                            })?;
                        Ok(Bytes::from(decrypted_output))
                    });
            }
        }
        let res = self.inner.seismic_call(tx).await;
        res
    }

    async fn send_transaction_internal(
        &self,
        mut tx: SendableTx<N>,
    ) -> TransportResult<PendingTransactionBuilder<T, N>> {
        if let Some(builder) = tx.as_mut_builder() {
            if self.should_encrypt_input(builder) {
                let network_pk = self.inner.get_tee_pubkey().await.map_err(|e| {
                    TransportErrorKind::custom_str(&format!(
                        "Error getting tee pubkey from server: {:?}",
                        e
                    ))
                })?;
                let encryption_keypair = TxSeismicElements::get_rand_encryption_keypair();
                let seismic_elements = TxSeismicElements::default()
                    .with_encryption_pubkey(encryption_keypair.public_key())
                    .with_encryption_nonce(TxSeismicElements::get_rand_encryption_nonce());

                // Encrypt using recipient's public key and generated private key
                let plaintext_input = builder.input().unwrap();
                let encrypted_input = seismic_elements
                    .client_encrypt(&plaintext_input, &network_pk, &encryption_keypair.secret_key())
                    .map_err(|e| {
                        TransportErrorKind::custom_str(&format!("Error encrypting input: {:?}", e))
                    })?;

                builder.set_input(Bytes::from(encrypted_input));
                builder.set_seismic_elements(seismic_elements);
            }
        }
        let res = self.inner.send_transaction_internal(tx).await;
        res
    }
}
