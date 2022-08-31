# integrator
Proof of Concept of a payment system in Rust for transaction processing and accounts maintenance.


![Tests](https://img.shields.io/badge/tests-9-green)
[![License](https://img.shields.io/badge/license-MIT-green)](./LICENSE.txt)

## Features

- Single asset, multiple accounts.
- Command line friendly.
- Pre-validated CVS file input.
- Ignores invalid records.
Rejects invalid transactions:
  - InvalidType,
  - InsufficientFounds,
  - IDNotFound,
  - IconsistentWithValueHeld,
  - InvalidInput,
  - TargetTransactionAmountMissing,
- Bubbles processing errors.
- Extensible transaction types.

## Supported Transaction Types

```rust
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}
```

# Run Unit Tests
The unit tests in this version use the shared transaction store and their assertions are expected to be reset before each one runs using a single thread:

    cargo test -- --test-threads=1

# Build release version
    cargo build --release

# Command line help
    ./target/release/integrator --help

```➜  integrator git:(wrap-up) ✗ ./target/release/integrator --help
integrator 1.0
Sebastian Sastre <sebastianconcept@gmail.com>
PoC payment system to demonstrate transactions processing and account maintenance using CSV files.

USAGE:
    integrator <FILENAME>

ARGS:
    <FILENAME>    Defines the CSV filename to use as input.

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```