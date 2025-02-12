
//! Autogenerated weights for pallet_dao_membership
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/societal-node
// benchmark
// pallet
// --chain
// dev
// --steps
// 50
// --repeat
// 20
// --pallet
// pallet_dao_membership
// --extrinsic
// *
// --execution
// wasm
// --wasm-execution
// compiled
// --template
// ./.maintain/frame-weight-template.hbs
// --output
// ./pallets/dao-membership/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_dao_membership.
pub trait WeightInfo {
	fn add_member() -> Weight;
	fn remove_member() -> Weight;
	fn swap_member() -> Weight;
	fn reset_members() -> Weight;
	fn change_key() -> Weight;
}

/// Weights for pallet_dao_membership using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Proposals (r:1 w:0)
	/// Proof: DaoCouncil Proposals (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn add_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6786`
		//  Estimated: `16983`
		// Minimum execution time: 26_000_000 picoseconds.
		Weight::from_parts(27_000_000, 16983)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Proposals (r:1 w:0)
	/// Proof: DaoCouncil Proposals (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn remove_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6786`
		//  Estimated: `16983`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(26_000_000, 16983)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Proposals (r:1 w:0)
	/// Proof: DaoCouncil Proposals (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn swap_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6786`
		//  Estimated: `16983`
		// Minimum execution time: 26_000_000 picoseconds.
		Weight::from_parts(27_000_000, 16983)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Proposals (r:1 w:0)
	/// Proof: DaoCouncil Proposals (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn reset_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6914`
		//  Estimated: `16983`
		// Minimum execution time: 33_000_000 picoseconds.
		Weight::from_parts(34_000_000, 16983)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Proposals (r:1 w:0)
	/// Proof: DaoCouncil Proposals (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn change_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6635`
		//  Estimated: `13358`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(22_000_000, 13358)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Proposals (r:1 w:0)
	/// Proof: DaoCouncil Proposals (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn add_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6786`
		//  Estimated: `16983`
		// Minimum execution time: 26_000_000 picoseconds.
		Weight::from_parts(27_000_000, 16983)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Proposals (r:1 w:0)
	/// Proof: DaoCouncil Proposals (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn remove_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6786`
		//  Estimated: `16983`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(26_000_000, 16983)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Proposals (r:1 w:0)
	/// Proof: DaoCouncil Proposals (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn swap_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6786`
		//  Estimated: `16983`
		// Minimum execution time: 26_000_000 picoseconds.
		Weight::from_parts(27_000_000, 16983)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Proposals (r:1 w:0)
	/// Proof: DaoCouncil Proposals (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn reset_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6914`
		//  Estimated: `16983`
		// Minimum execution time: 33_000_000 picoseconds.
		Weight::from_parts(34_000_000, 16983)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Proposals (r:1 w:0)
	/// Proof: DaoCouncil Proposals (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn change_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6635`
		//  Estimated: `13358`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(22_000_000, 13358)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
