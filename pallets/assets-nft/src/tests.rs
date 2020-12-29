use crate::*;
use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use sp_core::hash::H256;
use super::RawEvent;
use frame_system as system;


pub type System = system::Module<Test>;

const ASSET_ID: [u8; 32] = [
	3, 23, 10, 46, 117, 151, 183, 183,
	227, 216, 76, 5, 57, 29, 19, 154,
	98, 177, 87, 231, 135, 134, 216,
	192, 130, 242, 157, 207, 76, 17, 19, 20
];

fn events() -> Vec<TestEvent> {
	let evt = System::events()
		.into_iter()
		.map(|evt| evt.event)
		.collect::<Vec<_>>();

	System::reset_events();

	evt
}

fn last_event() -> TestEvent {
	System::events()
		.pop()
		.expect("Event expected")
		.event
}

#[test]
fn mint() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let asset_id = H256::from_slice(&ASSET_ID);

		assert_ok!(AssetsNFT::mint(Origin::signed(0), 1, vec![], 100));

		assert_noop!(
			AssetsNFT::mint(Origin::signed(0), 1, vec![], 100),
			Error::<Test, DefaultInstance>::AssetExists
		);

		assert_noop!(
			AssetsNFT::mint(Origin::signed(0), 1, vec![0], 100),
			Error::<Test, DefaultInstance>::TooManyAssetsForAccount
		);

		assert_noop!(
			AssetsNFT::mint(Origin::signed(1), 2, vec![0], 100),
			Error::<Test, DefaultInstance>::TooManyAssets
		);

		assert_eq!(AssetsNFT::total_asset(), 1);
		assert_eq!(AssetsNFT::burned_asset(), 0);
		assert_eq!(AssetsNFT::total_for_account(1), 1);
		assert_eq!(AssetsNFT::assets_for_account(1), vec![(asset_id, vec![])]);
		assert_eq!(AssetsNFT::account_for_asset(asset_id), 1);
		assert_eq!(AssetsNFT::asset_balances(asset_id, 1), 100);
		assert_eq!(AssetsNFT::total_asset_supply(asset_id), 100);
		assert_eq!(events(), [
			TestEvent::asset_nft(RawEvent::Minted(asset_id, 1, 100)),
		]);
	});
}

#[test]
fn burn() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let asset_id = H256::from_slice(&ASSET_ID);

		assert_ok!(AssetsNFT::mint(Origin::signed(0), 1, vec![], 100));
		assert_ok!(AssetsNFT::burn(Origin::signed(0), asset_id));

		assert_eq!(AssetsNFT::total_asset(), 0);
		assert_eq!(AssetsNFT::burned_asset(), 1);
		assert_eq!(AssetsNFT::total_for_account(1), 0);
		assert_eq!(AssetsNFT::assets_for_account(1), vec![]);
		assert_eq!(AssetsNFT::account_for_asset(asset_id), 0);
		assert_eq!(AssetsNFT::asset_balances(asset_id, 1), 0);
		assert_eq!(AssetsNFT::total_asset_supply(asset_id), 0);
		assert_eq!(last_event(), TestEvent::asset_nft(RawEvent::Burned(asset_id)));
	})
}

#[test]
fn transfer_asset() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let asset_id = H256::from_slice(&ASSET_ID);

		assert_ok!(AssetsNFT::mint(Origin::signed(0), 1, vec![], 100));
		assert_ok!(AssetsNFT::transfer_asset(Origin::signed(1), 2, asset_id));

		assert_eq!(AssetsNFT::total_for_account(1), 0);
		assert_eq!(AssetsNFT::total_for_account(2), 1);
		assert_eq!(AssetsNFT::account_for_asset(asset_id), 2);
		assert_eq!(AssetsNFT::assets_for_account(1), vec![]);
		assert_eq!(AssetsNFT::assets_for_account(2), vec![(asset_id, vec![])]);
		assert_eq!(last_event(), TestEvent::asset_nft(RawEvent::AssetTransferred(asset_id, 2)));
	})
}

#[test]
fn transfer_token() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let asset_id = H256::from_slice(&ASSET_ID);

		assert_ok!(AssetsNFT::mint(Origin::signed(0), 1, vec![], 100));
		assert_ok!(AssetsNFT::transfer_token(Origin::signed(1), 2, asset_id, 20));

		assert_eq!(AssetsNFT::asset_balances(asset_id, 1), 80);
		assert_eq!(AssetsNFT::asset_balances(asset_id, 2), 20);
		assert_eq!(last_event(), TestEvent::asset_nft(RawEvent::TokenTransferred(asset_id, 2, 20)));

		assert_noop!(
			AssetsNFT::transfer_token(Origin::signed(1), 2, asset_id, 0),
			Error::<Test, DefaultInstance>::TokenAmountZero
		);

		assert_noop!(
			AssetsNFT::transfer_token(Origin::signed(1), 2, asset_id, 81),
			Error::<Test, DefaultInstance>::TokenBalanceLow
		);

		assert_noop!(
			AssetsNFT::transfer_token(Origin::signed(3), 2, asset_id, 1),
			Error::<Test, DefaultInstance>::NotAssetTokenOwner
		);
	})
}
