use super::*;
use frame_support::{traits::fungibles::Inspect, BoundedVec};
use frame_system::pallet_prelude::BlockNumberFor;
use orml_traits::MultiCurrency;
//use primitives::CarbonCreditsValidator;
pub use primitives::UserLevel;
use sp_runtime::traits::Get;

pub type CurrencyBalanceOf<T> =
	<<T as Config>::Currency as MultiCurrency<<T as frame_system::Config>::AccountId>>::Balance;
pub type CurrencyIdOf<T> =
	<<T as Config>::Currency as MultiCurrency<<T as frame_system::Config>::AccountId>>::CurrencyId;
//pub type AssetBalanceOf<T> = T::Balance;

pub type AssetIdOf<T> =
	<<T as Config>::Asset as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;

pub type ProjectIdOf<T> = <T as pallet_carbon_credits::Config>::ProjectId;

pub type GroupIdOf<T> = <T as pallet_carbon_credits::Config>::GroupId;

pub type CollectiveId = u32;
/// ValidatorAccounts type of pallet
pub type ValidatorAccountsListOf<T> =
	BoundedVec<<T as frame_system::Config>::AccountId, <T as pallet::Config>::MaxValidators>;

pub type OrderInfoOf<T> = OrderInfo<
	<T as frame_system::Config>::AccountId,
	<T as pallet_carbon_credits::Config>::AssetId,
	<T as pallet_carbon_credits::Config>::Balance,
	CurrencyIdOf<T>,
	CurrencyBalanceOf<T>,
>;

pub type BuyOrderInfoOf<T> = BuyOrderInfo<
	<T as frame_system::Config>::AccountId,
	<T as pallet_carbon_credits::Config>::AssetId,
	<T as pallet_carbon_credits::Config>::Balance,
	CurrencyBalanceOf<T>,
	BlockNumberFor<T>,
	<T as Config>::MaxTxHashLen,
	<T as Config>::MaxValidators,
>;

/// PayoutExecutedToSellerOf<T> represents a specialized version of PayoutExecutedToSeller
/// where the generic parameters are replaced with the corresponding types from the `Runtime`
/// configuration.
pub type PayoutExecutedToSellerOf<T> = PayoutExecutedToSeller<
	CurrencyBalanceOf<T>,
	<T as Config>::MaxTxHashLen,
	<T as Config>::MaxOrderIds,
	<T as Config>::MaxAddressLen,
>;

pub type SellerPayoutPreferenceOf<T> = SellerPayoutPreference<<T as Config>::MaxAddressLen>;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
pub struct OrderInfo<AccountId, AssetId, AssetBalance, TokenId, TokenBalance> {
	pub owner: AccountId,
	pub units: AssetBalance,
	pub price_per_unit: TokenBalance,
	pub currency_id: TokenId,
	pub asset_id: AssetId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
pub struct BuyOrderInfo<
	AccountId,
	AssetId,
	AssetBalance,
	TokenBalance,
	Time,
	TxProofLen: Get<u32> + Clone,
	MaxValidators: Get<u32> + Clone,
> {
	pub order_id: OrderId,
	pub buyer: AccountId,
	pub units: AssetBalance,
	pub price_per_unit: TokenBalance,
	pub asset_id: AssetId,
	pub total_fee: TokenBalance,
	pub total_amount: TokenBalance,
	pub expiry_time: Time,
	pub payment_info: Option<PaymentInfo<AccountId, TxProofLen, MaxValidators>>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
pub struct PaymentInfo<AccountId, TxProofLen: Get<u32> + Clone, MaxValidators: Get<u32> + Clone> {
	pub chain_id: u32,
	pub tx_proof: BoundedVec<u8, TxProofLen>,
	pub validators: BoundedVec<AccountId, MaxValidators>,
}

/// The preference set by a seller for receiveing payment transactions
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
pub struct SellerPayoutPreference<MaxAddressLen: Get<u32> + Clone> {
	/// The chain ID associated with the payment.
	/// We do not enforce this but the chainID is represented as follows:
	/// 0 - Stripe
	/// 1 - Eth
	/// 137 - Polygon
	pub chain_id: u32,

	/// The recipient's address where the payment should be sent
	pub recipient_address: BoundedVec<u8, MaxAddressLen>,
}

/// PayoutExecutedToSeller represents the information of a payment executed to a seller.
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
pub struct PayoutExecutedToSeller<
	Balance,
	TxHashLen: Get<u32> + Clone,
	MaxOrderIds: Get<u32> + Clone,
	MaxAddressLen: Get<u32> + Clone,
> {
	/// The order IDs associated with the payment.
	pub order_id: BoundedVec<OrderId, MaxOrderIds>,

	/// The chain ID associated with the payment.
	/// We do not enforce this but the chainID is represented as follows:
	/// 0 - Stripe
	/// 1 - Eth
	/// 137 - Polygon
	pub chain_id: u32,

	/// The recipient's address where the payment was sent.
	pub recipient_address: BoundedVec<u8, MaxAddressLen>,

	/// The amount of the payment executed.
	pub amount: Balance,

	/// The transaction hash associated with the payment.
	pub tx_hash: BoundedVec<u8, TxHashLen>,
}

pub type OrderId = u128;

pub type BuyOrderId = u128;