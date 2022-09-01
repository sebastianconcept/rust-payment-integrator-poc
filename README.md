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
The following transaction types are currently supported.

```rust
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}
```
Additional [Design Notes Here](#design-notes).

## Run Unit Tests
The unit tests in this version use the shared transaction store and their assertions are expected to be reset before each one runs using a single thread:

    cargo test -- --test-threads=1

## Build release version
    cargo build --release

## Command line help
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

## <div id="design-notes">Design Notes</div>

- The program models the payments processing using the aid of these objects:
  - `App`. The spot to start taking input and sending it to collaborators for processing and output generation.
  - `Transaction`. The main model for the different types of operations to process. It can be instantiated from a parsed CSV record using `from_record`. 
  - `Transactions`. It's a helper object for keeping a store support dedicated to transactions. If in the future the transactions have to change its support, that can be conveniently refactored only from there.
  - `Account`. The accounts belong to the app object and they help to keep correct state of a client's account and process transactions.
- I've used TDD for this program to ensure result correctness and at the same time to help me to incrementally add functionality detecting any regression as I need to introduce changes. So far there are 9 unit tests with what I think are self-evident, unambiguous names to the most fundamental functionality.

- `Amount`, `ClientID` and `TransactionID` have dedicated types to ensure correctness and allow a change from a single point in code in case of future type migrations.

- I've found cases where specs were vague or ambiguous to decide program behavior. I've added the `RejectedTransaction` enum to handle detailed feedback and specificity when the account processes these:
  - When a withdrawal amount is greater than the accounts' available value, it will produce `Err(RejectedTransaction::InsufficientFounds)`.
  - Input is assumed to be valid but the code is defensive, so when a deposit or withdrawal transaction is found to not have an amount or it failed to parse a valid value, processing it will produce a `Err(RejectedTransaction::InvalidInput)`.
  - When transaction ids can't help to locate a transaction, processing will produce a `Err(RejectedTransaction::IDNotFound)`.
  - When the amount of a dispute is greater than the account's available value, processing it will produce `Err(RejectedTransaction::InsufficientFounds)`.
  - When a resolve transaction brings an amount that is greater than the held amount, processing will produce an `Err(RejectedTransaction::InconsistentWithValueHeld)`.

## Unit tests
Executing:

    cargo test -- --test-threads=1

Will show:
```
running 9 tests
test tests::unit::can_parse_a_deposit_command ... ok
test tests::unit::can_parse_a_withdrawal_command ... ok
test tests::unit::can_parse_input_filename_from_command_line ... ok
test tests::unit::can_read_a_record_streamed_from_a_csv_input_file ... ok
test tests::unit::chargeback_decreases_held_and_total_balances_and_locks_account ... ok
test tests::unit::deposit_can_increase_account_balance ... ok
test tests::unit::dispute_increase_disputed_balance_and_maintain_total ... ok
test tests::unit::resolve_decrease_held_balances_increase_available_and_maintain_total ... ok
test tests::unit::withdrawal_can_decrease_account_balance ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

