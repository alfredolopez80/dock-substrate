// use crate::{Module,Trait,Error};
use crate::*;
use sp_core::H256;
use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
};
use frame_system as system;

impl_outer_origin! {
	pub enum Origin for Test {}
}

// Configure a mock runtime to test the pallet.

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

// impl_outer_dispatch! {
//     pub enum Call for Test where origin: Origin {
//         frame_system::System,
//         pallet_balances::Balances,
//         democracy::Democracy,
//     }
// }

pub struct TestUpdaterDockFiatRate {}
impl UpdaterDockFiatRate for TestUpdaterDockFiatRate {
	fn update_dock_fiat_rate() {}
}
impl Trait for Test {
	type Event = ();
	type UpdaterDockFiatRate = TestUpdaterDockFiatRate;
	// type Call = Call;
}

pub type FiatFilterModule = Module<Test>;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}




// TEST

// use crate::{Error};
use frame_support::{assert_ok, assert_noop};
use frame_system::{ RawOrigin};

#[test]
fn root_set_update_freq__OK() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(FiatFilterModule::root_set_update_freq(RawOrigin::Root.into(), 42u64));
		// Read pallet storage and assert an expected result.
		assert_eq!(FiatFilterModule::update_freq(), 42u64);
	});
}

#[test]
fn root_set_update_freq__Err_NotRoot() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			FiatFilterModule::root_set_update_freq(Origin::signed(1), 42u64),
			// Error::<Test>::NoneValue
			DispatchError::BadOrigin
		);
	});
}
