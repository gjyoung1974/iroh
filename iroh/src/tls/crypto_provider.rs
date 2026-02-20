//! Centralized crypto provider selection based on feature flags.
//!
//! This module provides a single point of configuration for the TLS crypto backend.
//! Enable exactly one of `crypto-ring` or `crypto-aws-lc-rs` features.

#[cfg(all(feature = "crypto-ring", feature = "crypto-aws-lc-rs"))]
compile_error!("Features `crypto-ring` and `crypto-aws-lc-rs` are mutually exclusive");

#[cfg(not(any(feature = "crypto-ring", feature = "crypto-aws-lc-rs")))]
compile_error!("Either feature `crypto-ring` or `crypto-aws-lc-rs` must be enabled");

use std::sync::Arc;

use rustls::crypto::CryptoProvider;

pub(crate) fn default_provider() -> CryptoProvider {
    #[cfg(feature = "crypto-ring")]
    {
        rustls::crypto::ring::default_provider()
    }
    #[cfg(feature = "crypto-aws-lc-rs")]
    {
        rustls::crypto::aws_lc_rs::default_provider()
    }
}

pub(crate) fn any_eddsa_type(
    der: &webpki_types::PrivatePkcs8KeyDer<'_>,
) -> Result<Arc<dyn rustls::sign::SigningKey>, rustls::Error> {
    #[cfg(feature = "crypto-ring")]
    {
        rustls::crypto::ring::sign::any_eddsa_type(der)
    }
    #[cfg(feature = "crypto-aws-lc-rs")]
    {
        rustls::crypto::aws_lc_rs::sign::any_eddsa_type(der)
    }
}

/// Re-export the appropriate webpki algorithm set for TLS signature verification.
#[cfg(feature = "crypto-ring")]
pub(crate) use webpki::ring as webpki_algs;
#[cfg(feature = "crypto-aws-lc-rs")]
pub(crate) use webpki::aws_lc_rs as webpki_algs;
