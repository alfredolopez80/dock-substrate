use crate::*;
use codec::{Decode, Encode};
use core_mods::{anchor, did};

use frame_support::dispatch::Dispatchable;
use frame_support::weights::Weight;
use frame_support::{impl_outer_dispatch, impl_outer_event, impl_outer_origin, parameter_types};
use frame_system as system;
use rand::random;
use sp_core::H256;
use sp_core::{sr25519, Pair};
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
type AnchorModule = anchor::Module<Test>;
type BlobModule = blob::Module<Test>;
type RevokeModule = revoke::Module<Test>;
impl_outer_dispatch! {
    pub enum TestCall for Test where origin: Origin {
        system::System,
        did::DIDModule,
        anchor::AnchorModule,
        blob::BlobModule,
        revoke::RevokeModule,
    }
}

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
impl Config for Test {
    type Event = ();
    type UpdaterDockFiatRate = TestUpdaterDockFiatRate;
    type Call = TestCall;
    type Currency = pallet_balances::Module<Self>;
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
impl crate::anchor::Trait for Test {
    type Event = ();
}
impl did::Trait for Test {
    type Event = ();
}
impl crate::revoke::Trait for Test {}
parameter_types! {
    pub const MaxBlobSize: u32 = 1024;
}
impl crate::blob::Trait for Test {
    type MaxBlobSize = MaxBlobSize;
}

pub struct TestUpdaterDockFiatRate {}
impl UpdaterDockFiatRate for TestUpdaterDockFiatRate {
    fn update_dock_fiat_rate() -> DispatchResultWithPostInfo {
        Ok(Pays::No.into())
    }
}

pub type FiatFilterModule = Module<Test>;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

/// generate a random keypair
pub fn gen_kp() -> sr25519::Pair {
    sr25519::Pair::generate_with_phrase(None).0
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

#[test]
fn call_did_new() {
    new_test_ext().execute_with(|| {
        let d: did::Did = rand::random();
        let kp = gen_kp();
        let call = TestCall::DIDModule(did::Call::<Test>::new(
            d.clone(),
            did::KeyDetail::new(
                d,
                did::PublicKey::Sr25519(did::Bytes32 {
                    value: kp.public().0,
                }),
            ),
        ));
        FiatFilterModule::execute_call(Origin::signed(1), Box::new(call.clone())).unwrap();

        // TODO assert fee 50
    });
}
#[test]
fn call_did_update_key() {
    new_test_ext().execute_with(|| {
        let key_update = todo!();
        let sig = todo!();
        // let call = TestCall::DIDModule(did::Call::<Test>::key_update(key_update,sig));

        // TODO assert fee 51
    });
}
#[test]
fn call_did_remove() {
    new_test_ext().execute_with(|| {
        let to_remove = todo!();
        let sig = todo!();
        // let call = TestCall::DIDModule(did::Call::<Test>::remove(to_remove, sig));
        // FiatFilterModule::execute_call(Origin::signed(1), Box::new(call.clone())).unwrap();

        // TODO assert fee 52
    });
}

#[test]
fn call_anchor_deploy() {
    new_test_ext().execute_with(|| {
        let dat = (0..32).map(|_| rand::random()).collect();
        let call = TestCall::AnchorModule(anchor::Call::<Test>::deploy(dat));
        FiatFilterModule::execute_call(Origin::signed(1), Box::new(call.clone())).unwrap();

        // TODO assert fee 60
    });
}

#[test]
fn call_blob_new() {
    new_test_ext().execute_with(|| {
        let blob = todo!();
        // let call = TestCall::BlobModule(blob::Call::<Test>::new(blob));
        // FiatFilterModule::execute_call(Origin::signed(1), Box::new(call.clone())).unwrap();

        // TODO assert fee 70
    });
}

#[test]
fn call_revoke_new_registry() {
    new_test_ext().execute_with(|| {
        let id = todo!();
        let registry = todo!();
        // let call = TestCall::BlobModule(blob::Call::<Test>::new(blob));
        // FiatFilterModule::execute_call(Origin::signed(1), Box::new(call.clone())).unwrap();

        // TODO assert fee 80
    });
}
#[test]
fn call_revoke_revoke() {
    new_test_ext().execute_with(|| {
        let revoke = todo!();
        let proof = todo!();
        // let call = TestCall::BlobModule(blob::Call::<Test>::new(blob));
        // FiatFilterModule::execute_call(Origin::signed(1), Box::new(call.clone())).unwrap();

        // TODO assert fee 81
    });
}
#[test]
fn call_revoke_unrevoke() {
    new_test_ext().execute_with(|| {
        let unrevoke = todo!();
        let proof = todo!();
        // let call = TestCall::BlobModule(blob::Call::<Test>::new(blob));
        // FiatFilterModule::execute_call(Origin::signed(1), Box::new(call.clone())).unwrap();

        // TODO assert fee 82
    });
}
#[test]
fn call_revoke_remove_registry() {
    new_test_ext().execute_with(|| {
        let removal = todo!();
        let proof = todo!();
        // let call = TestCall::BlobModule(blob::Call::<Test>::new(blob));
        // FiatFilterModule::execute_call(Origin::signed(1), Box::new(call.clone())).unwrap();

        // TODO assert fee 83
    });
}
