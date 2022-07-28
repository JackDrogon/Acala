// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_honzon
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_honzon.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_honzon::WeightInfo for WeightInfo<T> {
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Honzon Authorization (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	fn authorize() -> Weight {
		(30_023_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Honzon Authorization (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	fn unauthorize() -> Weight {
		(31_449_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:0)
	// Storage: Honzon Authorization (r:0 w:1)
	fn unauthorize_all(c: u32, ) -> Weight {
		(22_604_000 as Weight)
			// Standard Error: 408_000
			.saturating_add((4_535_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Storage: Loans Positions (r:1 w:1)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: System Account (r:1 w:1)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	fn adjust_loan() -> Weight {
		(112_387_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(18 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Storage: Honzon Authorization (r:1 w:0)
	// Storage: Loans Positions (r:2 w:2)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Storage: Rewards SharesAndWithdrawnRewards (r:2 w:2)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Loans TotalPositions (r:1 w:1)
	fn transfer_loan_from() -> Weight {
		(87_216_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(17 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Storage: Loans Positions (r:1 w:1)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Storage: Tokens Accounts (r:6 w:6)
	// Storage: System Account (r:3 w:2)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: CdpTreasury DebitPool (r:1 w:1)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Storage: AuctionManager TotalCollateralInAuction (r:1 w:0)
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Storage: Dex LiquidityPool (r:2 w:2)
	// Storage: StableAsset Pools (r:1 w:0)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	fn close_loan_has_debit_by_dex() -> Weight {
		(259_295_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(36 as Weight))
			.saturating_add(T::DbWeight::get().writes(16 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:2 w:1)
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Storage: Dex LiquidityPool (r:3 w:2)
	// Storage: StableAsset Pools (r:1 w:0)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Storage: Loans Positions (r:1 w:1)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	fn expand_position_collateral() -> Weight {
		(199_525_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(13 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Storage: Loans Positions (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Storage: Dex LiquidityPool (r:3 w:2)
	// Storage: StableAsset Pools (r:1 w:0)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Storage: Tokens Accounts (r:5 w:5)
	// Storage: System Account (r:2 w:1)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn shrink_position_debit() -> Weight {
		(197_554_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(23 as Weight))
			.saturating_add(T::DbWeight::get().writes(14 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Storage: CdpEngine CollateralParams (r:2 w:0)
	// Storage: Loans Positions (r:2 w:2)
	// Storage: Loans TotalPositions (r:2 w:2)
	// Storage: CdpEngine DebitExchangeRate (r:2 w:0)
	// Storage: Prices LockedPrice (r:3 w:0)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	fn transfer_debit() -> Weight {
		(137_148_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(21 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: Loans Positions (r:1 w:0)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	fn precompile_get_current_collateral_ratio() -> Weight {
		(32_314_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
	}
}
