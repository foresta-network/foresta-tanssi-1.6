use crate::{mock::*, Config, Error, Event, UnlockDuration, BountiesCount, Bounty, BountyStatus};
use frame_support::{
	assert_noop, assert_ok, traits::{OnIdle, OnFinalize, OnInitialize}, weights::Weight, BoundedVec, PalletId,
};
use frame_system::RawOrigin;
use sp_runtime::{traits::AccountIdConversion, Percent};
use pallet_carbon_credits::{
	BatchGroupListOf, BatchGroupOf, BatchOf, ProjectCreateParams, RegistryListOf, SDGTypesListOf,
};
use pallet_dex::{SellerReceivables, Treasury};
use primitives::{Batch, RegistryDetails, RegistryName, Royalty, SDGDetails, SdgType, CurrencyId,  UserLevel};
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

pub fn init_project_and_treasury<T: Config>() {
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
}

fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 0 {
			ForestaBounties::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		ForestaBounties::on_initialize(System::block_number());
	}
}

#[test]
fn it_works_for_force_set_unlock_duration() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		
		assert_eq!(UnlockDuration::<Test>::get(), 0);

		assert_ok!(ForestaBounties::force_set_unlock_duration(RawOrigin::Root.into(),1000));

		assert_eq!(UnlockDuration::<Test>::get(), 1000);
		
	});
}

#[test]
fn it_works_for_create_bounty() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let project_id = 0;
		let admin = 1;

		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();
		
		assert_eq!(Tokens::free_balance(USDT, &dex_account), 0);

		//assert_ok!(Tokens::transfer(Some(4).into(), dex_account, USDT, 50));

		//assert_eq!(Tokens::free_balance(USDT, &dex_account), 50);

		init_project_and_treasury::<Test>();

		assert_eq!(Tokens::free_balance(USDT, &dex_account), 660);

		let pot = Treasury::<Test>::get(project_id,USDT);
		assert_eq!(pot,100);

		//create bounty

		assert_eq!(BountiesCount::<Test>::get(),0);

		assert_ok!(ForestaBounties::create_bounty(RawOrigin::Root.into(),project_id,USDT,50,
		"Bounty1".as_bytes().to_vec().try_into().unwrap()));
		
		assert_eq!(BountiesCount::<Test>::get(),1);
	});
}

#[test]
fn it_works_for_validate_bounty() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let project_id = 0;
		let admin = 1;
		let recipient = 2;
		let bounty_id = 0;
		let block_duration = 1000u64;

		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		init_project_and_treasury::<Test>();

		//create bounty

		let metadata = "Bounty1".as_bytes().to_vec().try_into().unwrap();

		assert_ok!(ForestaBounties::create_bounty(RawOrigin::Root.into(),project_id,USDT,50,
		metadata));

		
		let bounty = ForestaBounties::get_bounty(bounty_id).unwrap();
		assert_eq!(bounty.status,BountyStatus::Proposed);

		// Project admin activates bounty

		run_to_block(100);
		
		assert_ok!(ForestaBounties::activate_bounty(RawOrigin::Signed(admin).into(),bounty_id,recipient,block_duration));

		let activated_bounty = ForestaBounties::get_bounty(bounty_id).unwrap();
		assert_eq!(activated_bounty.status,BountyStatus::Active);
	});
}

#[test]
fn it_works_for_create_and_cancel_bounty() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let project_id = 0;
		let admin = 1;
		let bounty_id = 0;

		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		init_project_and_treasury::<Test>();

		//create bounty

		let metadata = "Bounty1".as_bytes().to_vec().try_into().unwrap();

		assert_ok!(ForestaBounties::create_bounty(RawOrigin::Root.into(),project_id,USDT,50,
		metadata));

		
		let bounty = ForestaBounties::get_bounty(bounty_id).unwrap();
		assert_eq!(bounty.status,BountyStatus::Proposed);	

		run_to_block(100);

		// Project admin cancels bounty
		
		assert_ok!(ForestaBounties::cancel_bounty(RawOrigin::Signed(admin).into(),bounty_id));

		let cancelled_bounty = ForestaBounties::get_bounty(bounty_id).unwrap();
		assert_eq!(cancelled_bounty.status,BountyStatus::Cancelled);
	});
}

#[test]
fn it_works_for_create_validate_and_cancel_bounty() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let project_id = 0;
		let admin = 1;
		let recipient = 2;
		let bounty_id = 0;
		let block_duration = 1000u64;

		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		init_project_and_treasury::<Test>();

		//create bounty

		let metadata = "Bounty1".as_bytes().to_vec().try_into().unwrap();

		assert_ok!(ForestaBounties::create_bounty(RawOrigin::Root.into(),project_id,USDT,50,
		metadata));

		
		let bounty = ForestaBounties::get_bounty(bounty_id).unwrap();
		assert_eq!(bounty.status,BountyStatus::Proposed);

		// Project admin activates bounty

		run_to_block(100);
		
		assert_ok!(ForestaBounties::activate_bounty(RawOrigin::Signed(admin).into(),bounty_id,recipient,block_duration));

		let activated_bounty = ForestaBounties::get_bounty(bounty_id).unwrap();
		assert_eq!(activated_bounty.status,BountyStatus::Active);

		// Project admin cancels bounty

		run_to_block(200);

		assert_ok!(ForestaBounties::cancel_bounty(RawOrigin::Signed(admin).into(),bounty_id));

		let cancelled_bounty = ForestaBounties::get_bounty(bounty_id).unwrap();
		assert_eq!(cancelled_bounty.status,BountyStatus::Cancelled);
	});
}

#[test]
fn it_works_for_recipient_submits_bounty() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let project_id = 0;
		let admin = 1;
		let recipient = 2;
		let bounty_id = 0;
		let block_duration = 1000u64;

		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		init_project_and_treasury::<Test>();

		//create bounty

		let metadata = "Bounty1".as_bytes().to_vec().try_into().unwrap();

		assert_ok!(ForestaBounties::create_bounty(RawOrigin::Root.into(),project_id,USDT,50,
		metadata));

		
		let bounty = ForestaBounties::get_bounty(bounty_id).unwrap();
		assert_eq!(bounty.status,BountyStatus::Proposed);

		// Project admin activates bounty

		run_to_block(100);
		
		assert_ok!(ForestaBounties::activate_bounty(RawOrigin::Signed(admin).into(),bounty_id,recipient,block_duration));

		let activated_bounty = ForestaBounties::get_bounty(bounty_id).unwrap();
		assert_eq!(activated_bounty.status,BountyStatus::Active);

		// Recipient submits bounty

		run_to_block(800);

		let submission = "Submission1".as_bytes().to_vec().try_into().unwrap();

		assert_ok!(ForestaBounties::submit_bounty(RawOrigin::Signed(recipient).into(),bounty_id,submission));

		let submitted_bounty = ForestaBounties::get_bounty(bounty_id).unwrap();
		assert_eq!(submitted_bounty.status,BountyStatus::Submitted);

	});
}

#[test]
fn it_works_for_bounty_awarded() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let project_id = 0;
		let admin = 1;
		let recipient = 2;
		let bounty_id = 0;
		let block_duration = 1000u64;

		let dex_account: u64 = PalletId(*b"bitg/dex").into_account_truncating();

		init_project_and_treasury::<Test>();

		// Set bounty payout unlcok duration

		assert_ok!(ForestaBounties::force_set_unlock_duration(RawOrigin::Root.into(),1000));


		//create bounty

		let metadata = "Bounty1".as_bytes().to_vec().try_into().unwrap();

		assert_ok!(ForestaBounties::create_bounty(RawOrigin::Root.into(),project_id,USDT,50,
		metadata));

		
		let bounty = ForestaBounties::get_bounty(bounty_id).unwrap();
		assert_eq!(bounty.status,BountyStatus::Proposed);

		// Project admin activates bounty

		run_to_block(100);
		
		assert_ok!(ForestaBounties::activate_bounty(RawOrigin::Signed(admin).into(),bounty_id,recipient,block_duration));

		let activated_bounty = ForestaBounties::get_bounty(bounty_id).unwrap();
		assert_eq!(activated_bounty.status,BountyStatus::Active);

		// Recipient submits bounty

		run_to_block(800);

		let submission = "Submission1".as_bytes().to_vec().try_into().unwrap();

		assert_ok!(ForestaBounties::submit_bounty(RawOrigin::Signed(recipient).into(),bounty_id,submission));

		let submitted_bounty = ForestaBounties::get_bounty(bounty_id).unwrap();
		assert_eq!(submitted_bounty.status,BountyStatus::Submitted);

		// Admin awards bounty

		run_to_block(1100);

		assert_ok!(ForestaBounties::award_bounty(RawOrigin::Signed(admin).into(),bounty_id));

		assert_eq!(Tokens::free_balance(USDT, &recipient), 0);

		// Check payout

		run_to_block(2101);

		assert_eq!(Tokens::free_balance(USDT, &recipient), 50);

	});
}





