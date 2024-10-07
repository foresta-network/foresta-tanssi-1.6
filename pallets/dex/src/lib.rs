// This file is part of Foresta.
// Copyright (C) 2024 Foresta.
// This code is licensed under MIT license (see LICENSE.txt for details)
//
//! Foresta DEX Pallet
//! The DEX pallet allows permissionless listing and buying of carbon credits. The pallet currently
//! only supports fixed price purchase of carbon credits from a listing. A user can create a listing
//! with the amount of Carbon credits for sale and the price expected for each unit, this sale order
//! remains onchain until cancelled by the user or completely filled. While the listing is active,
//! any user can call buy_order specifying the number of Carbon credits to purchase, the amount from
//! the buyer is transferred to the seller and any fees applicable to the pallet account.
//!
//! ## Interface
//!
//! ### Permissionless Functions
//!
//! * `create_sell_order`: Creates a new sell order onchain
//! * `cancel_sell_order`: Cancel an existing sell order
//! * `buy_order`: Purchase units from exising sell order
//!
//! ### Permissioned Functions
//!
//! * `force_set_purchase_fee` : Set the purchase fee percentage for the dex
//! * `force_set_payment_fee` : Set the payment fee percentage for the dex
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity, clippy::too_many_arguments)]
use codec::{Decode, Encode, MaxEncodedLen};
use sp_runtime::RuntimeDebug;

pub use pallet::*;
use scale_info::TypeInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
pub use weights::WeightInfo;
mod types;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use crate::{types::*, WeightInfo};
	use frame_support::{
		pallet_prelude::*,
		traits::{fungibles::{Mutate, Create}, tokens::Preservation::Expendable, Contains},
		PalletId,
	};
	use frame_system::pallet_prelude::{OriginFor, *};
	use orml_traits::MultiCurrency;
	use sp_runtime::{
		traits::{AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedSub, One, Zero},
		Percent, Saturating,
	};
	use sp_std::fmt::Debug;

	#[pallet::pallet]

	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_carbon_credits::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The units in which we record currency balance.
		type CurrencyBalance: Member
			+ Parameter
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo
			+ From<u128>;

		// Asset manager config
		type Asset: Create<Self::AccountId, AssetId = Self::AssetId, Balance = Self::Balance> + 
			Mutate<Self::AccountId, Balance = Self::Balance>;

		// Token handler config - this is what the pallet accepts as payment
		type Currency: MultiCurrency<Self::AccountId, Balance = Self::CurrencyBalance>;

		/// The origin which may forcibly set storage or add authorised accounts
		type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// The minimum units of asset to create a sell order
		#[pallet::constant]
		type MinUnitsToCreateSellOrder: Get<Self::Balance>;

		/// The minimum price per unit of asset to create a sell order
		#[pallet::constant]
		type MinPricePerUnit: Get<CurrencyBalanceOf<Self>>;

		/// The maximum payment fee that can be set
		#[pallet::constant]
		type MaxPaymentFee: Get<Percent>;

		/// The maximum Royalty that can be set
		#[pallet::constant]
		type MaxRoyalty: Get<Percent>;

		/// The maximum purchase fee that can be set
		#[pallet::constant]
		type MaxPurchaseFee: Get<CurrencyBalanceOf<Self>>;

		/// The DEX pallet id
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// The maximum validators for a payment
		type MaxValidators: Get<u32> + TypeInfo + Clone;

		/// The maximum length of tx hash that can be stored on chain
		type MaxTxHashLen: Get<u32> + TypeInfo + Clone + Debug + PartialEq;

		/// The maximum length of address that can be stored on chain
		type MaxAddressLen: Get<u32> + TypeInfo + Clone + Debug + PartialEq;

		/// The maximum length of orderIds that can be stored on chain
		type MaxOrderIds: Get<u32> + TypeInfo + Clone + Debug + PartialEq;

		/// The maximum payouts to store onchain
		type MaxPayoutsToStore: Get<u32> + TypeInfo + Clone + Debug + PartialEq;

		/// The maximum open orders allowed for a user
		type MaxOpenOrdersPerUser: Get<u32> + TypeInfo + Clone + Debug + PartialEq;

		type MaxMembersPerCollective: Get<u32>;

		/// KYC provider config
		type KYCProvider: Contains<Self::AccountId>;

		/// The expiry time for buy order
		type BuyOrderExpiryTime: Get<BlockNumberFor<Self>>;
	}

	// orders information
	#[pallet::storage]
	#[pallet::getter(fn order_count)]
	pub type OrderCount<T: Config> = StorageValue<_, OrderId, ValueQuery>;

	// Payment fees charged by dex
	#[pallet::storage]
	#[pallet::getter(fn payment_fees)]
	pub type PaymentFees<T: Config> = StorageValue<_, Percent, ValueQuery>;

	// purchase fees charged by dex
	#[pallet::storage]
	#[pallet::getter(fn purchase_fees)]
	pub type PurchaseFees<T: Config> = StorageValue<_, CurrencyBalanceOf<T>, ValueQuery>;

	// Payment fees charged by dex
	#[pallet::storage]
	#[pallet::getter(fn royalty_percentage)]
	pub type RoyaltyPercent<T: Config> = StorageValue<_, Percent, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn order_info)]
	pub type Orders<T: Config> = StorageMap<_, Blake2_128Concat, OrderId, OrderInfoOf<T>>;

	// Seller receivables from sales
	
	#[pallet::storage]
	#[pallet::getter(fn seller_receivables)]
	pub type SellerReceivables<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId, 
		Blake2_128Concat,
		CurrencyIdOf<T>,
		CurrencyBalanceOf<T>,
		ValueQuery,
	>;

	// Project Treasury	
	#[pallet::storage]
	#[pallet::getter(fn get_pot)]
	pub type Treasury<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		ProjectIdOf<T>,
		Blake2_128Concat,
		CurrencyIdOf<T>,
		CurrencyBalanceOf<T>,
		ValueQuery,
	>;



	/// storage to track the limit of units allowed in open orders
	#[pallet::storage]
	#[pallet::getter(fn user_open_order_units_allowed)]
	pub type UserOpenOrderUnitsAllowed<T: Config> =
		StorageMap<_, Blake2_128Concat, UserLevel, T::Balance>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new sell order has been created
		SellOrderCreated {
			order_id: OrderId,
			asset_id: T::AssetId,
			project_id: ProjectIdOf<T>,
			group_id: GroupIdOf<T>,
			units: T::Balance,
			currency_id: CurrencyIdOf<T>,
			price_per_unit: CurrencyBalanceOf<T>,
			owner: T::AccountId,
		},
		/// A sell order was cancelled
		SellOrderCancelled { order_id: OrderId, seller: T::AccountId },
		
		/// User open order units limit set
		UserOpenOrderUnitsLimitUpdated { level: UserLevel, limit: T::Balance },
		/// BuyOrdersByUser storage was cleard
		BuyOrdersByUserCleared { user: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error when calculating orderId
		OrderIdOverflow,
		/// The orderId does not exist
		InvalidOrderId,
		/// Only the order owner can perform this call
		InvalidOrderOwner,
		/// The expected asset_id does not match the order
		InvalidAssetId,
		/// Error when calculating order units
		OrderUnitsOverflow,
		/// The amount does not cover fees + transaction
		InsufficientCurrency,
		/// Below minimum price
		BelowMinimumPrice,
		/// Below minimum units
		BelowMinimumUnits,
		/// Arithmetic overflow
		ArithmeticError,
		/// Asset not permitted to be listed
		AssetNotPermitted,
		/// Seller and buyer cannot be same
		SellerAndBuyerCannotBeSame,
		/// Cannot set more than the maximum payment fee
		CannotSetMoreThanMaxPaymentFee,
		/// The fee amount exceeds the limit set by user
		FeeExceedsUserLimit,
		/// The purchasea fee amount exceeds the limit
		CannotSetMoreThanMaxPurchaseFee,
		/// not authorized to perform action
		NotAuthorised,
		/// Duplicate validator account
		ValidatorAccountAlreadyExists,
		/// Exceeded the maximum allowed validator count
		TooManyValidatorAccounts,
		/// Different chainId provided when validating transaction
		ChainIdMismatch,
		/// TXProof provided by the validator is different from previous validation
		TxProofMismatch,
		/// User not kyc authorized to perform action
		KYCAuthorisationFailed,
		/// Already validated
		DuplicateValidation,
		/// Not seller payment authority
		NotSellerPayoutAuthority,
		/// Seller payout authority has not been set
		SellerPayoutAuthorityNotSet,
		/// No receivable found for seller
		NoReceivables,
		/// receivable amount is less than payment
		ReceivableLessThanPayment,
		/// User has too many open orders
		OpenOrderLimitExceeded,
		/// User has too many units as unpaid open orders
		UserOpenOrderUnitsAllowedExceeded,
		/// Limits for open orders not configured correctly
		UserOpenOrderUnitsLimtNotFound,
		/// Min validators cannot be zero
		MinValidatorsCannotBeZero,
		/// NotEnoughFunds
		NotEnoughFunds,
		/// InvalidPoolId
		InvalidPoolId,
		/// ProjectNotFound
		ProjectNotFound,
		/// Treasury Not Found
		TreasuryNotFound,
		/// CannotSetMoreThanMaxRoyalty
		CannotSetMoreThanMaxRoyalty,
		/// ReceivableNotFound
		ReceivableNotFound,
	}

	

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new sell order for given `asset_id`
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::create_sell_order())]
		pub fn create_sell_order(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			units: T::Balance,
			currency_id: CurrencyIdOf<T>,
			price_per_unit: CurrencyBalanceOf<T>,
		) -> DispatchResult {
			let seller = ensure_signed(origin.clone())?;
			Self::check_kyc_approval(&seller)?;
			// ensure the asset_id can be listed
			let (project_id, group_id) = pallet_carbon_credits::Pallet::<T>::asset_id_lookup(asset_id)
				.ok_or(Error::<T>::AssetNotPermitted)?;

			// ensure minimums are satisfied
			ensure!(units >= T::MinUnitsToCreateSellOrder::get(), Error::<T>::BelowMinimumUnits);
			ensure!(price_per_unit >= T::MinPricePerUnit::get(), Error::<T>::BelowMinimumPrice);

			// transfer assets from seller to pallet
			T::Asset::transfer(asset_id.clone(), &seller, &Self::account_id(), units, Expendable)?;

			let order_id = Self::order_count();
			let next_order_id =
				order_id.checked_add(One::one()).ok_or(Error::<T>::OrderIdOverflow)?;
			OrderCount::<T>::put(next_order_id);

			// order values
			Orders::<T>::insert(
				order_id,
				OrderInfo {
					owner: seller.clone(),
					units,
					price_per_unit,
					asset_id: asset_id.clone(),
					currency_id: currency_id.clone(),
				},
			);

			Self::deposit_event(Event::SellOrderCreated {
				order_id,
				asset_id,
				project_id,
				group_id,
				units,
				currency_id,
				price_per_unit,
				owner: seller,
			});

			Ok(())
		}

		/// Cancel an existing sell order with `order_id`
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::cancel_sell_order())]
		pub fn cancel_sell_order(origin: OriginFor<T>, order_id: OrderId) -> DispatchResult {
			let seller = ensure_signed(origin.clone())?;

			// check validity
			let order = Orders::<T>::take(order_id).ok_or(Error::<T>::InvalidOrderId)?;

			ensure!(seller == order.owner, Error::<T>::InvalidOrderOwner);

			// transfer assets from pallet to seller
			T::Asset::transfer(
				order.asset_id,
				&Self::account_id(),
				&order.owner,
				order.units,
				Expendable,
			)?;

			Self::deposit_event(Event::SellOrderCancelled { order_id, seller });
			Ok(())
		}

		/// Force set PaymentFees value
		/// Can only be called by ForceOrigin
		#[pallet::call_index(3)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::force_set_payment_fee())]
		pub fn force_set_payment_fee(origin: OriginFor<T>, payment_fee: Percent) -> DispatchResult {
			<T as pallet::Config>::ForceOrigin::ensure_origin(origin)?;
			ensure!(
				payment_fee <= T::MaxPaymentFee::get(),
				Error::<T>::CannotSetMoreThanMaxPaymentFee
			);
			PaymentFees::<T>::set(payment_fee);
			Ok(())
		}

		/// Force set PurchaseFee value
		/// Can only be called by ForceOrigin
		#[pallet::call_index(4)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::force_set_purchase_fee())]
		pub fn force_set_purchase_fee(
			origin: OriginFor<T>,
			purchase_fee: CurrencyBalanceOf<T>,
		) -> DispatchResult {
			<T as pallet::Config>::ForceOrigin::ensure_origin(origin)?;
			ensure!(
				purchase_fee <= T::MaxPurchaseFee::get(),
				Error::<T>::CannotSetMoreThanMaxPurchaseFee
			);
			PurchaseFees::<T>::set(purchase_fee);
			Ok(())
		}

		
		#[pallet::call_index(12)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::force_set_purchase_fee())]
		pub fn force_set_open_order_allowed_limits(
			origin: OriginFor<T>,
			level: UserLevel,
			limit: T::Balance,
		) -> DispatchResult {
			<T as pallet::Config>::ForceOrigin::ensure_origin(origin)?;
			UserOpenOrderUnitsAllowed::<T>::set(level.clone(), Some(limit));
			Self::deposit_event(Event::UserOpenOrderUnitsLimitUpdated { level, limit });
			Ok(())
		}

		/// Force set Royalty value
		/// Can only be called by ForceOrigin
		#[pallet::call_index(14)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::force_set_payment_fee())]
		pub fn force_set_royalty(origin: OriginFor<T>, royalty_percent: Percent) -> DispatchResult {
			<T as pallet::Config>::ForceOrigin::ensure_origin(origin)?;
			ensure!(
				royalty_percent <= T::MaxRoyalty::get(),
				Error::<T>::CannotSetMoreThanMaxRoyalty
			);
			RoyaltyPercent::<T>::set(royalty_percent);
			Ok(())
		}

		#[pallet::call_index(15)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::buy_order())]
		pub fn trade_order(
			origin: OriginFor<T>,
			order_id: OrderId,
			asset_id: T::AssetId,
			units: T::Balance,
			max_fee: CurrencyBalanceOf<T>,
			retire: bool,
			retirement_reason: Option<sp_std::vec::Vec<u8>>,
		) -> DispatchResult {
			let buyer = ensure_signed(origin)?;
			Self::check_kyc_approval(&buyer)?;

			if units.is_zero() {
				return Ok(())
			}

			Orders::<T>::try_mutate(order_id, |maybe_order| -> DispatchResult {
				let mut order = maybe_order.take().ok_or(Error::<T>::InvalidOrderId)?;

				// ensure the expected asset matches the order
				ensure!(asset_id == order.asset_id, Error::<T>::InvalidAssetId);

				// ensure the seller and buyer are not the same
				ensure!(buyer != order.owner, Error::<T>::SellerAndBuyerCannotBeSame);

				// ensure volume remaining can cover the buy order
				ensure!(units <= order.units, Error::<T>::OrderUnitsOverflow);

				// get the projectId and groupId for events
				let (project_id, group_id) = pallet_carbon_credits::Pallet::<T>::asset_id_lookup(asset_id)
					.ok_or(Error::<T>::AssetNotPermitted)?;

				let currency_id = order.currency_id;

				// reduce the buy_order units from total volume
				order.units =
					order.units.checked_sub(&units).ok_or(Error::<T>::OrderUnitsOverflow)?;

				// calculate fees
				let units_as_u128: u128 =
					units.try_into().map_err(|_| Error::<T>::ArithmeticError)?;
				let price_per_unit_as_u128: u128 =
					order.price_per_unit.try_into().map_err(|_| Error::<T>::ArithmeticError)?;

				let required_currency = price_per_unit_as_u128
					.checked_mul(units_as_u128)
					.ok_or(Error::<T>::ArithmeticError)?;

				let payment_fee = PaymentFees::<T>::get().mul_ceil(required_currency);
				let purchase_fee: u128 =
					PurchaseFees::<T>::get().try_into().map_err(|_| Error::<T>::ArithmeticError)?;

				let total_fee =
					payment_fee.checked_add(purchase_fee).ok_or(Error::<T>::OrderUnitsOverflow)?;

		
				let total_amount = total_fee
					.checked_add(required_currency)
					.ok_or(Error::<T>::OrderUnitsOverflow)?;

				ensure!(max_fee >= total_fee.into(), Error::<T>::FeeExceedsUserLimit);

				let amount_minus_fees = total_amount.checked_sub(total_fee)
									.ok_or(Error::<T>::OrderUnitsOverflow)?;
				
				let royalty = RoyaltyPercent::<T>::get().mul_ceil(amount_minus_fees);

				// add amount record to the seller

				let current_receivables = Self::seller_receivables(order.owner.clone(),currency_id);

				let amount_to_seller = amount_minus_fees.checked_sub(royalty)
								.ok_or(Error::<T>::OrderUnitsOverflow)?;

				let new_receivables = current_receivables
									.checked_add(&amount_to_seller.into())
									.ok_or(Error::<T>::OrderUnitsOverflow)?;
				SellerReceivables::<T>::insert(order.owner.clone(),currency_id,new_receivables);

				let mut pot = Self::get_pot(project_id,currency_id);

				pot = pot.checked_add(&royalty.into()).ok_or(Error::<T>::OrderUnitsOverflow)?;

				Treasury::<T>::insert(project_id,currency_id,pot);
				/*
				Self::deposit_event(Event::BuyOrderFilled {
					order_id,
					sell_order_id: order_id,
					units: units,
					project_id: project_id.clone(),
					group_id: group_id.clone(),
					price_per_unit: order.price_per_unit,
					fees_paid: total_fee.into(),
					seller: order.owner.clone(),
					buyer: buyer.clone(),
				});
				*/
				if retire == true {
					pallet_carbon_credits::Pallet::<T>::retire_carbon_credits(
						Self::account_id().clone(),
						project_id.clone(),
						group_id.clone(),
						units,
						retirement_reason,
						None,
						None,
						None,
					)?;
				} else {
					// transfer assets from pallet to buyer
					T::Asset::transfer(asset_id.clone(),&Self::account_id(),&buyer, units, Expendable)?;
				}

				// Currency Transfer

				T::Currency::transfer(currency_id,&buyer.clone(),&Self::account_id(),total_amount.into())?;

				let units_order = order.units.clone();

				let remaining_units: u128 =
					units_order.try_into().map_err(|_| Error::<T>::ArithmeticError)?;

				if remaining_units > 0u128 {
					*maybe_order = Some(order);
				}
	
				Ok(())
			})
		}

		#[pallet::call_index(16)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::buy_order())]
		pub fn claim_tokens(
			origin: OriginFor<T>,
			currency_id: CurrencyIdOf<T>,
			amount: CurrencyBalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let current_receivables = Self::seller_receivables(who.clone(),currency_id);

			ensure!(amount <= current_receivables,Error::<T>::NotEnoughFunds);

			let new_receivables = current_receivables
						.checked_sub(&amount)
						.ok_or(Error::<T>::OrderUnitsOverflow)?;
			T::Currency::transfer(currency_id,&Self::account_id(),&who,amount)?;
			SellerReceivables::<T>::insert(who.clone(),currency_id,new_receivables);

			Ok(())
		}

	}

	impl<T: Config> Pallet<T> {
		/// The account ID of the CarbonCredits pallet
		pub fn account_id() -> T::AccountId {
			<T as pallet::Config>::PalletId::get().into_account_truncating()
		}

		/// Checks if given account is kyc approved
		pub fn check_kyc_approval(account_id: &T::AccountId) -> DispatchResult {
			if !<T as pallet::Config>::KYCProvider::contains(account_id) {
				Err(Error::<T>::KYCAuthorisationFailed.into())
			} else {
				Ok(())
			}
		}

		pub fn do_spend_funds(project_id: ProjectIdOf<T>, beneficiary: T::AccountId, currency_id: CurrencyIdOf<T>, amount: CurrencyBalanceOf<T>) -> DispatchResult {
			
			let mut balance = Self::get_pot(project_id,currency_id);
			ensure!(balance >= amount, Error::<T>::NotEnoughFunds);

			let account_id = Self::account_id();

			let new_balance = balance.checked_sub(&amount)
			.ok_or(Error::<T>::ReceivableLessThanPayment)?;

			T::Currency::transfer(currency_id,&account_id,&beneficiary,amount)?;
			Treasury::<T>::insert(project_id,currency_id,new_balance);
			Ok(())
		}
	}
}
