use frame_support::{
    dispatch::{result::Result, DispatchError, DispatchResult},
    traits::Get,
};
use sp_std::vec::Vec;

pub trait NFT<AccountId> {
    type AssetId;
    type AssetInfo;
    type AssetLimit: Get<u128>;
    type UserAssetLimit: Get<u64>;

    fn total_asset() -> u128;
    fn burned_asset() -> u128;
    fn total_for_account(account: &AccountId) -> u64;
    fn assets_for_account(account: &AccountId) -> Vec<(Self::AssetId, Self::AssetInfo)>;
    fn account_for_asset(asset_id: &Self::AssetId) -> AccountId;
    fn mint(owner_account: &AccountId, asset_info: Self::AssetInfo) -> Result<Self::AssetId, DispatchError>;
    fn burn(asset_id: &Self::AssetId) -> DispatchResult;
    fn transfer(dest_account: &AccountId, asset_id: &Self::AssetId) -> DispatchResult;
}