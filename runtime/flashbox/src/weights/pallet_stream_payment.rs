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


//! Autogenerated weights for pallet_stream_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-04-12, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-1`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("flashbox_dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_stream_payment
// --extrinsic
// *
// --chain=flashbox_dev
// --steps
// 50
// --repeat
// 20
// --template=benchmarking/frame-weight-runtime-template.hbs
// --json-file
// raw.json
// --output
// tmp/flashbox_weights/pallet_stream_payment.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_stream_payment using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_stream_payment::WeightInfo for SubstrateWeight<T> {
	/// Storage: `StreamPayment::NextStreamId` (r:1 w:1)
	/// Proof: `StreamPayment::NextStreamId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `StreamPayment::LookupStreamsWithTarget` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithTarget` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `StreamPayment::LookupStreamsWithSource` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithSource` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `StreamPayment::Streams` (r:0 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn open_stream() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `3593`
		// Minimum execution time: 60_457_000 picoseconds.
		Weight::from_parts(61_394_000, 3593)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `StreamPayment::LookupStreamsWithTarget` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithTarget` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `StreamPayment::LookupStreamsWithSource` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithSource` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn close_stream() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `579`
		//  Estimated: `6196`
		// Minimum execution time: 123_881_000 picoseconds.
		Weight::from_parts(126_338_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn perform_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `579`
		//  Estimated: `6196`
		// Minimum execution time: 86_906_000 picoseconds.
		Weight::from_parts(88_817_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn request_change_immediate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `579`
		//  Estimated: `6196`
		// Minimum execution time: 125_805_000 picoseconds.
		Weight::from_parts(128_165_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn request_change_delayed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `247`
		//  Estimated: `3712`
		// Minimum execution time: 15_225_000 picoseconds.
		Weight::from_parts(15_712_000, 3712)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn accept_requested_change() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `617`
		//  Estimated: `6196`
		// Minimum execution time: 117_052_000 picoseconds.
		Weight::from_parts(118_871_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn cancel_change_request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `3750`
		// Minimum execution time: 11_259_000 picoseconds.
		Weight::from_parts(11_693_000, 3750)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn immediately_change_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `579`
		//  Estimated: `6196`
		// Minimum execution time: 116_771_000 picoseconds.
		Weight::from_parts(118_428_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}