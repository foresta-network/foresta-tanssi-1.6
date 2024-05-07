// This file is part of Foresta.
// Copyright (C) 2024 Foresta.
// This code is licensed under MIT license (see LICENSE.txt for details)
//! Autogenerated weights for pallet_carbon_credits_pool
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-06-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/bitg-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --log=warn
// --pallet=pallet-carbon-credits-pools
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=./pallets/vcu-pools/src/weights.rs
// --template=./.maintain/bitg-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_carbon_credits_pool.
pub trait WeightInfo {
	fn create() -> Weight;
	fn deposit() -> Weight;
	fn retire() -> Weight;
}

/// Weights for pallet_carbon_credits_pool using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: VCUPools Pools (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:1)
	fn create() -> Weight {
		Weight::from_parts(25_000_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: VCUPools Pools (r:1 w:1)
	// Storage: VCU Projects (r:1 w:0)
	// Storage: Assets Asset (r:2 w:2)
	// Storage: Assets Account (r:3 w:3)
	// Storage: System Account (r:2 w:2)
	fn deposit() -> Weight {
		Weight::from_parts(67_000_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	// Storage: VCUPools Pools (r:1 w:1)
	// Storage: Assets Asset (r:2 w:2)
	// Storage: Assets Account (r:3 w:3)
	// Storage: VCU Projects (r:1 w:1)
	// Storage: VCU NextItemId (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Storage: Uniques Account (r:0 w:1)
	// Storage: VCU RetiredCredits (r:0 w:1)
	fn retire() -> Weight {
		Weight::from_parts(101_000_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(13_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: VCUPools Pools (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:1)
	fn create() -> Weight {
		Weight::from_parts(25_000_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: VCUPools Pools (r:1 w:1)
	// Storage: VCU Projects (r:1 w:0)
	// Storage: Assets Asset (r:2 w:2)
	// Storage: Assets Account (r:3 w:3)
	// Storage: System Account (r:2 w:2)
	fn deposit() -> Weight {
		Weight::from_parts(67_000_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	// Storage: VCUPools Pools (r:1 w:1)
	// Storage: Assets Asset (r:2 w:2)
	// Storage: Assets Account (r:3 w:3)
	// Storage: VCU Projects (r:1 w:1)
	// Storage: VCU NextItemId (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Storage: Uniques Account (r:0 w:1)
	// Storage: VCU RetiredCredits (r:0 w:1)
	fn retire() -> Weight {
		Weight::from_parts(101_000_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(11_u64))
			.saturating_add(RocksDbWeight::get().writes(13_u64))
	}
}
