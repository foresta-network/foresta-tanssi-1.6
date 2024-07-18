// This file is part of Foresta.
// Copyright (C) 2024 Foresta.
// This code is licensed under MIT license (see LICENSE.txt for details)
use frame_support::{
	pallet_prelude::DispatchResult,
	parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU16, ConstU32, Contains, Nothing},
	PalletId,
};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_system::{EnsureRoot, EnsureSigned};
use orml_traits::{parameter_type_with_key,MultiCurrency};
use primitives::{Amount, Balance, CarbonCreditsValidator, CurrencyId};
use sp_core::{ConstU128, ConstU64, H256};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup, AccountIdConversion},
	BuildStorage, Percent,
};
use sp_std::convert::{TryFrom, TryInto};

pub type AccountId = u64;
pub const USDT: CurrencyId = CurrencyId::USDT;

use crate as pallet_dex;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Assets: pallet_assets::{Pallet, Call, Storage, Event<T>},
		KYCMembership: pallet_membership,
		Uniques: pallet_uniques,
		CarbonCredits: pallet_carbon_credits::{Pallet, Call, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Call, Storage, Event<T>},
		Dex: pallet_dex::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Block = Block;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type RuntimeTask = RuntimeTask;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type AccountStore = System;
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type FreezeIdentifier = ();
	type MaxHolds = ConstU32<0>;
	type MaxFreezes = ConstU32<0>;
}

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type RemoveItemsLimit = ConstU32<1000>;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<Self::AccountId>>;
	type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type AssetDeposit = ConstU128<0>;
	type AssetAccountDeposit = ConstU128<0>;
	type MetadataDepositBase = ConstU128<0>;
	type MetadataDepositPerByte = ConstU128<0>;
	type ApprovalDeposit = ConstU128<0>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = ();
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Default::default()
	};
}

impl orml_tokens::Config for Test {
	type Amount = Amount;
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type DustRemovalWhitelist = Nothing;
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = ();
	type MaxReserves = ();
	type CurrencyHooks = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}

parameter_types! {
	pub const MarketplaceEscrowAccount : u64 = 10;
	pub const CarbonCreditsPalletId: PalletId = PalletId(*b"bitg/ccp");
	pub CarbonCreditsPalletAcccount : u64 = PalletId(*b"bitg/ccp").into_account_truncating();
	#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
	pub const MaxGroupSize: u32 = 10;
  }
  
  impl pallet_carbon_credits::Config for Test {
	  type AssetHandler = Assets;
	  type AssetId = u32;
	  type Balance = u128;
	  type RuntimeEvent = RuntimeEvent;
	  type ForceOrigin = frame_system::EnsureRoot<u64>;
	  type ItemId = u32;
	  type ProjectId = u32;
	  type GroupId = u32;
	  type KYCProvider = KYCMembership;
	  type MarketplaceEscrow = MarketplaceEscrowAccount;
	  type MaxAuthorizedAccountCount = ConstU32<2>;
	  type MaxDocumentCount = ConstU32<2>;
	  type MaxGroupSize = MaxGroupSize;
	  type MaxIpfsReferenceLength = ConstU32<20>;
	  type MaxLongStringLength = ConstU32<100>;
	  type MaxCoordinatesLength = ConstU32<8>;
	  type MaxRoyaltyRecipients = ConstU32<5>;
	  type MaxShortStringLength = ConstU32<20>;
	  type MinProjectId = ConstU32<1000>;
	  type NFTHandler = Uniques;
	  type PalletId = CarbonCreditsPalletId;
	  type MaxRetirementRecords = ConstU32<100>;
	  type WeightInfo = ();
  }

pub struct MockKycProvider;
impl Contains<u64> for MockKycProvider {
	fn contains(value: &u64) -> bool {
		// special account to test negative kyc
		if value == &20 {
			return false
		}

		true
	}
}

impl pallet_uniques::Config for Test {
	type AttributeDepositBase = ConstU128<1>;
	type CollectionDeposit = ConstU128<0>;
	type CollectionId = u32;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<u64>>;
	type Currency = Balances;
	type DepositPerByte = ConstU128<1>;
	type RuntimeEvent = RuntimeEvent;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type ItemDeposit = ConstU128<0>;
	type ItemId = u32;
	type KeyLimit = ConstU32<50>;
	type Locker = ();
	type MetadataDepositBase = ConstU128<1>;
	type StringLimit = ConstU32<50>;
	type ValueLimit = ConstU32<50>;
	type WeightInfo = ();
}

impl pallet_membership::Config for Test {
	type AddOrigin = EnsureRoot<u64>;
	type RuntimeEvent = RuntimeEvent;
	type MaxMembers = ConstU32<10>;
	type MembershipChanged = ();
	type MembershipInitialized = ();
	type PrimeOrigin = EnsureRoot<u64>;
	type RemoveOrigin = EnsureRoot<u64>;
	type ResetOrigin = EnsureRoot<u64>;
	type SwapOrigin = EnsureRoot<u64>;
	type WeightInfo = ();
}

parameter_types! {
	pub const DexPalletId: PalletId = PalletId(*b"bitg/dex");
	pub const MinUnitsToCreateSellOrder : u32 = 2;
	pub const MinPricePerUnit : u32 = 1;
	pub const MaxPaymentFee : Percent = Percent::from_percent(50);
	pub const MaxRoyalty : Percent = Percent::from_percent(10);
	pub const MaxPurchaseFee : u128 = 100u128;
	#[derive(Clone, scale_info::TypeInfo)]
	pub const MaxValidators : u32 = 10;
	#[derive(Clone, scale_info::TypeInfo, Debug, PartialEq)]
	pub const MaxTxHashLen : u32 = 100;
	#[derive(Clone, scale_info::TypeInfo)]
	pub const BuyOrderExpiryTime : u32 = 2;
	#[derive(Clone, scale_info::TypeInfo, Debug, PartialEq)]
	pub const MaxAddressLen : u32 = 100;
	#[derive(Clone, scale_info::TypeInfo, Debug, PartialEq)]
	pub const MaxOrderIds : u32 = 100;
	#[derive(Clone, scale_info::TypeInfo, Debug, PartialEq)]
	pub const MaxPayoutsToStore : u32 = 1000;
	#[derive(Clone, scale_info::TypeInfo, Debug, PartialEq)]
	pub const MaxOpenOrdersPerUser : u32 = 2;
	pub const MaxMembersC : u32 = 100;
}

impl pallet_dex::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Asset = Assets;
	type Currency = Tokens;
	type CurrencyBalance = u128;
	type PalletId = DexPalletId;
	type KYCProvider = MockKycProvider;
	type MinPricePerUnit = MinPricePerUnit;
	type MaxValidators = MaxValidators;
	type MaxTxHashLen = MaxTxHashLen;
	type BuyOrderExpiryTime = BuyOrderExpiryTime;
	type MaxAddressLen = MaxAddressLen;
	type MaxOrderIds = MaxOrderIds;
	type MaxOpenOrdersPerUser = MaxOpenOrdersPerUser;
	type MinUnitsToCreateSellOrder = MinUnitsToCreateSellOrder;
	type ForceOrigin = EnsureRoot<AccountId>;
	type MaxPaymentFee = MaxPaymentFee;
	type MaxRoyalty = MaxRoyalty;
	type MaxPurchaseFee = MaxPurchaseFee;
	type MaxPayoutsToStore = MaxPayoutsToStore;
	type MaxMembersPerCollective = MaxMembersC;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	pallet_membership::GenesisConfig::<Test> {
		members: sp_core::bounded_vec![1, 3, 4, 10],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();
	
	orml_tokens::GenesisConfig::<Test> { balances: vec![(4, USDT, 100), (10, USDT, 10000)] }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext: sp_io::TestExternalities = t.into();
	// need to set block number to 1 to test events
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn last_event() -> RuntimeEvent {
	System::events().pop().expect("Event expected").event
}