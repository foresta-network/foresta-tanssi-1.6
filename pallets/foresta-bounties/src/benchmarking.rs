//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_runtime::traits::BlockNumber;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn force_set_unlock_duration() {
		let duration = 1000u32.into();
		#[extrinsic_call]
		force_set_unlock_duration(RawOrigin::Root,duration);

		assert_eq!(UnlockDuration::<T>::get(), duration);
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}