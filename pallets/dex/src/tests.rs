// This file is part of Foresta.
// Copyright (C) 2024 Foresta.
// This code is licensed under MIT license (see LICENSE.txt for details)
use crate::{
	mock::*, Config, types::UserLevel, Error, Event, Orders, SellerReceivables,
Treasury };
use frame_support::{
	assert_noop, assert_ok, traits::OnIdle, weights::Weight, BoundedVec, PalletId,
};
use frame_system::RawOrigin;
use sp_runtime::{traits::AccountIdConversion, Percent};
use pallet_carbon_credits::{
	BatchGroupListOf, BatchGroupOf, BatchOf, ProjectCreateParams, RegistryListOf, SDGTypesListOf,
};
use primitives::{Batch, RegistryDetails, RegistryName, Royalty, SDGDetails, SdgType, CurrencyId};
use orml_traits::MultiCurrency;


/// helper function to generate standard registry details
fn get_default_registry_details<T: Config>() -> RegistryListOf<T> {
	let registry_details = RegistryDetails {
		reg_name: RegistryName::Verra,
		name: "reg_name".as_bytes().to_vec().try_into().unwrap(),
		id: "reg_id".as_bytes().to_vec().try_into().unwrap(),
		summary: "reg_summary".as_bytes().to_vec().try_into().unwrap(),
	};
	vec![registry_details].try_into().unwrap()
}

/// helper function to generate standard sdg details
fn get_default_sdg_details<T: Config>() -> SDGTypesListOf<T> {
	let sdg_details: SDGTypesListOf<T> = vec![SDGDetails {
		sdg_type: SdgType::LifeOnLand,
		description: "sdg_desp".as_bytes().to_vec().try_into().unwrap(),
		references: "sdg_ref".as_bytes().to_vec().try_into().unwrap(),
	}]
	.try_into()
	.unwrap();

	sdg_details
}

fn get_single_batch_list<T: Config>() -> BoundedVec<BatchOf<T>, T::MaxGroupSize> {
	vec![Batch {
		name: "batch_name".as_bytes().to_vec().try_into().unwrap(),
		uuid: "batch_uuid".as_bytes().to_vec().try_into().unwrap(),
		issuance_year: 2020_u16,
		start_date: 2020_u16,
		end_date: 2020_u16,
		total_supply: 100_u32.into(),
		minted: 0_u32.into(),
		retired: 0_u32.into(),
	}]
	.try_into()
	.unwrap()
}

fn get_multiple_batch_list<T: Config>() -> BoundedVec<BatchOf<T>, T::MaxGroupSize> {
	vec![
		Batch {
			name: "batch_name".as_bytes().to_vec().try_into().unwrap(),
			uuid: "batch_uuid".as_bytes().to_vec().try_into().unwrap(),
			issuance_year: 2020_u16,
			start_date: 2020_u16,
			end_date: 2020_u16,
			total_supply: 100_u32.into(),
			minted: 0_u32.into(),
			retired: 0_u32.into(),
		},
		Batch {
			name: "batch_name_2".as_bytes().to_vec().try_into().unwrap(),
			uuid: "batch_uuid_2".as_bytes().to_vec().try_into().unwrap(),
			issuance_year: 2021_u16,
			start_date: 2021_u16,
			end_date: 2021_u16,
			total_supply: 100_u32.into(),
			minted: 0_u32.into(),
			retired: 0_u32.into(),
		},
	]
	.try_into()
	.unwrap()
}

/// helper function to generate standard batch details
fn get_default_batch_group<T: Config>() -> BatchGroupListOf<T>
where
	<T as frame_system::Config>::AccountId: From<u32>,
{
	vec![BatchGroupOf::<T> {
		name: "batch_group_name".as_bytes().to_vec().try_into().unwrap(),
		uuid: "batch_group_uuid".as_bytes().to_vec().try_into().unwrap(),
		asset_id: 0_u32.into(),
		total_supply: 100_u32.into(),
		minted: 0_u32.into(),
		retired: 0_u32.into(),
		batches: get_single_batch_list::<T>(),
	}]
	.try_into()
	.unwrap()
}

/// helper function to generate multiple batch details
fn get_multiple_batch_group<T: Config>() -> BatchGroupListOf<T>
where
	<T as frame_system::Config>::AccountId: From<u32>,
{
	vec![BatchGroupOf::<T> {
		name: "batch_group_name".as_bytes().to_vec().try_into().unwrap(),
		uuid: "batch_group_uuid".as_bytes().to_vec().try_into().unwrap(),
		asset_id: 0_u32.into(),
		total_supply: 100_u32.into(),
		minted: 0_u32.into(),
		retired: 0_u32.into(),
		batches: get_multiple_batch_list::<T>(),
	}]
	.try_into()
	.unwrap()
}

/// helper function to generate standard creation details
fn get_default_creation_params<T: Config>() -> ProjectCreateParams<T>
where
	<T as frame_system::Config>::AccountId: From<u32>,
{
	let royalty = Royalty::<T::AccountId> {
		account_id: 1_u32.into(),
		percent_of_fees: Percent::from_percent(0),
	};

	let creation_params = ProjectCreateParams {
		name: "name".as_bytes().to_vec().try_into().unwrap(),
		description: "description".as_bytes().to_vec().try_into().unwrap(),
		location: "(1, 1), (2, 2), (3, 3), (4, 4)".as_bytes().to_vec().try_into().unwrap(),
		images: vec!["image_link".as_bytes().to_vec().try_into().unwrap()].try_into().unwrap(),
		videos: vec!["video_link".as_bytes().to_vec().try_into().unwrap()].try_into().unwrap(),
		documents: vec!["document_link".as_bytes().to_vec().try_into().unwrap()]
			.try_into()
			.unwrap(),
		registry_details: get_default_registry_details::<T>(),
		sdg_details: get_default_sdg_details::<T>(),
		royalties: Some(vec![royalty].try_into().unwrap()),
		batch_groups: get_default_batch_group::<T>(),
		project_type: None,
	};

	creation_params
}

pub fn create_project_and_mint<T: Config>(
	originator_account: u64,
	amount_to_mint: u32,
	batch: bool,
) {
	let mut creation_params = get_default_creation_params::<Test>();
	let project_id = 0;
	let group_id = 0;
	if batch {
		// replace the default with mutiple batches
		let created_batch_list = get_multiple_batch_group::<Test>();
		creation_params.batch_groups = created_batch_list;
	}

	let authorised_account = 10;

	assert_ok!(CarbonCredits::create(
		RawOrigin::Signed(originator_account).into(),
		creation_params,1u32
	));

	// approve project so minting can happen
	assert_ok!(CarbonCredits::force_add_authorized_account(
		RawOrigin::Root.into(),
		authorised_account
	));
	assert_ok!(CarbonCredits::approve_project(
		RawOrigin::Root.into(),
		project_id,
		true
	),);

	// mint should work with all params correct
	assert_ok!(CarbonCredits::mint(
		RawOrigin::Signed(authorised_account).into(),
		project_id,
		group_id,
		amount_to_mint.into(),
		false
	));
}


#[test]
fn basic_create_sell_order_should_work() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let project_tokens_to_mint = 100;
		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		create_project_and_mint::<Test>(seller, project_tokens_to_mint, false);
		//assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		//assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);
		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 5, USDT, 1));

		// storage should be updated correctly
		let sell_order_storage = Orders::<Test>::get(0).unwrap();
		assert_eq!(sell_order_storage.owner, seller);
		assert_eq!(sell_order_storage.units, 5);
		assert_eq!(sell_order_storage.price_per_unit, 1);
		assert_eq!(sell_order_storage.asset_id, asset_id);

		// Balance should be setup correctly
		assert_eq!(Assets::balance(asset_id, seller), 95);
		assert_eq!(Assets::balance(asset_id, dex_account), 5);

		assert_eq!(
			last_event(),
			Event::SellOrderCreated {
				order_id: 0,
				asset_id,
				project_id: 0,
				group_id: 0,
				units: 5,
				currency_id: USDT,
				price_per_unit: 1,
				owner: seller
			}
			.into()
		);
	});
}

#[test]
fn create_sell_order_less_than_minimum_should_fail() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let project_tokens_to_mint = 100;

		create_project_and_mint::<Test>(seller, project_tokens_to_mint, false);
		//assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		//assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// sell order with less than minimum units
		assert_noop!(
			Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 0, USDT, 1),
			Error::<Test>::BelowMinimumUnits
		);

		// sell order with less than minimum price
		assert_noop!(
			Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 5, USDT, 0),
			Error::<Test>::BelowMinimumPrice
		);
	});
}

#[test]
fn create_sell_order_should_fail_if_caller_does_not_have_asset_balance() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;

		let project_tokens_to_mint = 100;

		create_project_and_mint::<Test>(seller, project_tokens_to_mint, false);
		//assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		//assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);
		// should not be able to create a sell order since the amount is greater than seller balance
		assert_noop!(
			Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 101, USDT, 1),
			sp_runtime::ArithmeticError::Underflow
		);
	});
}

#[test]
fn cancel_sell_order_should_work() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		let project_tokens_to_mint = 100;

		create_project_and_mint::<Test>(seller, project_tokens_to_mint, false);
		//assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		//assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 5, USDT, 1));

		// storage should be updated correctly
		let sell_order_storage = Orders::<Test>::get(0).unwrap();
		assert_eq!(sell_order_storage.owner, seller);
		assert_eq!(sell_order_storage.units, 5);
		assert_eq!(sell_order_storage.price_per_unit, 1);
		assert_eq!(sell_order_storage.asset_id, asset_id);

		// Balance should be setup correctly
		assert_eq!(Assets::balance(asset_id, seller), 95);
		assert_eq!(Assets::balance(asset_id, dex_account), 5);

		// non existing order should fail
		assert_noop!(
			Dex::cancel_sell_order(RuntimeOrigin::signed(4), 10),
			Error::<Test>::InvalidOrderId
		);

		// only owner can cancel sell order
		assert_noop!(
			Dex::cancel_sell_order(RuntimeOrigin::signed(4), 0),
			Error::<Test>::InvalidOrderOwner
		);

		// owner should be able to cancel a sell order
		assert_ok!(Dex::cancel_sell_order(RuntimeOrigin::signed(seller), 0));

		// storage should be updated correctly
		assert!(Orders::<Test>::get(0).is_none());

		// Balance should be returned correctly
		assert_eq!(Assets::balance(asset_id, seller), 100);
		assert_eq!(Assets::balance(asset_id, dex_account), 0);

		assert_eq!(last_event(), Event::SellOrderCancelled { order_id: 0, seller }.into());
	});
}


#[test]
fn cannot_set_more_than_max_fee() {
	new_test_ext().execute_with(|| {
		// set fee values
		assert_noop!(
			Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(51)),
			Error::<Test>::CannotSetMoreThanMaxPaymentFee
		);
		assert_ok!(Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(10)));
	});
}

#[test]
fn cannot_set_more_than_max_royalty() {
	new_test_ext().execute_with(|| {
		// set fee values
		assert_noop!(
			Dex::force_set_royalty(RuntimeOrigin::root(), Percent::from_percent(15)),
			Error::<Test>::CannotSetMoreThanMaxRoyalty
		);
		assert_ok!(Dex::force_set_royalty(RuntimeOrigin::root(), Percent::from_percent(10)));
	});
}

#[test]
fn buy_sell_order_test() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let buyer = 4;
		let buy_order_id = 0;
		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		let project_tokens_to_mint = 100;

		create_project_and_mint::<Test>(seller, project_tokens_to_mint, false);
		//assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		//assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// set fee values
		assert_ok!(Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(10)));
		assert_ok!(Dex::force_set_purchase_fee(RuntimeOrigin::root(), 10u32.into()));
		assert_ok!(Dex::force_set_royalty(RuntimeOrigin::root(), Percent::from_percent(10)));

		// configure limit to avoid failure
		assert_ok!(Dex::force_set_open_order_allowed_limits(
			RuntimeOrigin::root(),
			UserLevel::KYCLevel1,
			1000
		));

		assert_eq!(Tokens::free_balance(USDT, &dex_account), 0);

		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 5, USDT, 1000));

		// create a new buy order
		assert_ok!(Dex::trade_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 110, false, None));

		let project_id = 0;
		// seller receivable should be updated with the correct amount
		let seller_receivables = SellerReceivables::<Test>::get(seller,USDT);
		// seller 9 treasury 1
		assert_eq!(seller_receivables, 900);
		let pot = Treasury::<Test>::get(project_id,USDT);
		assert_eq!(pot,100);

		// Asset balance should be set correctly
		assert_eq!(Assets::balance(asset_id, seller), 95);
		assert_eq!(Assets::balance(asset_id, dex_account), 4);
		assert_eq!(Assets::balance(asset_id, buyer), 1);// Retest
		assert_eq!(Tokens::free_balance(USDT, &dex_account), 1110);
	});
}

#[test]
fn retire_sell_order_test() {
	new_test_ext().execute_with(|| {
		let asset_id = 0;
		let seller = 1;
		let buyer = 4;
		let buy_order_id = 0;
		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		let project_tokens_to_mint = 100;

		create_project_and_mint::<Test>(seller, project_tokens_to_mint, false);
		//assert_ok!(Assets::force_create(RuntimeOrigin::root(), asset_id, 1, true, 1));
		//assert_ok!(Assets::mint(RuntimeOrigin::signed(seller), asset_id, 1, 100));
		assert_eq!(Assets::balance(asset_id, seller), 100);

		// set fee values
		assert_ok!(Dex::force_set_payment_fee(RuntimeOrigin::root(), Percent::from_percent(10)));
		assert_ok!(Dex::force_set_purchase_fee(RuntimeOrigin::root(), 10u32.into()));
		assert_ok!(Dex::force_set_royalty(RuntimeOrigin::root(), Percent::from_percent(10)));

		// configure limit to avoid failure
		assert_ok!(Dex::force_set_open_order_allowed_limits(
			RuntimeOrigin::root(),
			UserLevel::KYCLevel1,
			1000
		));

		assert_eq!(Tokens::free_balance(USDT, &dex_account), 0);

		// should be able to create a sell order
		assert_ok!(Dex::create_sell_order(RuntimeOrigin::signed(seller), asset_id, 5, USDT, 1000));

		// create a new buy order
		assert_ok!(Dex::trade_order(RuntimeOrigin::signed(buyer), 0, asset_id, 1, 110, true, None));

		let project_id = 0;
		// seller receivable should be updated with the correct amount
		let seller_receivables = SellerReceivables::<Test>::get(seller,USDT);
		// seller 9 treasury 1
		assert_eq!(seller_receivables, 900);
		let pot = Treasury::<Test>::get(project_id,USDT);
		assert_eq!(pot,100);

		// Asset balance should be set correctly
		assert_eq!(Assets::balance(asset_id, seller), 95);
		assert_eq!(Assets::balance(asset_id, dex_account), 4);
		assert_eq!(Assets::balance(asset_id, buyer), 0);// Retest

		assert_eq!(Tokens::free_balance(USDT, &dex_account), 1110);

		// Seller claims half receivables

		assert_eq!(Tokens::free_balance(USDT, &seller), 0);

		assert_ok!(Dex::claim_tokens(RuntimeOrigin::signed(seller),USDT,450));
		assert_eq!(SellerReceivables::<Test>::get(seller,USDT),450);
		assert_eq!(Tokens::free_balance(USDT, &seller), 450);
	});
}