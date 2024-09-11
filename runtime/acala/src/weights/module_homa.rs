// This file is part of Acala.

// Copyright (C) 2020-2024 Acala Foundation.
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

//! Autogenerated weights for module_homa
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 35.0.1
//! DATE: 2024-04-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-41-141`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_homa.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_homa::WeightInfo for WeightInfo<T> {
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::LastEraBumpedBlock` (r:1 w:0)
	// Proof: `Homa::LastEraBumpedBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::BumpEraFrequency` (r:1 w:0)
	// Proof: `Homa::BumpEraFrequency` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `823`
		//  Estimated: `2308`
		// Minimum execution time: 6_283 nanoseconds.
		Weight::from_parts(6_483_000, 2308)
			.saturating_add(T::DbWeight::get().reads(4))
	}
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::LastEraBumpedBlock` (r:1 w:1)
	// Proof: `Homa::LastEraBumpedBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::BumpEraFrequency` (r:1 w:0)
	// Proof: `Homa::BumpEraFrequency` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::RelayChainCurrentEra` (r:1 w:1)
	// Proof: `Homa::RelayChainCurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::EstimatedRewardRatePerEra` (r:1 w:0)
	// Proof: `Homa::EstimatedRewardRatePerEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::StakingLedgers` (r:2 w:1)
	// Proof: `Homa::StakingLedgers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::TotalStakingBonded` (r:1 w:1)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::CommissionRate` (r:1 w:0)
	// Proof: `Homa::CommissionRate` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:2 w:2)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:3 w:3)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `XcmInterface::XcmDestWeightAndFee` (r:4 w:0)
	// Proof: `XcmInterface::XcmDestWeightAndFee` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::UnclaimedRedemption` (r:1 w:1)
	// Proof: `Homa::UnclaimedRedemption` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::ToBondPool` (r:1 w:1)
	// Proof: `Homa::ToBondPool` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::SoftBondedCapPerSubAccount` (r:1 w:0)
	// Proof: `Homa::SoftBondedCapPerSubAccount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `EVM::XcmOrigin` (r:1 w:1)
	// Proof: `EVM::XcmOrigin` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `UnknownTokens::ConcreteFungibleBalances` (r:1 w:0)
	// Proof: `UnknownTokens::ConcreteFungibleBalances` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::RedeemRequests` (r:2 w:1)
	// Proof: `Homa::RedeemRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::Unbondings` (r:1 w:1)
	// Proof: `Homa::Unbondings` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::TotalVoidLiquid` (r:0 w:1)
	// Proof: `Homa::TotalVoidLiquid` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_initialize_with_bump_era(n: u32,) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2961`
		//  Estimated: `13851`
		// Minimum execution time: 298_418 nanoseconds.
		Weight::from_parts(305_164_000, 13851)
			.saturating_add(T::DbWeight::get().reads(32))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(18))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(n.into())))
	}
	// Storage: `Homa::TotalStakingBonded` (r:1 w:0)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::ToBondPool` (r:1 w:1)
	// Proof: `Homa::ToBondPool` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::SoftBondedCapPerSubAccount` (r:1 w:0)
	// Proof: `Homa::SoftBondedCapPerSubAccount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:3 w:3)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalVoidLiquid` (r:1 w:1)
	// Proof: `Homa::TotalVoidLiquid` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::EstimatedRewardRatePerEra` (r:1 w:0)
	// Proof: `Homa::EstimatedRewardRatePerEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1701`
		//  Estimated: `8856`
		// Minimum execution time: 59_996 nanoseconds.
		Weight::from_parts(61_537_000, 8856)
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: `Homa::RedeemRequests` (r:1 w:1)
	// Proof: `Homa::RedeemRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn request_redeem() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1610`
		//  Estimated: `6234`
		// Minimum execution time: 41_569 nanoseconds.
		Weight::from_parts(42_722_000, 6234)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: `Homa::RedeemRequests` (r:50 w:50)
	// Proof: `Homa::RedeemRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::ToBondPool` (r:1 w:1)
	// Proof: `Homa::ToBondPool` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::TotalStakingBonded` (r:1 w:0)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalVoidLiquid` (r:1 w:0)
	// Proof: `Homa::TotalVoidLiquid` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::FastMatchFeeRate` (r:1 w:0)
	// Proof: `Homa::FastMatchFeeRate` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:52 w:52)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:51 w:51)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 50]`.
	fn fast_match_redeems(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1950 + n * (295 ±0)`
		//  Estimated: `6234 + n * (2770 ±0)`
		// Minimum execution time: 73_092 nanoseconds.
		Weight::from_parts(25_412_943, 6234)
			// Standard Error: 28_320
			.saturating_add(Weight::from_parts(41_844_237, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2770).saturating_mul(n.into()))
	}
	// Storage: `Homa::RelayChainCurrentEra` (r:1 w:0)
	// Proof: `Homa::RelayChainCurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::Unbondings` (r:2 w:1)
	// Proof: `Homa::Unbondings` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::UnclaimedRedemption` (r:1 w:1)
	// Proof: `Homa::UnclaimedRedemption` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	fn claim_redemption() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2021`
		//  Estimated: `7961`
		// Minimum execution time: 59_467 nanoseconds.
		Weight::from_parts(61_252_000, 7961)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: `Homa::EstimatedRewardRatePerEra` (r:1 w:1)
	// Proof: `Homa::EstimatedRewardRatePerEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::CommissionRate` (r:1 w:1)
	// Proof: `Homa::CommissionRate` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::FastMatchFeeRate` (r:1 w:1)
	// Proof: `Homa::FastMatchFeeRate` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::SoftBondedCapPerSubAccount` (r:0 w:1)
	// Proof: `Homa::SoftBondedCapPerSubAccount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn update_homa_params() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1139`
		//  Estimated: `2624`
		// Minimum execution time: 20_902 nanoseconds.
		Weight::from_parts(21_722_000, 2624)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::LastEraBumpedBlock` (r:0 w:1)
	// Proof: `Homa::LastEraBumpedBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::BumpEraFrequency` (r:0 w:1)
	// Proof: `Homa::BumpEraFrequency` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn update_bump_era_params() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1211`
		//  Estimated: `2696`
		// Minimum execution time: 18_032 nanoseconds.
		Weight::from_parts(18_556_000, 2696)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: `Homa::StakingLedgers` (r:10 w:10)
	// Proof: `Homa::StakingLedgers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::TotalStakingBonded` (r:1 w:1)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `n` is `[0, 10]`.
	fn reset_ledgers(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1139`
		//  Estimated: `2624 + n * (2475 ±0)`
		// Minimum execution time: 4_170 nanoseconds.
		Weight::from_parts(10_347_994, 2624)
			// Standard Error: 35_868
			.saturating_add(Weight::from_parts(5_696_010, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2475).saturating_mul(n.into()))
	}
	// Storage: `Homa::RelayChainCurrentEra` (r:1 w:1)
	// Proof: `Homa::RelayChainCurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn reset_current_era() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1139`
		//  Estimated: `2624`
		// Minimum execution time: 13_506 nanoseconds.
		Weight::from_parts(13_876_000, 2624)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
