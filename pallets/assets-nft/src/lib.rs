#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use codec::FullCodec;
use frame_support::{
	Parameter, decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
	traits::{EnsureOrigin, Get},
	Hashable,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::{Hash, Member, AtLeast32BitUnsigned, Zero, StaticLookup};
use sp_std::{cmp::Eq, fmt::Debug, vec::Vec};

mod nft;
mod token;
mod weight_info;
mod default_weights;

use crate::nft::NFT;
use crate::token::Token;
use crate::weight_info::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait<I: Instance = DefaultInstance>: system::Trait {
	type Event: From<Event<Self, I>> + Into<<Self as system::Trait>::Event>;

	type MintOrigin: EnsureOrigin<Self::Origin>;

	type BurnOrigin: EnsureOrigin<Self::Origin>;

	type AssetLimit: Get<u128>;

	type UserAssetLimit: Get<u64>;

	type BurnOwnedCheck: Get<bool>;

	type AllowBurn: Get<bool>;

	type AssetInfo: Hashable + Member + Debug + Default + FullCodec + Ord;

	type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

	type WeightInfo: WeightInfo;
}

pub type AssetId<T> = <T as system::Trait>::Hash;

pub type Asset<T, I> = (AssetId<T>, <T as Trait<I>>::AssetInfo);

decl_storage! {
	trait Store for Module<T: Trait<I>, I: Instance = DefaultInstance> as AssetNFT {
		TotalAsset get(fn total_asset): u128 = 0;

		BurnedAsset get(fn burned_asset): u128 = 0;

		TotalForAccount get(fn total_for_account):
			map hasher(blake2_128_concat) T::AccountId => u64 = 0;

		AssetsForAccount get(fn assets_for_account):
			map hasher(blake2_128_concat) T::AccountId => Vec<Asset<T, I>>;

		AccountForAsset get(fn account_for_asset):
			map hasher(twox_64_concat) AssetId<T> => T::AccountId;

		AssetBalances get(fn asset_balances):
			double_map hasher(twox_64_concat) AssetId<T>, hasher(blake2_128_concat) T::AccountId => T::Balance;

		TotalAssetSupply get(fn total_asset_supply):
			map hasher(twox_64_concat) AssetId<T> => T::Balance;
	}
}

decl_event!(
	pub enum Event<T, I = DefaultInstance>
	where
		AssetId = <T as system::Trait>::Hash,
		<T as system::Trait>::AccountId,
		<T as Trait<I>>::Balance,
	{
        Burned(AssetId),
        Minted(AssetId, AccountId, Balance),
        AssetTransferred(AssetId, AccountId),
        TokenTransferred(AssetId, AccountId, Balance),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait<I>, I: Instance> {
		AssetExists,
		TooManyAssetsForAccount,
		TooManyAssets,
		TokenAmountZero,
		TokenBalanceLow,
		NotAssetOwner,
		NotAssetTokenOwner,
		NonexistentAsset,
		DisallowedOperation,
	}
}

decl_module! {
	pub struct Module<T: Trait<I>, I: Instance = DefaultInstance> for enum Call where origin: T::Origin {
		type Error = Error<T, I>;

		fn deposit_event() = default;

		#[weight = T::WeightInfo::mint()]
		pub fn mint(
			origin,
			dest_account: T::AccountId,
			asset_info: T::AssetInfo,
			balance: T::Balance,
		) -> dispatch::DispatchResult {
			T::MintOrigin::ensure_origin(origin)?;

			let asset_id = <Self as NFT<_>>::mint(&dest_account, asset_info)?;
            <Self as Token<_, _>>::issue(&dest_account, &asset_id, balance)?;
            Self::deposit_event(RawEvent::Minted(asset_id, dest_account.clone(), balance));

			Ok(())
		}

		#[weight = T::WeightInfo::burn()]
		pub fn burn(
			origin,
			asset_id: AssetId<T>
		) -> dispatch::DispatchResult {
			ensure!(T::AllowBurn::get(), Error::<T, I>::DisallowedOperation);

			T::BurnOrigin::ensure_origin(origin.clone())?;

			if T::BurnOwnedCheck::get() {
				let who = ensure_signed(origin)?;
				ensure!(who == Self::account_for_asset(&asset_id), Error::<T, I>::NotAssetOwner);
			}

			<Self as NFT<_>>::burn(&asset_id)?;
			Self::deposit_event(RawEvent::Burned(asset_id.clone()));

            Ok(())
		}

		#[weight = T::WeightInfo::transfer_asset()]
		pub fn transfer_asset(
			origin,
			dest_account: <T::Lookup as StaticLookup>::Source,
			asset_id: AssetId<T>
		) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(who == Self::account_for_asset(&asset_id), Error::<T, I>::NotAssetOwner);

			let dest_account = T::Lookup::lookup(dest_account)?;
			<Self as NFT<_>>::transfer(&dest_account, &asset_id)?;
            Self::deposit_event(RawEvent::AssetTransferred(asset_id.clone(), dest_account.clone()));

            Ok(())
		}

		#[weight = T::WeightInfo::transfer_token()]
		pub fn transfer_token(
			origin,
			dest_account: <T::Lookup as StaticLookup>::Source,
			asset_id: AssetId<T>,
			amount: T::Balance
		) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(!Self::asset_balances(&asset_id, &who).is_zero(), Error::<T, I>::NotAssetTokenOwner);

			let dest_account = T::Lookup::lookup(dest_account)?;
			<Self as Token<_, _>>::transfer(&who, &dest_account, &asset_id, amount.clone())?;
            Self::deposit_event(RawEvent::TokenTransferred(asset_id.clone(), dest_account.clone(), amount.clone()));

			Ok(())
		}
	}
}

impl<T: Trait<I>, I: Instance> NFT<T::AccountId> for Module<T, I> {
	type AssetId = AssetId<T>;
	type AssetInfo = T::AssetInfo;
	type AssetLimit = T::AssetLimit;
	type UserAssetLimit = T::UserAssetLimit;

	fn total_asset() -> u128 {
		Self::total_asset()
	}

	fn burned_asset() -> u128 {
		Self::burned_asset()
	}

	fn total_for_account(account: &T::AccountId) -> u64 {
		Self::total_for_account(account)
	}

	fn assets_for_account(account: &T::AccountId) -> Vec<Asset<T, I>> {
		Self::assets_for_account(account)
	}

	fn account_for_asset(asset_id: &Self::AssetId) -> T::AccountId {
		Self::account_for_asset(asset_id)
	}

	fn mint(
		owner_account: &T::AccountId,
		asset_info: <T as Trait<I>>::AssetInfo,
	) -> dispatch::result::Result<Self::AssetId, dispatch::DispatchError> {
		let asset_id = T::Hashing::hash_of(&asset_info);

		ensure!(!AccountForAsset::<T, I>::contains_key(&asset_id), Error::<T, I>::AssetExists);
		ensure!(Self::total_for_account(owner_account) < T::UserAssetLimit::get(), Error::<T, I>::TooManyAssetsForAccount);
		ensure!(Self::total_asset() < T::AssetLimit::get(), Error::<T, I>::TooManyAssets);

		let new_asset = (asset_id, asset_info);

		TotalAsset::<I>::mutate(|total| *total += 1);
		<TotalForAccount<T, I>>::mutate(owner_account, |total| *total += 1);
		<AccountForAsset<T, I>>::insert(asset_id, &owner_account);
		<AssetsForAccount<T, I>>::mutate(owner_account, |assets| {
			match assets.binary_search(&new_asset) {
				Ok(_pos) => {} // should never happen
				Err(pos) => assets.insert(pos, new_asset),
			}
		});

		Ok(asset_id)
	}

	fn burn(asset_id: &Self::AssetId) -> dispatch::DispatchResult {
		let owner = Self::account_for_asset(asset_id);

		ensure!(owner != T::AccountId::default(), Error::<T, I>::NonexistentAsset);

		let burn_asset = (*asset_id, <T as Trait<I>>::AssetInfo::default());

		TotalAsset::<I>::mutate(|total| *total -= 1);
		BurnedAsset::<I>::mutate(|total| *total += 1);
		<TotalForAccount<T, I>>::mutate(&owner, |total| *total -= 1);
		<AccountForAsset<T, I>>::remove(&asset_id);
		<AssetsForAccount<T, I>>::mutate(owner, |assets| {
			let pos = assets
				.binary_search(&burn_asset)
				.expect("We already checked that we have the correct owner; qed");
			assets.remove(pos);
		});

		<AssetBalances<T, I>>::remove_prefix(&asset_id);
		<TotalAssetSupply<T, I>>::remove(&asset_id);

		Ok(())
	}

	fn transfer(
		dest_account: &T::AccountId,
		asset_id: &Self::AssetId
	) -> dispatch::DispatchResult {
		let owner = Self::account_for_asset(&asset_id);

		ensure!(owner != T::AccountId::default(), Error::<T, I>::NonexistentAsset);
		ensure!(Self::total_for_account(dest_account) < T::UserAssetLimit::get(), Error::<T, I>::TooManyAssetsForAccount);

		let transfer_asset = (*asset_id, <T as Trait<I>>::AssetInfo::default());

		<TotalForAccount<T, I>>::mutate(&owner, |total| *total -= 1);
		<TotalForAccount<T, I>>::mutate(dest_account, |total| *total += 1);
		<AccountForAsset<T, I>>::insert(&asset_id, &dest_account);

		let asset = <AssetsForAccount<T, I>>::mutate(owner, |assets| {
			let pos = assets
				.binary_search(&transfer_asset)
				.expect("We already checked that we have the correct owner; qed");
			assets.remove(pos)
		});

		<AssetsForAccount<T, I>>::mutate(dest_account, |assets| {
			match assets.binary_search(&asset) {
				Ok(_pos) => {} // should never happen
				Err(pos) => assets.insert(pos, asset),
			}
		});

		Ok(())
	}
}

impl<T: Trait<I>, I: Instance> Token<T::AccountId, T::Balance> for Module<T, I> {
	type AssetId = AssetId<T>;

	fn issue(target: &T::AccountId, asset_id: &Self::AssetId, total: T::Balance) -> dispatch::DispatchResult {
		<AssetBalances<T, I>>::insert(asset_id, target, total.clone());
		<TotalAssetSupply<T, I>>::insert(asset_id, total.clone());

		Ok(())
	}

	fn transfer(origin: &T::AccountId, target: &T::AccountId, asset_id: &Self::AssetId, amount: T::Balance) -> dispatch::DispatchResult {
		let origin_balance = <AssetBalances<T, I>>::get(asset_id, origin.clone());

		ensure!(!amount.is_zero(), Error::<T, I>::TokenAmountZero);
		ensure!(origin_balance >= amount, Error::<T, I>::TokenBalanceLow);

		<AssetBalances<T, I>>::insert(asset_id, origin.clone(), origin_balance - amount.clone());
		<AssetBalances<T, I>>::mutate(asset_id, target, |balance| *balance += amount.clone());

		Ok(())
	}
}
