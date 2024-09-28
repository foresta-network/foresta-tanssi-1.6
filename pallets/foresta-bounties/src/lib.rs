#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

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

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Bounty<T:Config> {
		pub currency_id: CurrencyIdOf<T>,
        pub value: CurrencyBalanceOf<T>,
		pub project_id: ProjectIdOf<T>,
		pub metadata: BoundedVec<u8,T::MaxBountyDescription>,
		pub status: BountyStatus,
		pub recipient: Option<T::AccountId>,
		pub end: Option<BlockNumberFor<T>>,
		pub unlock: Option<BlockNumberFor<T>>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug,MaxEncodedLen, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	pub enum BountyStatus {
		InActive,
		Proposed,
		Active,
		Cancelled,
		Awarded,
		Submitted,
		Fulfilled,
	}


	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_carbon_credits::Config + pallet_dex::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
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
		type MaxBountySubmission: Get<u32>;
		type MaxConcurrentPayouts: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn get_unlock_duration)]
	pub type UnlockDuration<T> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

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
	#[pallet::getter(fn get_bounties_count)]
	pub type BountiesCount<T: Config> = StorageValue<_, BountyId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_pending_payouts)]
	pub type PendingPayouts<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BlockNumberFor<T>,
		BoundedVec<BountyId, T::MaxConcurrentPayouts>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_submission_hash)]
	pub type Submissions<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BountyId,
		BoundedVec<u8, T::MaxBountySubmission>,
		ValueQuery,
	>;

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		BountyCreated {
			uid: BountyId,
			currency_id: CurrencyIdOf<T>,
			amount: CurrencyBalanceOf<T>,
		},
		BountyActivated {
			bounty_id: BountyId,
			recipient: T::AccountId,
		},
		BountyCancelled {
			bounty_id: BountyId,
		},
		BountySubmitted {
			bounty_id: BountyId,
			who: T::AccountId,
		},
		BountyAwarded {
			bounty_id: BountyId,
			unlock: BlockNumberFor<T>,
		},

	}

	#[pallet::error]
	pub enum Error<T> {
		/// Insufficient Treasury Balance
		InsufficientTreasuryBalance,
		/// Project Not Found
		ProjectNotFound,
		/// Not The Project Admin
		NotTheProjectAdmin,
		/// Bounty Does Not Exist
		BountyDoesNotExist,
		/// Bounty Cannot Be Awarded
		BountyCannotBeAwarded,
		/// Inactive Bounty
		InActiveBounty,
		/// Max Concurrent Payouts Exceeded
		MaxConcurrentPayoutsExceeded,
		/// NotTheRecipient
		NotTheRecipient,
		/// Bounty Expired
		BountyExpired,
		/// Bounty Not Awarded
		BountyNotAwarded,
		/// Bounty Not Submitted
		BountyNotSubmitted,
		/// BountyCannotBeCancelled
		BountyCannotBeCancelled,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: frame_system::pallet_prelude::BlockNumberFor<T>) -> Weight {
			let mut weight = T::DbWeight::get().reads_writes(1, 1);

			let approval = PendingPayouts::<T>::take(n);

			for v_id in approval.iter() {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				let _ = Self::do_make_payment(*v_id);

			}

			weight
		}
	}

	
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::force_set_unlock_duration())]
		pub fn create_bounty(origin: OriginFor<T>, project_id: ProjectIdOf<T>,
		currency_id: CurrencyIdOf<T>, amount: CurrencyBalanceOf<T>, 
		metadata: BoundedVec<u8,T::MaxBountyDescription>) -> DispatchResult {

			<T as pallet::Config>::ForceOrigin::ensure_origin(origin)?;// Remove
			ensure!(Self::check_treasury_pot(project_id,currency_id,amount),Error::<T>::InsufficientTreasuryBalance);

			let bounty = Bounty::<T> {
				currency_id: currency_id,
        	    value: amount,
				project_id: project_id,
				metadata: metadata,
				status: BountyStatus::Proposed,
				recipient: None,
				end: None,
				unlock: None,
			};

			let uid = Self::get_bounties_count();
			let uid2 = uid.checked_add(1u32.into()).ok_or(ArithmeticError::Overflow)?;

			Bounties::<T>::insert(uid,&bounty);
			BountiesCount::<T>::put(uid2);
			Self::deposit_event(Event::BountyCreated{ uid, currency_id, amount });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::force_set_unlock_duration())]
		pub fn activate_bounty(origin: OriginFor<T>, bounty_id: BountyId,
		recipient: T::AccountId, duration: BlockNumberFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut bounty = Bounties::<T>::get(bounty_id).ok_or(Error::<T>::BountyDoesNotExist).unwrap();

			ensure!(Self::check_project_admin(who.clone(),bounty.project_id),Error::<T>::NotTheProjectAdmin);
			
			ensure!(bounty.status == BountyStatus::Proposed,Error::<T>::BountyCannotBeAwarded);
			
			let now = frame_system::Pallet::<T>::block_number();
			bounty.end = Some(now + duration);
			bounty.recipient = Some(recipient.clone());
			bounty.status = BountyStatus::Active;

			Bounties::<T>::insert(bounty_id,&bounty);
			Self::deposit_event(Event::BountyActivated{ bounty_id, recipient });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::force_set_unlock_duration())]
		pub fn cancel_bounty(origin: OriginFor<T>, bounty_id: BountyId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut bounty = Bounties::<T>::get(bounty_id).ok_or(Error::<T>::BountyDoesNotExist).unwrap();

			ensure!(Self::check_project_admin(who.clone(),bounty.project_id),Error::<T>::NotTheProjectAdmin);
			
			ensure!(bounty.status == BountyStatus::Proposed || bounty.status == BountyStatus::Active,Error::<T>::BountyCannotBeCancelled);

			bounty.status = BountyStatus::Cancelled;

			Bounties::<T>::insert(bounty_id,&bounty);
			Self::deposit_event(Event::BountyCancelled{ bounty_id });

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::force_set_unlock_duration())]
		pub fn submit_bounty(origin: OriginFor<T>, bounty_id: BountyId,
		submission_hash: BoundedVec<u8,T::MaxBountySubmission>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let now = frame_system::Pallet::<T>::block_number();
			let mut bounty = Bounties::<T>::get(bounty_id).ok_or(Error::<T>::BountyDoesNotExist).unwrap();
			ensure!(now < bounty.end.unwrap(),Error::<T>::BountyExpired);
			ensure!(bounty.status == BountyStatus::Active,Error::<T>::BountyNotAwarded);
			ensure!(bounty.recipient.clone().unwrap() == who.clone(), Error::<T>::NotTheRecipient);

			bounty.status = BountyStatus::Submitted;

			Bounties::<T>::insert(bounty_id,&bounty);
			Submissions::<T>::insert(bounty_id,submission_hash);
			Self::deposit_event(Event::BountySubmitted{ bounty_id, who });
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::force_set_unlock_duration())]
		pub fn award_bounty(origin: OriginFor<T>, bounty_id: BountyId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut bounty = Bounties::<T>::get(bounty_id).ok_or(Error::<T>::BountyDoesNotExist).unwrap();

			ensure!(Self::check_project_admin(who.clone(),bounty.project_id),Error::<T>::NotTheProjectAdmin);
			ensure!(bounty.status == BountyStatus::Submitted,Error::<T>::BountyNotSubmitted);
			
			let now = frame_system::Pallet::<T>::block_number();
			let unlock_duration = UnlockDuration::<T>::get();
			let unlock = now + unlock_duration;
			bounty.unlock = Some(unlock);
			bounty.status = BountyStatus::Awarded;

			Bounties::<T>::insert(bounty_id,&bounty);
			PendingPayouts::<T>::try_mutate(unlock, |payouts| {
				payouts.try_push(bounty_id).map_err(|_| Error::<T>::MaxConcurrentPayoutsExceeded)?;
				Ok::<(),DispatchError>(())
			})?; 

			Self::deposit_event(Event::BountyAwarded{ bounty_id, unlock });
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::force_set_unlock_duration())]
		pub fn force_set_unlock_duration(origin: OriginFor<T>,unlock_duration: BlockNumberFor<T>) -> DispatchResult {
			<T as pallet::Config>::ForceOrigin::ensure_origin(origin)?;

			UnlockDuration::<T>::set(unlock_duration);

			Ok(())
		}

	}

	impl<T:Config> Pallet<T> {
		pub fn check_treasury_pot(project_id: ProjectIdOf<T>,
		currency_id: CurrencyIdOf<T>, amount: CurrencyBalanceOf<T>) -> bool {
			let balance = pallet_dex::Pallet::<T>::get_pot(project_id,currency_id);
			balance > amount
		}

		pub fn check_project_admin(who: T::AccountId, project_id: ProjectIdOf<T>) -> bool {
			let project_details: Option<pallet_carbon_credits::ProjectDetail<T>> = pallet_carbon_credits::Pallet::get_project_details(project_id);
			let mut res = false;
			match project_details {
				Some(x) => res = x.originator == who,
				None => res = false,
			}
			res
		}

		pub fn do_make_payment(bounty_id: BountyId) -> DispatchResult {
			let mut bounty = Bounties::<T>::get(bounty_id).ok_or(Error::<T>::BountyDoesNotExist).unwrap();
			ensure!(bounty.status == BountyStatus::Awarded,Error::<T>::BountyNotAwarded);

			pallet_dex::Pallet::<T>::do_spend_funds(bounty.project_id, bounty.recipient.unwrap(),
				bounty.currency_id, bounty.value)?;
			Ok(())
		}
	}
}