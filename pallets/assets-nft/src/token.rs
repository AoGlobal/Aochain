use frame_support::{dispatch::{DispatchResult}};


pub trait Token<AccountId, Balance> {
    type AssetId;

    fn issue(origin: &AccountId, assert_id: &Self::AssetId, total: Balance) -> DispatchResult;
    fn transfer(origin: &AccountId, target: &AccountId, assert_id: &Self::AssetId, amount: Balance) -> DispatchResult;
}