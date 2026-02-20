  Summary

  Implemented runtime-pluggable CryptoProvider for the Endpoint builder and relay client. The provider flows through the entire stack:

  Public API — iroh/src/endpoint.rs:
  - New Builder::crypto_provider(Arc<CryptoProvider>) method lets callers inject their own rustls crypto backend (e.g. aws-lc-rs for FIPS)

  Internal threading path:
  Endpoint::Builder.crypto_provider(provider)
    → TlsConfig (stores and uses provider for make_client_config/make_server_config)
    → socket::Options → RelayActorConfig → RelayConnectionOptions
    → relay::ClientBuilder.crypto_provider(provider)
    → MaybeTlsStreamBuilder.connect() uses provider for TLS
    → default_quic_client_config() uses provider for QUIC address discovery

  Files modified:
  - iroh/src/endpoint.rs — Builder field + method
  - iroh/src/tls.rs — TlsConfig stores and uses provider
  - iroh/src/socket.rs — Options, StaticConfig, default_quic_client_config(), test helpers
  - iroh/src/socket/transports/relay/actor.rs — Config, RelayConnectionOptions, test helpers
  - iroh-relay/src/client.rs — ClientBuilder field + method
  - iroh-relay/src/client/tls.rs — MaybeTlsStreamBuilder field + connect() uses it

  Default behavior unchanged — when no provider is set, the feature-flag default (crypto-ring or crypto-aws-lc-rs) is used. All 90 iroh tests and 9 relay tests pass.

✻ Crunched for 10m 57s

⏺ Background command "Run iroh-relay library tests" completed (exit code 0)

⏺ Already retrieved those results — all 9 iroh-relay tests passed. The implementation is complete and verified.
