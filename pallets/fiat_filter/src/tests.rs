// // use crate::{Module,Trait,Error};
use crate::*;
use codec::{Decode, Encode};
use core_mods::did;
use frame_support::dispatch::Dispatchable;
use frame_support::weights::Weight;
use frame_support::{impl_outer_dispatch, impl_outer_event, impl_outer_origin, parameter_types};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};

impl_outer_origin! {
    pub enum Origin for Test where system = frame_system {}
}
type System = frame_system::Module<Test>;
type DIDModule = did::Module<Test>;
impl_outer_dispatch! {
    pub enum TestCall for Test where origin: Origin {
        system::System,
        did::DIDModule,
    }
}
// mod fiat_filter {
//     pub use crate::Event;
// }

// Configure a mock runtime to test the pallet.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
    pub const BlockHashCount: u64 = 250;
}
impl system::Config for Test {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Call = TestCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = ();
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type BlockWeights = ();
    type BlockLength = ();
    type SS58Prefix = ();
}
pub type FiatFilterModule = Module<Test>;

impl Config for Test {
    type Event = ();
    type UpdaterDockFiatRate = TestUpdaterDockFiatRate;
    type Call = TestCall;
    type Currency = pallet_balances::Module<Self>;
}
impl did::Trait for Test {
    type Event = ();
}
parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type Balance = u64;
    type Event = ();
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

pub struct TestUpdaterDockFiatRate {}
impl UpdaterDockFiatRate for TestUpdaterDockFiatRate {
    fn update_dock_fiat_rate() -> DispatchResultWithPostInfo {
        Ok(Pays::No.into())
    }
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

// TESTS

use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

#[test]
fn root_set_update_freq__OK() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(FiatFilterModule::root_set_update_freq(
            RawOrigin::Root.into(),
            42u64
        ));
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
