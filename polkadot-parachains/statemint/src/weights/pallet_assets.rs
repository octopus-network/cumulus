// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_assets`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("statemint-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-collator
// benchmark
// pallet
// --chain=statemint-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_assets
// --extrinsic=*
// --steps=50
// --repeat=20
// --json-file=./bench-statemint.json
// --header=./file_header.txt
// --output=./polkadot-parachains/statemint/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_assets`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_assets::WeightInfo for WeightInfo<T> {
	// Storage: Assets Asset (r:1 w:1)
	fn create() -> Weight {
		(21_404_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn force_create() -> Weight {
		(11_941_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:5002 w:5001)
	// Storage: System Account (r:5000 w:5000)
	// Storage: Assets Metadata (r:1 w:0)
	// Storage: Assets Approvals (r:501 w:500)
	fn destroy(c: u32, s: u32, a: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 38_000
			.saturating_add((13_682_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 38_000
			.saturating_add((16_373_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 385_000
			.saturating_add((17_884_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	fn mint() -> Weight {
		(23_749_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	fn burn() -> Weight {
		(27_537_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		(39_816_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_keep_alive() -> Weight {
		(33_679_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn force_transfer() -> Weight {
		(40_031_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Account (r:1 w:1)
	fn freeze() -> Weight {
		(15_521_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Account (r:1 w:1)
	fn thaw() -> Weight {
		(15_355_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn freeze_asset() -> Weight {
		(12_528_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn thaw_asset() -> Weight {
		(12_321_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:0)
	fn transfer_ownership() -> Weight {
		(13_330_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn set_team() -> Weight {
		(12_340_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	fn set_metadata(_n: u32, s: u32, ) -> Weight {
		(25_114_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	fn clear_metadata() -> Weight {
		(24_841_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	fn force_set_metadata(_n: u32, s: u32, ) -> Weight {
		(13_924_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	fn force_clear_metadata() -> Weight {
		(25_548_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn force_asset_status() -> Weight {
		(12_271_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	fn approve_transfer() -> Weight {
		(28_166_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Assets Approvals (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_approved() -> Weight {
		(51_760_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	fn cancel_approval() -> Weight {
		(28_702_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	fn force_cancel_approval() -> Weight {
		(29_913_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
