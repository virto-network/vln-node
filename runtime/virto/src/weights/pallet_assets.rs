// Copyright Parity Technologies (UK) Ltd.
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
//! DATE: 2023-06-20, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("asset-hub-kusama-dev"), DB CACHE: 1024

// Executed Command:
// target/production/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/var/lib/gitlab-runner/builds/zyw4fam_/0/parity/mirrors/cumulus/.git/.artifacts/bench.json
// --pallet=pallet_assets
// --chain=asset-hub-kusama-dev
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/asset-hub-kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_assets`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_assets::WeightInfo for WeightInfo<T> {
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3675`
		// Minimum execution time: 26_803_000 picoseconds.
		Weight::from_parts(27_212_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn force_create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3675`
		// Minimum execution time: 11_413_000 picoseconds.
		Weight::from_parts(11_755_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn start_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277`
		//  Estimated: `3675`
		// Minimum execution time: 13_649_000 picoseconds.
		Weight::from_parts(13_992_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1001 w:1000)
	/// Proof: Assets Account (max_values: None, max_size: Some(134), added: 2609, mode: MaxEncodedLen)
	/// Storage: System Account (r:1000 w:1000)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `c` is `[0, 1000]`.
	/// The range of component `c` is `[0, 1000]`.
	fn destroy_accounts(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + c * (208 ±0)`
		//  Estimated: `3675 + c * (2609 ±0)`
		// Minimum execution time: 16_659_000 picoseconds.
		Weight::from_parts(16_877_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			// Standard Error: 4_837
			.saturating_add(Weight::from_parts(12_015_579, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2609).saturating_mul(c.into()))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Approvals (r:1001 w:1000)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2623, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy_approvals(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `414 + a * (86 ±0)`
		//  Estimated: `3675 + a * (2623 ±0)`
		// Minimum execution time: 17_700_000 picoseconds.
		Weight::from_parts(17_937_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			// Standard Error: 3_243
			.saturating_add(Weight::from_parts(13_846_653, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 2623).saturating_mul(a.into()))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:0)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	fn finish_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `243`
		//  Estimated: `3675`
		// Minimum execution time: 13_768_000 picoseconds.
		Weight::from_parts(14_051_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(134), added: 2609, mode: MaxEncodedLen)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `243`
		//  Estimated: `3675`
		// Minimum execution time: 25_151_000 picoseconds.
		Weight::from_parts(25_580_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(134), added: 2609, mode: MaxEncodedLen)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `351`
		//  Estimated: `3675`
		// Minimum execution time: 30_470_000 picoseconds.
		Weight::from_parts(31_052_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(134), added: 2609, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `351`
		//  Estimated: `6208`
		// Minimum execution time: 41_875_000 picoseconds.
		Weight::from_parts(42_493_000, 0)
			.saturating_add(Weight::from_parts(0, 6208))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(134), added: 2609, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `351`
		//  Estimated: `6208`
		// Minimum execution time: 36_879_000 picoseconds.
		Weight::from_parts(38_073_000, 0)
			.saturating_add(Weight::from_parts(0, 6208))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(134), added: 2609, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `351`
		//  Estimated: `6208`
		// Minimum execution time: 41_787_000 picoseconds.
		Weight::from_parts(42_372_000, 0)
			.saturating_add(Weight::from_parts(0, 6208))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(134), added: 2609, mode: MaxEncodedLen)
	fn freeze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `351`
		//  Estimated: `3675`
		// Minimum execution time: 16_915_000 picoseconds.
		Weight::from_parts(17_359_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(134), added: 2609, mode: MaxEncodedLen)
	fn thaw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `351`
		//  Estimated: `3675`
		// Minimum execution time: 16_837_000 picoseconds.
		Weight::from_parts(17_598_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn freeze_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277`
		//  Estimated: `3675`
		// Minimum execution time: 13_493_000 picoseconds.
		Weight::from_parts(13_959_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn thaw_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277`
		//  Estimated: `3675`
		// Minimum execution time: 13_178_000 picoseconds.
		Weight::from_parts(13_574_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:0)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `243`
		//  Estimated: `3675`
		// Minimum execution time: 14_461_000 picoseconds.
		Weight::from_parts(14_999_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn set_team() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `243`
		//  Estimated: `3675`
		// Minimum execution time: 13_465_000 picoseconds.
		Weight::from_parts(13_898_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn set_metadata(n: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `243`
		//  Estimated: `3675`
		// Minimum execution time: 28_106_000 picoseconds.
		Weight::from_parts(28_109_577, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			// Standard Error: 4_194
			.saturating_add(Weight::from_parts(15_049, 0).saturating_mul(n.into()))
			// Standard Error: 4_194
			.saturating_add(Weight::from_parts(19_600, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `407`
		//  Estimated: `3675`
		// Minimum execution time: 28_488_000 picoseconds.
		Weight::from_parts(28_852_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn force_set_metadata(_n: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `82`
		//  Estimated: `3675`
		// Minimum execution time: 13_076_000 picoseconds.
		Weight::from_parts(13_553_158, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			// Standard Error: 1_371
			.saturating_add(Weight::from_parts(5_010, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	fn force_clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `407`
		//  Estimated: `3675`
		// Minimum execution time: 27_960_000 picoseconds.
		Weight::from_parts(28_488_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn force_asset_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `243`
		//  Estimated: `3675`
		// Minimum execution time: 12_685_000 picoseconds.
		Weight::from_parts(12_972_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2623, mode: MaxEncodedLen)
	fn approve_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277`
		//  Estimated: `3675`
		// Minimum execution time: 31_166_000 picoseconds.
		Weight::from_parts(31_861_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2623, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(134), added: 2609, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `521`
		//  Estimated: `6208`
		// Minimum execution time: 59_372_000 picoseconds.
		Weight::from_parts(60_201_000, 0)
			.saturating_add(Weight::from_parts(0, 6208))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2623, mode: MaxEncodedLen)
	fn cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `447`
		//  Estimated: `3675`
		// Minimum execution time: 33_161_000 picoseconds.
		Weight::from_parts(33_561_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2623, mode: MaxEncodedLen)
	fn force_cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `447`
		//  Estimated: `3675`
		// Minimum execution time: 33_624_000 picoseconds.
		Weight::from_parts(34_111_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn set_min_balance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `243`
		//  Estimated: `3675`
		// Minimum execution time: 13_756_000 picoseconds.
		Weight::from_parts(14_098_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(134), added: 2609, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn touch() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346`
		//  Estimated: `3675`
		// Minimum execution time: 32_493_000 picoseconds.
		Weight::from_parts(33_272_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(134), added: 2609, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn touch_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `243`
		//  Estimated: `3675`
		// Minimum execution time: 30_824_000 picoseconds.
		Weight::from_parts(31_497_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(134), added: 2609, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `472`
		//  Estimated: `3675`
		// Minimum execution time: 29_576_000 picoseconds.
		Weight::from_parts(30_149_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(134), added: 2609, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn refund_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `402`
		//  Estimated: `3675`
		// Minimum execution time: 28_092_000 picoseconds.
		Weight::from_parts(28_419_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(134), added: 2609, mode: MaxEncodedLen)
	fn block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `351`
		//  Estimated: `3675`
		// Minimum execution time: 16_860_000 picoseconds.
		Weight::from_parts(17_197_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}