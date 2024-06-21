#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::storage::{with_transaction, TransactionOutcome};
use sp_runtime::DispatchError;
use sp_std::result::Result;

pub mod offchain_worker;
pub mod ordered_set;

pub use offchain_worker::OffchainErr;
pub use ordered_set::OrderedSet;

/// Execute the supplied function in a new storage transaction.
///
/// All changes to storage performed by the supplied function are discarded if
/// the returned outcome is `Result::Err`.
///
/// Transactions can be nested to any depth. Commits happen to the parent
/// transaction.
pub fn with_transaction_result<R>(f: impl FnOnce() -> Result<R, DispatchError>) -> Result<R, DispatchError> {
	with_transaction(|| {
		let res = f();
		if res.is_ok() {
			TransactionOutcome::Commit(res)
		} else {
			TransactionOutcome::Rollback(res)
		}
	})
}

/// Simulate execution of the supplied function in a new storage transaction.
/// Changes to storage performed by the supplied function are always discarded.
pub fn simulate_execution<R>(f: impl FnOnce() -> Result<R, DispatchError>) -> Result<R, DispatchError> {
	with_transaction(|| {
		let res = f();
		TransactionOutcome::Rollback(res)
	})
}

