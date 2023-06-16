
//! Autogenerated weights for pallet_dao
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_dao
// --extrinsic
// *
// --execution
// wasm
// --wasm-execution
// compiled
// --template
// ./.maintain/frame-weight-template.hbs
// --output
// ./pallets/dao/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_dao.
pub trait WeightInfo {
	fn create_dao() -> Weight;
	fn approve_dao() -> Weight;
	fn update_dao_metadata() -> Weight;
	fn update_dao_policy() -> Weight;
	fn mint_dao_token() -> Weight;
	fn spend_dao_funds() -> Weight;
	fn launch_dao_referendum() -> Weight;
	fn bake_dao_referendum() -> Weight;
}

/// Weights for pallet_dao using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Dao NextDaoId (r:1 w:1)
	/// Proof: Dao NextDaoId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommitteeMembers Members (r:1 w:1)
	/// Proof: DaoTechnicalCommitteeMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:3 w:3)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:0 w:1)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Dao Policies (r:0 w:1)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommittee Members (r:0 w:1)
	/// Proof: DaoTechnicalCommittee Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn create_dao() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `121780`
		// Minimum execution time: 171_000_000 picoseconds.
		Weight::from_parts(173_000_000, 121780)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(16_u64))
	}
	/// Storage: Dao PendingDaos (r:1 w:1)
	/// Proof: Dao PendingDaos (max_values: None, max_size: Some(7639), added: 10114, mode: MaxEncodedLen)
	/// Storage: Dao NextDaoId (r:1 w:1)
	/// Proof: Dao NextDaoId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:3 w:3)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:0 w:1)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Dao Policies (r:0 w:1)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn approve_dao() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1436`
		//  Estimated: `111697`
		// Minimum execution time: 59_000_000 picoseconds.
		Weight::from_parts(62_000_000, 111697)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(11_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:1 w:1)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	fn update_dao_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1189`
		//  Estimated: `8153`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_000_000, 8153)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Dao Policies (r:1 w:1)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:3 w:3)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:4 w:4)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn update_dao_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `838`
		//  Estimated: `178926`
		// Minimum execution time: 92_000_000 picoseconds.
		Weight::from_parts(93_000_000, 178926)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:1 w:0)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	fn mint_dao_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1672`
		//  Estimated: `15451`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(31_000_000, 15451)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DaoTreasury Approvals (r:1 w:1)
	/// Proof: DaoTreasury Approvals (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	/// Storage: DaoBounties BountyApprovals (r:1 w:1)
	/// Proof: DaoBounties BountyApprovals (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	fn spend_dao_funds() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `751`
		//  Estimated: `14976`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(26_000_000, 14976)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy LastTabledWasExternal (r:1 w:0)
	/// Proof: DaoDemocracy LastTabledWasExternal (max_values: None, max_size: Some(13), added: 2488, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy NextExternal (r:1 w:0)
	/// Proof: DaoDemocracy NextExternal (max_values: None, max_size: Some(144), added: 2619, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy PublicProps (r:1 w:1)
	/// Proof: DaoDemocracy PublicProps (max_values: None, max_size: Some(8363), added: 10838, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy DepositOf (r:1 w:1)
	/// Proof: DaoDemocracy DepositOf (max_values: None, max_size: Some(3242), added: 5717, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:1 w:0)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumCount (r:1 w:1)
	/// Proof: DaoDemocracy ReferendumCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumInfoOf (r:0 w:1)
	/// Proof: DaoDemocracy ReferendumInfoOf (max_values: None, max_size: Some(213), added: 2688, mode: MaxEncodedLen)
	fn launch_dao_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2106`
		//  Estimated: `44554`
		// Minimum execution time: 58_000_000 picoseconds.
		Weight::from_parts(60_000_000, 44554)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy LowestUnbaked (r:1 w:1)
	/// Proof: DaoDemocracy LowestUnbaked (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumCount (r:1 w:0)
	/// Proof: DaoDemocracy ReferendumCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumInfoOf (r:1 w:0)
	/// Proof: DaoDemocracy ReferendumInfoOf (max_values: None, max_size: Some(213), added: 2688, mode: MaxEncodedLen)
	fn bake_dao_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `714`
		//  Estimated: `14265`
		// Minimum execution time: 20_000_000 picoseconds.
		Weight::from_parts(21_000_000, 14265)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Dao NextDaoId (r:1 w:1)
	/// Proof: Dao NextDaoId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommitteeMembers Members (r:1 w:1)
	/// Proof: DaoTechnicalCommitteeMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:3 w:3)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:0 w:1)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Dao Policies (r:0 w:1)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommittee Members (r:0 w:1)
	/// Proof: DaoTechnicalCommittee Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn create_dao() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `121780`
		// Minimum execution time: 171_000_000 picoseconds.
		Weight::from_parts(173_000_000, 121780)
			.saturating_add(RocksDbWeight::get().reads(12_u64))
			.saturating_add(RocksDbWeight::get().writes(16_u64))
	}
	/// Storage: Dao PendingDaos (r:1 w:1)
	/// Proof: Dao PendingDaos (max_values: None, max_size: Some(7639), added: 10114, mode: MaxEncodedLen)
	/// Storage: Dao NextDaoId (r:1 w:1)
	/// Proof: Dao NextDaoId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:3 w:3)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:0 w:1)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Dao Policies (r:0 w:1)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn approve_dao() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1436`
		//  Estimated: `111697`
		// Minimum execution time: 59_000_000 picoseconds.
		Weight::from_parts(62_000_000, 111697)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(11_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:1 w:1)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	fn update_dao_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1189`
		//  Estimated: `8153`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_000_000, 8153)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Dao Policies (r:1 w:1)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:3 w:3)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:4 w:4)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn update_dao_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `838`
		//  Estimated: `178926`
		// Minimum execution time: 92_000_000 picoseconds.
		Weight::from_parts(93_000_000, 178926)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:1 w:0)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	fn mint_dao_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1672`
		//  Estimated: `15451`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(31_000_000, 15451)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DaoTreasury Approvals (r:1 w:1)
	/// Proof: DaoTreasury Approvals (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	/// Storage: DaoBounties BountyApprovals (r:1 w:1)
	/// Proof: DaoBounties BountyApprovals (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	fn spend_dao_funds() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `751`
		//  Estimated: `14976`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(26_000_000, 14976)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy LastTabledWasExternal (r:1 w:0)
	/// Proof: DaoDemocracy LastTabledWasExternal (max_values: None, max_size: Some(13), added: 2488, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy NextExternal (r:1 w:0)
	/// Proof: DaoDemocracy NextExternal (max_values: None, max_size: Some(144), added: 2619, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy PublicProps (r:1 w:1)
	/// Proof: DaoDemocracy PublicProps (max_values: None, max_size: Some(8363), added: 10838, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy DepositOf (r:1 w:1)
	/// Proof: DaoDemocracy DepositOf (max_values: None, max_size: Some(3242), added: 5717, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:1 w:0)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumCount (r:1 w:1)
	/// Proof: DaoDemocracy ReferendumCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumInfoOf (r:0 w:1)
	/// Proof: DaoDemocracy ReferendumInfoOf (max_values: None, max_size: Some(213), added: 2688, mode: MaxEncodedLen)
	fn launch_dao_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2106`
		//  Estimated: `44554`
		// Minimum execution time: 58_000_000 picoseconds.
		Weight::from_parts(60_000_000, 44554)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy LowestUnbaked (r:1 w:1)
	/// Proof: DaoDemocracy LowestUnbaked (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumCount (r:1 w:0)
	/// Proof: DaoDemocracy ReferendumCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumInfoOf (r:1 w:0)
	/// Proof: DaoDemocracy ReferendumInfoOf (max_values: None, max_size: Some(213), added: 2688, mode: MaxEncodedLen)
	fn bake_dao_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `714`
		//  Estimated: `14265`
		// Minimum execution time: 20_000_000 picoseconds.
		Weight::from_parts(21_000_000, 14265)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
