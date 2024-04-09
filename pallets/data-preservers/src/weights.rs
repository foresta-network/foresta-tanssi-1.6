// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>


//! Autogenerated weights for pallet_data_preservers
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-04-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `pop-os`, CPU: `12th Gen Intel(R) Core(TM) i7-1260P`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_data_preservers
// --extrinsic
// *
// --chain=dev
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// tmp/pallet_data_preservers.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_data_preservers.
pub trait WeightInfo {
	fn set_boot_nodes(x: u32, y: u32, ) -> Weight;
	fn force_assignment() -> Weight;
}

/// Weights for pallet_data_preservers using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:0)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::BootNodes` (r:0 w:1)
	/// Proof: `DataPreservers::BootNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn set_boot_nodes(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195`
		//  Estimated: `3660`
		// Minimum execution time: 10_457_000 picoseconds.
		Weight::from_parts(10_366_234, 3660)
			// Standard Error: 787
			.saturating_add(Weight::from_parts(5_398, 0).saturating_mul(x.into()))
			// Standard Error: 16_425
			.saturating_add(Weight::from_parts(354_995, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `DataPreservers::Assignments` (r:0 w:1)
	/// Proof: `DataPreservers::Assignments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_assignment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_398_000 picoseconds.
		Weight::from_parts(5_683_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:0)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::BootNodes` (r:0 w:1)
	/// Proof: `DataPreservers::BootNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn set_boot_nodes(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195`
		//  Estimated: `3660`
		// Minimum execution time: 10_457_000 picoseconds.
		Weight::from_parts(10_366_234, 3660)
			// Standard Error: 787
			.saturating_add(Weight::from_parts(5_398, 0).saturating_mul(x.into()))
			// Standard Error: 16_425
			.saturating_add(Weight::from_parts(354_995, 0).saturating_mul(y.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `DataPreservers::Assignments` (r:0 w:1)
	/// Proof: `DataPreservers::Assignments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_assignment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_398_000 picoseconds.
		Weight::from_parts(5_683_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
