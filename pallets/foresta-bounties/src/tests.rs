use crate::{mock::*, Error, Event, Something, UnlockDuration};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

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