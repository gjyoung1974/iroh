//! Centralized crypto provider selection based on feature flags.
//!
//! This module provides a single point of configuration for the TLS crypto backend.
//! Enable exactly one of `crypto-ring` or `crypto-aws-lc-rs` features.

#[cfg(all(feature = "crypto-ring", feature = "crypto-aws-lc-rs"))]
compile_error!("Features `crypto-ring` and `crypto-aws-lc-rs` are mutually exclusive");

#[cfg(not(any(feature = "crypto-ring", feature = "crypto-aws-lc-rs")))]
compile_error!("Either feature `crypto-ring` or `crypto-aws-lc-rs` must be enabled");

use rustls::crypto::CryptoProvider;

/// Returns the [`CryptoProvider`] for the selected crypto backend feature.
pub fn default_provider() -> CryptoProvider {
    #[cfg(feature = "crypto-ring")]
    {
        rustls::crypto::ring::default_provider()
    }
    #[cfg(feature = "crypto-aws-lc-rs")]
    {
        rustls::crypto::aws_lc_rs::default_provider()
    }
}
