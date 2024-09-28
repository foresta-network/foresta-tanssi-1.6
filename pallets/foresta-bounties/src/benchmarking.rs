//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as ForestaBounties;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_runtime::traits::BlockNumber;
use orml_traits::MultiCurrency;
use primitives::CurrencyId;

fn get_currency_id() -> CurrencyId {
	primitives::CurrencyId::USDT
}

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