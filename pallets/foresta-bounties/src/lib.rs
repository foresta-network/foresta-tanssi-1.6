//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single `u32` value
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! Learn more about FRAME macros [here](https://docs.substrate.io/reference/frame-macros/).
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::{BoundedVec,PalletId};
	use sp_runtime::{
		traits::{ One, MaybeSerializeDeserialize, CheckedAdd,
			 AccountIdConversion, AtLeast32BitUnsigned, Saturating}
		,ArithmeticError, };
	use frame_support::traits::{Bounded, Contains};
	use scale_info::TypeInfo;
	use codec::{FullCodec, MaxEncodedLen, EncodeLike};
	use orml_traits::MultiCurrency;

	pub type BountyId = u32;

	pub type CurrencyBalanceOf<T> =
	<<T as pallet_dex::Config>::Currency as MultiCurrency<<T as frame_system::Config>::AccountId>>::Balance;

	pub type CurrencyIdOf<T> =
	<<T as pallet_dex::Config>::Currency as MultiCurrency<<T as frame_system::Config>::AccountId>>::CurrencyId;

	pub type ProjectIdOf<T> = <T as pallet_carbon_credits::Config>::ProjectId;

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Bounty<T:Config> {
		pub currency_id: CurrencyIdOf<T>,
        pub value: CurrencyBalanceOf<T>,
		pub metadata: BoundedVec<u8,T::MaxBountyDescription>,
		pub status: BountyStatus,
		pub recipient: Option<T::AccountId>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug,MaxEncodedLen, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	pub enum BountyStatus {
		InActive,
		Proposed,
		Active,
		Fulfilled,
	}

	/// The pallet's configuration trait.
	///
	/// All our types and constants a pallet depends on must be declared here.
	/// These types are defined generically and made concrete when the pallet is declared in the
	/// `runtime/src/lib.rs` file of your chain.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_carbon_credits::Config + pallet_dex::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
        #[pallet::constant]
		type PalletId: Get<PalletId>;
		type KYCProvider: Contains<Self::AccountId>;
		type CurrencyBalance: Member
			+ Parameter
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo
			+ From<u128>;
		type Currency: MultiCurrency<Self::AccountId, Balance = <Self as pallet::Config>::CurrencyBalance>;
		type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		type MaxBountyDescription: Get<u32>;
	}


	
	#[pallet::storage]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn get_bounty)]
	pub(super) type Bounties<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BountyId,
		Bounty<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn bounties_count)]
	pub type BountiesCount<T: Config> = StorageValue<_, BountyId, ValueQuery>;

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A user has successfully set a new value.
		SomethingStored {
			/// The new value set.
			something: u32,
			/// The account who set the new value.
			who: T::AccountId,
		},

		BountyCreated {
			uid: BountyId,
			currency_id: CurrencyIdOf<T>,
			amount: CurrencyBalanceOf<T>,
		}
	}

		#[pallet::error]
	pub enum Error<T> {
		/// The value retrieved was `None` as no value was previously set.
		NoneValue,
		/// There was an attempt to increment the value in storage over `u32::MAX`.
		StorageOverflow,
		/// Insufficient Treasury Balance
		InsufficientTreasuryBalance,
	}

	
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::do_something())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::<T>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });

			// Return a successful `DispatchResult`
			Ok(())
		}

	
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::<T>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage. This will cause an error in the event
					// of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::<T>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::call_index(2)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::do_something())]
		pub fn create_bounty(origin: OriginFor<T>, project_id: ProjectIdOf<T>,
		currency_id: CurrencyIdOf<T>, amount: CurrencyBalanceOf<T>, 
		metadata: BoundedVec<u8,T::MaxBountyDescription>) -> DispatchResult {

			<T as pallet::Config>::ForceOrigin::ensure_origin(origin)?;// Remove
			ensure!(Self::check_treasury_pot(project_id,currency_id,amount),Error::<T>::InsufficientTreasuryBalance);

			let bounty = Bounty::<T> {
				currency_id: currency_id,
        	    value: amount,
				metadata: metadata,
				status: BountyStatus::Proposed,
				recipient: None,
			};

			let uid = Self::bounties_count();
			let uid2 = uid.checked_add(1u32.into()).ok_or(ArithmeticError::Overflow)?;

			Bounties::<T>::insert(uid,&bounty);
			BountiesCount::<T>::put(uid2);
			Self::deposit_event(Event::BountyCreated{ uid, currency_id, amount });
			Ok(())
		}
	}

	impl<T:Config> Pallet<T> {
		pub fn check_treasury_pot(project_id: ProjectIdOf<T>,
		currency_id: CurrencyIdOf<T>, amount: CurrencyBalanceOf<T>) -> bool {
			let balance = pallet_dex::Pallet::<T>::get_pot(project_id,currency_id);
			balance > amount
		}
	}
}