# Build iroh workspace with aws-lc-rs crypto backend
#
# Usage:
#   make                  # build all aws-lc-rs-compatible crates
#   make build            # same as above
#   make test             # test all aws-lc-rs-compatible crates
#   make relay            # build the relay server binary
#   make bench            # build the bench binary
#   make clippy           # clippy all aws-lc-rs-compatible crates
#   make ring             # build all crates with the default ring backend
#
# Crates that support aws-lc-rs: iroh, iroh-relay, iroh-bench
# Crates hardcoded to ring:      iroh-dns-server

CRYPTO ?= aws-lc-rs

# Feature flags per crypto backend
ifeq ($(CRYPTO),aws-lc-rs)
  IROH_FEATURES      := --no-default-features --features crypto-aws-lc-rs,metrics
  RELAY_FEATURES     := --no-default-features --features crypto-aws-lc-rs,metrics,server
  BENCH_FEATURES     := --no-default-features --features crypto-aws-lc-rs,metrics
else
  IROH_FEATURES      := --features crypto-ring
  RELAY_FEATURES     := --features crypto-ring,server
  BENCH_FEATURES     := --features crypto-ring
endif

.PHONY: build test relay bench clippy check ring clean

build: build-iroh build-relay build-bench

build-iroh:
	cargo build -p iroh $(IROH_FEATURES)

build-relay:
	cargo build -p iroh-relay $(RELAY_FEATURES)

build-bench:
	cargo build -p iroh-bench $(BENCH_FEATURES)

relay: build-relay

bench: build-bench

test: test-iroh test-relay test-bench

test-iroh:
	cargo test -p iroh $(IROH_FEATURES)

test-relay:
	cargo test -p iroh-relay $(RELAY_FEATURES)

test-bench:
	cargo test -p iroh-bench $(BENCH_FEATURES)

clippy: clippy-iroh clippy-relay clippy-bench

clippy-iroh:
	cargo clippy -p iroh $(IROH_FEATURES)

clippy-relay:
	cargo clippy -p iroh-relay $(RELAY_FEATURES)

clippy-bench:
	cargo clippy -p iroh-bench $(BENCH_FEATURES)

check: check-iroh check-relay check-bench

check-iroh:
	cargo check -p iroh $(IROH_FEATURES)

check-relay:
	cargo check -p iroh-relay $(RELAY_FEATURES)

check-bench:
	cargo check -p iroh-bench $(BENCH_FEATURES)

# Convenience: build everything with ring (includes bench + dns-server)
ring:
	cargo build --workspace

clean:
	cargo clean
