
//! Autogenerated weights for pallet_registry
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-02, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `kacper-HP-ProBook-445-G7`, CPU: `AMD Ryzen 7 4700U with Radeon Graphics`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// target/release/substrate
// benchmark
// pallet
// --pallet=pallet_registry
// --execution=wasm
// --wasm-execution=compiled
// --steps=20
// --repeat=10
// --output=frame/registry/src/weights.rs
// --extrinsic=*
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_registry.
pub trait WeightInfo {
	fn add_registry(r: u32, ) -> Weight;
	fn request_entity(s: u32, ) -> Weight;
	fn request_registration(s: u32, ) -> Weight;
	fn cancel_request() -> Weight;
	fn unregister(r: u32, ) -> Weight;
	fn register_entity(r: u32, s: u32, ) -> Weight;
	fn set_registered_entity(r: u32, s: u32, ) -> Weight;
}

/// Weights for pallet_registry using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: CompanyRegistry Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 9]`.
	fn add_registry(r: u32, ) -> Weight {
		// Minimum execution time: 25_949 nanoseconds.
		Weight::from_ref_time(26_630_676)
			// Standard Error: 15_513
			.saturating_add(Weight::from_ref_time(109_488).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CompanyRegistry NextEntityId (r:1 w:1)
	// Storage: CompanyRegistry Requests (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: CompanyRegistry EntityOwner (r:0 w:1)
	// Storage: CompanyRegistry OwnerEntities (r:0 w:1)
	/// The range of component `s` is `[2, 8194]`.
	fn request_entity(s: u32, ) -> Weight {
		// Minimum execution time: 60_154 nanoseconds.
		Weight::from_ref_time(62_525_821)
			// Standard Error: 60
			.saturating_add(Weight::from_ref_time(569).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: CompanyRegistry EntityOwner (r:1 w:0)
	// Storage: CompanyRegistry Requests (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `s` is `[2, 8194]`.
	fn request_registration(s: u32, ) -> Weight {
		// Minimum execution time: 73_700 nanoseconds.
		Weight::from_ref_time(76_333_235)
			// Standard Error: 152
			.saturating_add(Weight::from_ref_time(1_338).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: CompanyRegistry EntityOwner (r:1 w:0)
	// Storage: CompanyRegistry Requests (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn cancel_request() -> Weight {
		// Minimum execution time: 60_965 nanoseconds.
		Weight::from_ref_time(61_636_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: CompanyRegistry Registrars (r:1 w:0)
	// Storage: CompanyRegistry Registries (r:1 w:1)
	// Storage: CompanyRegistry EntityOwner (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[1, 10]`.
	fn unregister(_r: u32, ) -> Weight {
		// Minimum execution time: 59_964 nanoseconds.
		Weight::from_ref_time(62_568_430)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: CompanyRegistry Registrars (r:1 w:0)
	// Storage: CompanyRegistry Requests (r:1 w:1)
	// Storage: CompanyRegistry EntityOwner (r:1 w:0)
	// Storage: CompanyRegistry Registries (r:1 w:1)
	/// The range of component `r` is `[1, 10]`.
	/// The range of component `s` is `[2, 8194]`.
	fn register_entity(r: u32, s: u32, ) -> Weight {
		// Minimum execution time: 45_225 nanoseconds.
		Weight::from_ref_time(46_933_520)
			// Standard Error: 33_003
			.saturating_add(Weight::from_ref_time(19_716).saturating_mul(r.into()))
			// Standard Error: 37
			.saturating_add(Weight::from_ref_time(2_024).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: CompanyRegistry Registrars (r:1 w:0)
	// Storage: CompanyRegistry Registries (r:1 w:1)
	/// The range of component `r` is `[1, 10]`.
	/// The range of component `s` is `[2, 8194]`.
	fn set_registered_entity(r: u32, s: u32, ) -> Weight {
		// Minimum execution time: 34_325 nanoseconds.
		Weight::from_ref_time(34_274_327)
			// Standard Error: 139_638
			.saturating_add(Weight::from_ref_time(192_402).saturating_mul(r.into()))
			// Standard Error: 160
			.saturating_add(Weight::from_ref_time(1_251).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: CompanyRegistry Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 9]`.
	fn add_registry(r: u32, ) -> Weight {
		// Minimum execution time: 25_949 nanoseconds.
		Weight::from_ref_time(26_630_676)
			// Standard Error: 15_513
			.saturating_add(Weight::from_ref_time(109_488).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: CompanyRegistry NextEntityId (r:1 w:1)
	// Storage: CompanyRegistry Requests (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: CompanyRegistry EntityOwner (r:0 w:1)
	// Storage: CompanyRegistry OwnerEntities (r:0 w:1)
	/// The range of component `s` is `[2, 8194]`.
	fn request_entity(s: u32, ) -> Weight {
		// Minimum execution time: 60_154 nanoseconds.
		Weight::from_ref_time(62_525_821)
			// Standard Error: 60
			.saturating_add(Weight::from_ref_time(569).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(6))
	}
	// Storage: CompanyRegistry EntityOwner (r:1 w:0)
	// Storage: CompanyRegistry Requests (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `s` is `[2, 8194]`.
	fn request_registration(s: u32, ) -> Weight {
		// Minimum execution time: 73_700 nanoseconds.
		Weight::from_ref_time(76_333_235)
			// Standard Error: 152
			.saturating_add(Weight::from_ref_time(1_338).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: CompanyRegistry EntityOwner (r:1 w:0)
	// Storage: CompanyRegistry Requests (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn cancel_request() -> Weight {
		// Minimum execution time: 60_965 nanoseconds.
		Weight::from_ref_time(61_636_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: CompanyRegistry Registrars (r:1 w:0)
	// Storage: CompanyRegistry Registries (r:1 w:1)
	// Storage: CompanyRegistry EntityOwner (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[1, 10]`.
	fn unregister(_r: u32, ) -> Weight {
		// Minimum execution time: 59_964 nanoseconds.
		Weight::from_ref_time(62_568_430)
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: CompanyRegistry Registrars (r:1 w:0)
	// Storage: CompanyRegistry Requests (r:1 w:1)
	// Storage: CompanyRegistry EntityOwner (r:1 w:0)
	// Storage: CompanyRegistry Registries (r:1 w:1)
	/// The range of component `r` is `[1, 10]`.
	/// The range of component `s` is `[2, 8194]`.
	fn register_entity(r: u32, s: u32, ) -> Weight {
		// Minimum execution time: 45_225 nanoseconds.
		Weight::from_ref_time(46_933_520)
			// Standard Error: 33_003
			.saturating_add(Weight::from_ref_time(19_716).saturating_mul(r.into()))
			// Standard Error: 37
			.saturating_add(Weight::from_ref_time(2_024).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: CompanyRegistry Registrars (r:1 w:0)
	// Storage: CompanyRegistry Registries (r:1 w:1)
	/// The range of component `r` is `[1, 10]`.
	/// The range of component `s` is `[2, 8194]`.
	fn set_registered_entity(r: u32, s: u32, ) -> Weight {
		// Minimum execution time: 34_325 nanoseconds.
		Weight::from_ref_time(34_274_327)
			// Standard Error: 139_638
			.saturating_add(Weight::from_ref_time(192_402).saturating_mul(r.into()))
			// Standard Error: 160
			.saturating_add(Weight::from_ref_time(1_251).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
}