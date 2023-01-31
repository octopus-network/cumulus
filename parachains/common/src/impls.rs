// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Auxiliary struct/enums for parachain runtimes.
//! Taken from polkadot/runtime/common (at a21cd64) and adapted for parachains.
use crate::impls::fungibles::Inspect;
use cumulus_primitives_core::ParaId;
use frame_support::{
	pallet_prelude::DispatchError,
	traits::{
		fungibles::{self, Balanced, CreditOf, Transfer, Unbalanced},
		tokens::{DepositConsequence, WithdrawConsequence},
		Contains, ContainsPair, Currency, Get, Imbalance, OnUnbalanced, PalletInfoAccess,
	},
};
use sp_core::U256;
use pallet_asset_tx_payment::HandleCredit;
use pallet_dex::MultiAssetIdConverter;
use polkadot_primitives::AccountId;
use sp_runtime::{traits::Zero, DispatchResult};
use sp_std::marker::PhantomData;
use xcm::{
	latest::{AssetId, Fungibility::Fungible, MultiAsset, MultiLocation},
	opaque::lts::{
		Junction,
		Junction::Parachain,
		Junctions::{X2, X3},
	},
};

/// Type alias to conveniently refer to the `Currency::NegativeImbalance` associated type.
pub type NegativeImbalance<T> = <pallet_balances::Pallet<T> as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

/// Type alias to conveniently refer to `frame_system`'s `Config::AccountId`.
pub type AccountIdOf<R> = <R as frame_system::Config>::AccountId;

/// Implementation of `OnUnbalanced` that deposits the fees into a staking pot for later payout.
pub struct ToStakingPot<R>(PhantomData<R>);
impl<R> OnUnbalanced<NegativeImbalance<R>> for ToStakingPot<R>
where
	R: pallet_balances::Config + pallet_collator_selection::Config,
	AccountIdOf<R>: From<polkadot_primitives::AccountId> + Into<polkadot_primitives::AccountId>,
	<R as frame_system::Config>::RuntimeEvent: From<pallet_balances::Event<R>>,
{
	fn on_nonzero_unbalanced(amount: NegativeImbalance<R>) {
		let staking_pot = <pallet_collator_selection::Pallet<R>>::account_id();
		<pallet_balances::Pallet<R>>::resolve_creating(&staking_pot, amount);
	}
}

/// Implementation of `OnUnbalanced` that deals with the fees by combining tip and fee and passing
/// the result on to `ToStakingPot`.
pub struct DealWithFees<R>(PhantomData<R>);
impl<R> OnUnbalanced<NegativeImbalance<R>> for DealWithFees<R>
where
	R: pallet_balances::Config + pallet_collator_selection::Config,
	AccountIdOf<R>: From<polkadot_primitives::AccountId> + Into<polkadot_primitives::AccountId>,
	<R as frame_system::Config>::RuntimeEvent: From<pallet_balances::Event<R>>,
{
	fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance<R>>) {
		if let Some(mut fees) = fees_then_tips.next() {
			if let Some(tips) = fees_then_tips.next() {
				tips.merge_into(&mut fees);
			}
			<ToStakingPot<R> as OnUnbalanced<_>>::on_unbalanced(fees);
		}
	}
}

/// A `HandleCredit` implementation that naively transfers the fees to the block author.
/// Will drop and burn the assets in case the transfer fails.
pub struct AssetsToBlockAuthor<R, I>(PhantomData<(R, I)>);
impl<R, I> HandleCredit<AccountIdOf<R>, pallet_assets::Pallet<R, I>> for AssetsToBlockAuthor<R, I>
where
	I: 'static,
	R: pallet_authorship::Config + pallet_assets::Config<I>,
	AccountIdOf<R>: From<polkadot_primitives::AccountId> + Into<polkadot_primitives::AccountId>,
{
	fn handle_credit(credit: CreditOf<AccountIdOf<R>, pallet_assets::Pallet<R, I>>) {
		if let Some(author) = pallet_authorship::Pallet::<R>::author() {
			// In case of error: Will drop the result triggering the `OnDrop` of the imbalance.
			let _ = pallet_assets::Pallet::<R, I>::resolve(&author, credit);
		}
	}
}

/// Allow checking in assets that have issuance > 0.
pub struct NonZeroIssuance<AccountId, Assets>(PhantomData<(AccountId, Assets)>);
impl<AccountId, Assets> Contains<<Assets as fungibles::Inspect<AccountId>>::AssetId>
	for NonZeroIssuance<AccountId, Assets>
where
	Assets: fungibles::Inspect<AccountId>,
{
	fn contains(id: &<Assets as fungibles::Inspect<AccountId>>::AssetId) -> bool {
		!Assets::total_issuance(*id).is_zero()
	}
}

/// Asset filter that allows all assets from a certain location.
pub struct AssetsFrom<T>(PhantomData<T>);
impl<T: Get<MultiLocation>> ContainsPair<MultiAsset, MultiLocation> for AssetsFrom<T> {
	fn contains(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		let loc = T::get();
		&loc == origin &&
			matches!(asset, MultiAsset { id: AssetId::Concrete(asset_loc), fun: Fungible(_a) }
			if asset_loc.match_and_split(&loc).is_some())
	}
}
/// Whether the multilocation refers to an asset in the local assets pallet or not,
/// and if return the asset id.
fn is_local<SelfParaId: Get<ParaId>, Assets>(multilocation: MultiLocation) -> Option<u32>
where
	Assets: PalletInfoAccess,
{
	if let MultiLocation {
		parents: 1,
		interior:
			X3(
				Parachain(para_id),
				Junction::PalletInstance(pallet_index),
				Junction::GeneralIndex(asset_id),
			),
	} = multilocation
	{
		if ParaId::from(para_id) != SelfParaId::get() {
			None
		} else if pallet_index != <Assets as PalletInfoAccess>::index() as u8 {
			None
		} else {
			<u128 as TryInto<u32>>::try_into(asset_id).ok()
		}
	} else {
		None
	}
}

pub struct MultiLocationConverter<Balances, SelfParaId: Get<ParaId>> {
	_phantom: PhantomData<(Balances, SelfParaId)>,
}

impl<Balances, SelfParaId> MultiAssetIdConverter<MultiLocation, MultiLocation>
	for MultiLocationConverter<Balances, SelfParaId>
where
	Balances: PalletInfoAccess,
	SelfParaId: Get<ParaId>,
{
	fn get_native() -> MultiLocation {
		MultiLocation {
			parents: 1,
			interior: X2(
				Parachain(SelfParaId::get().into()),
				Junction::PalletInstance(<Balances as PalletInfoAccess>::index() as u8),
			),
		}
	}

	fn try_convert(asset: MultiLocation) -> Result<MultiLocation, ()> {
		Ok(asset)
	}

	fn into_multiasset_id(asset: MultiLocation) -> MultiLocation {
		asset
	}
}

pub struct DoubleAsset<Assets, ForeignAssets, SelfParaId> {
	_ignore: PhantomData<(Assets, ForeignAssets, SelfParaId)>,
}

impl<Assets, ForeignAssets, SelfParaId> Unbalanced<AccountId>
	for DoubleAsset<Assets, ForeignAssets, SelfParaId>
where
	SelfParaId: Get<ParaId>,
	ForeignAssets:
		Inspect<AccountId, Balance = u128, AssetId = MultiLocation> + Unbalanced<AccountId>,
	Assets: Inspect<AccountId, Balance = u128, AssetId = u32>
		+ PalletInfoAccess
		+ Unbalanced<AccountId>,
{
	fn set_balance(asset: Self::AssetId, who: &AccountId, amount: Self::Balance) -> DispatchResult {
		if let Some(asset) = is_local::<SelfParaId, Assets>(asset) {
			Assets::set_balance(asset, who, amount)
		} else {
			ForeignAssets::set_balance(asset, who, amount)
		}
	}

	/// Set the total issuance of `asset` to `amount`.
	fn set_total_issuance(asset: Self::AssetId, amount: Self::Balance) {
		if let Some(asset) = is_local::<SelfParaId, Assets>(asset) {
			Assets::set_total_issuance(asset, amount)
		} else {
			ForeignAssets::set_total_issuance(asset, amount)
		}
	}
}

impl<Assets, ForeignAssets, SelfParaId> Inspect<AccountId>
	for DoubleAsset<Assets, ForeignAssets, SelfParaId>
where
	SelfParaId: Get<ParaId>,
	ForeignAssets: Inspect<AccountId, Balance = u128, AssetId = MultiLocation>,
	Assets: Inspect<AccountId, Balance = u128, AssetId = u32> + PalletInfoAccess,
{
	type AssetId = MultiLocation;
	type Balance = u128;

	/// The total amount of issuance in the system.
	fn total_issuance(asset: Self::AssetId) -> Self::Balance {
		if let Some(asset) = is_local::<SelfParaId, Assets>(asset) {
			Assets::total_issuance(asset)
		} else {
			ForeignAssets::total_issuance(asset)
		}
	}

	/// The minimum balance any single account may have.
	fn minimum_balance(asset: Self::AssetId) -> Self::Balance {
		if let Some(asset) = is_local::<SelfParaId, Assets>(asset) {
			Assets::total_issuance(asset)
		} else {
			ForeignAssets::minimum_balance(asset)
		}
	}

	/// Get the `asset` balance of `who`.
	fn balance(asset: Self::AssetId, who: &AccountId) -> Self::Balance {
		if let Some(asset) = is_local::<SelfParaId, Assets>(asset) {
			Assets::balance(asset, who)
		} else {
			ForeignAssets::balance(asset, who)
		}
	}

	/// Get the maximum amount of `asset` that `who` can withdraw/transfer successfully.
	fn reducible_balance(asset: Self::AssetId, who: &AccountId, keep_alive: bool) -> Self::Balance {
		if let Some(asset) = is_local::<SelfParaId, Assets>(asset) {
			Assets::reducible_balance(asset, who, keep_alive)
		} else {
			ForeignAssets::reducible_balance(asset, who, keep_alive)
		}
	}

	/// Returns `true` if the `asset` balance of `who` may be increased by `amount`.
	///
	/// - `asset`: The asset that should be deposited.
	/// - `who`: The account of which the balance should be increased by `amount`.
	/// - `amount`: How much should the balance be increased?
	/// - `mint`: Will `amount` be minted to deposit it into `account`?
	fn can_deposit(
		asset: Self::AssetId,
		who: &AccountId,
		amount: Self::Balance,
		mint: bool,
	) -> DepositConsequence {
		if let Some(asset) = is_local::<SelfParaId, Assets>(asset) {
			Assets::can_deposit(asset, who, amount, mint)
		} else {
			ForeignAssets::can_deposit(asset, who, amount, mint)
		}
	}

	/// Returns `Failed` if the `asset` balance of `who` may not be decreased by `amount`, otherwise
	/// the consequence.
	fn can_withdraw(
		asset: Self::AssetId,
		who: &AccountId,
		amount: Self::Balance,
	) -> WithdrawConsequence<Self::Balance> {
		if let Some(asset) = is_local::<SelfParaId, Assets>(asset) {
			Assets::can_withdraw(asset, who, amount)
		} else {
			ForeignAssets::can_withdraw(asset, who, amount)
		}
	}

	/// Returns `true` if an `asset` exists.
	fn asset_exists(asset: Self::AssetId) -> bool {
		if let Some(asset) = is_local::<SelfParaId, Assets>(asset) {
			Assets::asset_exists(asset)
		} else {
			ForeignAssets::asset_exists(asset)
		}
	}
}

impl<Assets, ForeignAssets, SelfParaId> Transfer<AccountId>
	for DoubleAsset<Assets, ForeignAssets, SelfParaId>
where
	SelfParaId: Get<ParaId>,
	ForeignAssets: Transfer<AccountId, Balance = u128>
		+ Inspect<AccountId, Balance = u128, AssetId = MultiLocation>,
	Assets:
		Transfer<AccountId> + Inspect<AccountId, Balance = u128, AssetId = u32> + PalletInfoAccess,
{
	/// Transfer funds from one account into another.
	fn transfer(
		asset: MultiLocation,
		source: &AccountId,
		dest: &AccountId,
		amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError> {
		if let Some(asset_id) = is_local::<SelfParaId, Assets>(asset) {
			Assets::transfer(asset_id, source, dest, amount, keep_alive)
		} else {
			ForeignAssets::transfer(asset, source, dest, amount, keep_alive)
		}
	}
}


// TODO: remove this wrapper once https://github.com/paritytech/parity-common/pull/716 is merged.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct WrappedU256(sp_core::U256);

impl From<u128> for WrappedU256 {
	fn from(val: u128) -> Self {
		Self(U256::from(val))
	}
}
impl From<u64> for WrappedU256 {
	fn from(val: u64) -> Self {
		Self(U256::from(val))
	}
}
impl From<u32> for WrappedU256 {
	fn from(val: u32) -> Self {
		Self(U256::from(val))
	}
}

impl TryInto<u128> for WrappedU256 {
	type Error = &'static str;
	fn try_into(self) -> Result<u128, <Self as TryInto<u128>>::Error> {
		U256::try_into(self.0)
	}
}

impl sp_runtime::traits::IntegerSquareRoot for WrappedU256 {
	fn integer_sqrt_checked(&self) -> Option<WrappedU256> {
		self.0.integer_sqrt_checked().map(|val| Self(val))
	}
}

impl sp_runtime::traits::One for WrappedU256 {
	fn one() -> Self {
		Self(U256::one())
	}
}

impl sp_runtime::traits::Zero for WrappedU256 {
	fn zero() -> Self {
		Self(U256::zero())
	}
	fn is_zero(&self) -> bool {
		self.0.is_zero()
	}
}

impl sp_std::ops::Mul for WrappedU256 {
	type Output = WrappedU256;
	fn mul(self, other: WrappedU256) -> <Self as sp_std::ops::Mul<WrappedU256>>::Output {
		Self(self.0.mul(other.0))
	}
}
impl sp_std::ops::Add for WrappedU256 {
	type Output = WrappedU256;
	fn add(self, other: WrappedU256) -> <Self as sp_std::ops::Add<WrappedU256>>::Output {
		Self(self.0.add(other.0))
	}
}
impl sp_std::ops::Sub for WrappedU256 {
	type Output = WrappedU256;
	fn sub(self, other: WrappedU256) -> <Self as sp_std::ops::Sub<WrappedU256>>::Output {
		Self(self.0.sub(other.0))
	}
}
impl sp_std::ops::Div for WrappedU256 {
	type Output = WrappedU256;
	fn div(self, other: WrappedU256) -> <Self as sp_std::ops::Div<WrappedU256>>::Output {
		Self(self.0.div(other.0))
	}
}
impl sp_runtime::traits::CheckedAdd for WrappedU256 {
	fn checked_add(&self, other: &WrappedU256) -> Option<WrappedU256> {
		self.0.checked_add(other.0).map(|res| Self(res))
	}
}
impl sp_runtime::traits::CheckedSub for WrappedU256 {
	fn checked_sub(&self, other: &WrappedU256) -> Option<WrappedU256> {
		self.0.checked_sub(other.0).map(|res| Self(res))
	}
}
impl sp_runtime::traits::CheckedMul for WrappedU256 {
	fn checked_mul(&self, other: &WrappedU256) -> Option<WrappedU256> {
		self.0.checked_mul(other.0).map(|res| Self(res))
	}
}
impl sp_runtime::traits::CheckedDiv for WrappedU256 {
	fn checked_div(&self, other: &WrappedU256) -> Option<WrappedU256> {
		self.0.checked_div(other.0).map(|res| Self(res))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use frame_support::{
		parameter_types,
		traits::{FindAuthor, ValidatorRegistration},
		PalletId,
	};
	use frame_system::{limits, EnsureRoot};
	use pallet_collator_selection::IdentityCollator;
	use polkadot_primitives::AccountId;
	use sp_core::H256;
	use sp_runtime::{
		testing::Header,
		traits::{BlakeTwo256, IdentityLookup},
		Perbill,
	};
	use xcm::prelude::*;

	type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
	type Block = frame_system::mocking::MockBlock<Test>;
	const TEST_ACCOUNT: AccountId = AccountId::new([1; 32]);

	frame_support::construct_runtime!(
		pub enum Test where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
		{
			System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
			Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
			CollatorSelection: pallet_collator_selection::{Pallet, Call, Storage, Event<T>},
		}
	);

	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub BlockLength: limits::BlockLength = limits::BlockLength::max(2 * 1024);
		pub const AvailableBlockRatio: Perbill = Perbill::one();
		pub const MaxReserves: u32 = 50;
	}

	impl frame_system::Config for Test {
		type BaseCallFilter = frame_support::traits::Everything;
		type RuntimeOrigin = RuntimeOrigin;
		type Index = u64;
		type BlockNumber = u64;
		type RuntimeCall = RuntimeCall;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = AccountId;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type RuntimeEvent = RuntimeEvent;
		type BlockHashCount = BlockHashCount;
		type BlockLength = BlockLength;
		type BlockWeights = ();
		type DbWeight = ();
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = pallet_balances::AccountData<u64>;
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
		type MaxConsumers = frame_support::traits::ConstU32<16>;
	}

	impl pallet_balances::Config for Test {
		type Balance = u64;
		type RuntimeEvent = RuntimeEvent;
		type DustRemoval = ();
		type ExistentialDeposit = ();
		type AccountStore = System;
		type MaxLocks = ();
		type WeightInfo = ();
		type MaxReserves = MaxReserves;
		type ReserveIdentifier = [u8; 8];
	}

	pub struct OneAuthor;
	impl FindAuthor<AccountId> for OneAuthor {
		fn find_author<'a, I>(_: I) -> Option<AccountId>
		where
			I: 'a,
		{
			Some(TEST_ACCOUNT)
		}
	}

	pub struct IsRegistered;
	impl ValidatorRegistration<AccountId> for IsRegistered {
		fn is_registered(_id: &AccountId) -> bool {
			true
		}
	}

	parameter_types! {
		pub const PotId: PalletId = PalletId(*b"PotStake");
		pub const MaxCandidates: u32 = 20;
		pub const MaxInvulnerables: u32 = 20;
		pub const MinCandidates: u32 = 1;
	}

	impl pallet_collator_selection::Config for Test {
		type RuntimeEvent = RuntimeEvent;
		type Currency = Balances;
		type UpdateOrigin = EnsureRoot<AccountId>;
		type PotId = PotId;
		type MaxCandidates = MaxCandidates;
		type MinCandidates = MinCandidates;
		type MaxInvulnerables = MaxInvulnerables;
		type ValidatorId = <Self as frame_system::Config>::AccountId;
		type ValidatorIdOf = IdentityCollator;
		type ValidatorRegistration = IsRegistered;
		type KickThreshold = ();
		type WeightInfo = ();
	}

	impl pallet_authorship::Config for Test {
		type FindAuthor = OneAuthor;
		type UncleGenerations = ();
		type FilterUncle = ();
		type EventHandler = ();
	}

	pub fn new_test_ext() -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		// We use default for brevity, but you can configure as desired if needed.
		pallet_balances::GenesisConfig::<Test>::default()
			.assimilate_storage(&mut t)
			.unwrap();
		t.into()
	}

	#[test]
	fn test_fees_and_tip_split() {
		new_test_ext().execute_with(|| {
			let fee = Balances::issue(10);
			let tip = Balances::issue(20);

			assert_eq!(Balances::free_balance(TEST_ACCOUNT), 0);

			DealWithFees::on_unbalanceds(vec![fee, tip].into_iter());

			// Author gets 100% of tip and 100% of fee = 30
			assert_eq!(Balances::free_balance(CollatorSelection::account_id()), 30);
		});
	}

	#[test]
	fn assets_from_filters_correctly() {
		parameter_types! {
			pub SomeSiblingParachain: MultiLocation = MultiLocation::new(1, X1(Parachain(1234)));
		}

		let asset_location = SomeSiblingParachain::get()
			.clone()
			.pushed_with_interior(GeneralIndex(42))
			.expect("multilocation will only have 2 junctions; qed");
		let asset = MultiAsset { id: Concrete(asset_location), fun: 1_000_000u128.into() };
		assert!(
			AssetsFrom::<SomeSiblingParachain>::contains(&asset, &SomeSiblingParachain::get()),
			"AssetsFrom should allow assets from any of its interior locations"
		);
	}
}
