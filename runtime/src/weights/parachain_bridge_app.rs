// This file is part of the SORA network and Polkaswap app.

// Copyright (c) 2020, 2021, Polka Biome Ltd. All rights reserved.
// SPDX-License-Identifier: BSD-4-Clause

// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:

// Redistributions of source code must retain the above copyright notice, this list
// of conditions and the following disclaimer.
// Redistributions in binary form must reproduce the above copyright notice, this
// list of conditions and the following disclaimer in the documentation and/or other
// materials provided with the distribution.
//
// All advertising materials mentioning features or use of this software must display
// the following acknowledgement: This product includes software developed by Polka Biome
// Ltd., SORA, and Polkaswap.
//
// Neither the name of the Polka Biome Ltd. nor the names of its contributors may be used
// to endorse or promote products derived from this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY Polka Biome Ltd. AS IS AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Polka Biome Ltd. BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
// BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//! Autogenerated weights for `parachain_bridge_app`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `TRX40`, CPU: `AMD Ryzen Threadripper 3960X 24-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// ./target/release/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=parachain_bridge_app
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --output=./runtime/src/weights/parachain_bridge_app.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `parachain_bridge_app`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> parachain_bridge_app::WeightInfo for WeightInfo<T> {
	/// Storage: ParachainBridgeApp AssetKinds (r:1 w:0)
	/// Proof Skipped: ParachainBridgeApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:1 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainBridgeApp AllowedParachainAssets (r:100 w:100)
	/// Proof Skipped: ParachainBridgeApp AllowedParachainAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel MessageQueues (r:1 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel MessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel ChannelNonces (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeOutboundChannel ChannelNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainBridgeApp SidechainPrecision (r:0 w:1)
	/// Proof Skipped: ParachainBridgeApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// The range of component `a` is `[1, 100]`.
	fn register_thischain_asset(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1709`
		//  Estimated: `28522 + a * (2475 ±0)`
		// Minimum execution time: 68_373 nanoseconds.
		Weight::from_parts(66_646_938, 28522)
			// Standard Error: 1_731
			.saturating_add(Weight::from_ref_time(2_540_977).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_proof_size(2475).saturating_mul(a.into()))
	}
	/// Storage: ParachainBridgeApp AssetKinds (r:1 w:0)
	/// Proof Skipped: ParachainBridgeApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:1 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainBridgeApp AllowedParachainAssets (r:100 w:100)
	/// Proof Skipped: ParachainBridgeApp AllowedParachainAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel MessageQueues (r:1 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel MessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel ChannelNonces (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeOutboundChannel ChannelNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainBridgeApp SidechainPrecision (r:0 w:1)
	/// Proof Skipped: ParachainBridgeApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// The range of component `a` is `[1, 100]`.
	fn bind_sidechain_asset(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1709`
		//  Estimated: `28522 + a * (2475 ±0)`
		// Minimum execution time: 68_373 nanoseconds.
		Weight::from_parts(66_646_938, 28522)
			// Standard Error: 1_731
			.saturating_add(Weight::from_ref_time(2_540_977).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_proof_size(2475).saturating_mul(a.into()))
	}
	/// Storage: Technical TechAccounts (r:1 w:1)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets AssetOwners (r:1 w:1)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Owners (r:2 w:2)
	/// Proof Skipped: Permissions Owners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:2 w:1)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainBridgeApp AllowedParachainAssets (r:100 w:100)
	/// Proof Skipped: ParachainBridgeApp AllowedParachainAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel MessageQueues (r:1 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel MessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel ChannelNonces (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeOutboundChannel ChannelNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainBridgeApp SidechainPrecision (r:0 w:1)
	/// Proof Skipped: ParachainBridgeApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:0 w:1)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// The range of component `a` is `[1, 100]`.
	fn register_sidechain_asset(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2186`
		//  Estimated: `42077 + a * (2475 ±0)`
		// Minimum execution time: 143_367 nanoseconds.
		Weight::from_parts(144_031_928, 42077)
			// Standard Error: 2_668
			.saturating_add(Weight::from_ref_time(2_549_037).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(9))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_proof_size(2475).saturating_mul(a.into()))
	}
	/// Storage: ParachainBridgeApp BridgeTransferLimit (r:0 w:1)
	/// Proof Skipped: ParachainBridgeApp BridgeTransferLimit (max_values: Some(1), max_size: None, mode: Measured)
	fn set_transfer_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_380 nanoseconds.
		Weight::from_ref_time(4_530_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ParachainBridgeApp AssetKinds (r:1 w:0)
	/// Proof Skipped: ParachainBridgeApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainBridgeApp AllowedParachainAssets (r:1 w:1)
	/// Proof Skipped: ParachainBridgeApp AllowedParachainAssets (max_values: None, max_size: None, mode: Measured)
	fn add_assetid_paraid() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `5430`
		// Minimum execution time: 13_570 nanoseconds.
		Weight::from_parts(13_741_000, 5430)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ParachainBridgeApp AssetKinds (r:1 w:0)
	/// Proof Skipped: ParachainBridgeApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainBridgeApp AllowedParachainAssets (r:1 w:1)
	/// Proof Skipped: ParachainBridgeApp AllowedParachainAssets (max_values: None, max_size: None, mode: Measured)
	fn remove_assetid_paraid() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `332`
		//  Estimated: `5614`
		// Minimum execution time: 14_990 nanoseconds.
		Weight::from_parts(15_161_000, 5614)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: BridgeProxy Senders (r:1 w:0)
	/// Proof Skipped: BridgeProxy Senders (max_values: None, max_size: None, mode: Measured)
	fn update_transaction_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `2479`
		// Minimum execution time: 12_040 nanoseconds.
		Weight::from_parts(12_410_000, 2479)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: ParachainBridgeApp AssetKinds (r:1 w:0)
	/// Proof Skipped: ParachainBridgeApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainBridgeApp SidechainPrecision (r:1 w:0)
	/// Proof Skipped: ParachainBridgeApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy LockedAssets (r:1 w:1)
	/// Proof Skipped: BridgeProxy LockedAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: BridgeProxy Senders (r:0 w:1)
	/// Proof Skipped: BridgeProxy Senders (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy Transactions (r:0 w:1)
	/// Proof Skipped: BridgeProxy Transactions (max_values: None, max_size: None, mode: Measured)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2422`
		//  Estimated: `37154`
		// Minimum execution time: 93_954 nanoseconds.
		Weight::from_parts(94_634_000, 37154)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: ParachainBridgeApp BridgeTransferLimit (r:1 w:0)
	/// Proof Skipped: ParachainBridgeApp BridgeTransferLimit (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainBridgeApp RelaychainAsset (r:1 w:0)
	/// Proof Skipped: ParachainBridgeApp RelaychainAsset (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainBridgeApp AssetKinds (r:1 w:0)
	/// Proof Skipped: ParachainBridgeApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainBridgeApp SidechainPrecision (r:1 w:0)
	/// Proof Skipped: ParachainBridgeApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy LockedAssets (r:1 w:1)
	/// Proof Skipped: BridgeProxy LockedAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy LimitedAssets (r:1 w:0)
	/// Proof Skipped: BridgeProxy LimitedAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: SubstrateBridgeOutboundChannel MessageQueues (r:1 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel MessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel ChannelNonces (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeOutboundChannel ChannelNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy Senders (r:0 w:1)
	/// Proof Skipped: BridgeProxy Senders (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy Transactions (r:0 w:1)
	/// Proof Skipped: BridgeProxy Transactions (max_values: None, max_size: None, mode: Measured)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2305`
		//  Estimated: `58255`
		// Minimum execution time: 126_336 nanoseconds.
		Weight::from_parts(127_266_000, 58255)
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: ParachainBridgeApp SidechainPrecision (r:1 w:0)
	/// Proof Skipped: ParachainBridgeApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainBridgeApp AssetKinds (r:0 w:1)
	/// Proof Skipped: ParachainBridgeApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	fn finalize_asset_registration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `2889`
		// Minimum execution time: 12_251 nanoseconds.
		Weight::from_parts(12_560_000, 2889)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ParachainBridgeApp AssetKinds (r:1 w:0)
	/// Proof Skipped: ParachainBridgeApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy LockedAssets (r:1 w:1)
	/// Proof Skipped: BridgeProxy LockedAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1908`
		//  Estimated: `20974`
		// Minimum execution time: 60_002 nanoseconds.
		Weight::from_parts(60_823_000, 20974)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
