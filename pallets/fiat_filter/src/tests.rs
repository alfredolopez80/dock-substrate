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
type DidMod = did::Module<Test>;
type AnchorMod = anchor::Module<Test>;
type BlobMod = blob::Module<Test>;
type RevokeMod = revoke::Module<Test>;
impl_outer_dispatch! {
    pub enum TestCall for Test where origin: Origin {
        system::System,
        did::DidMod,
        anchor::AnchorMod,
        blob::BlobMod,
        revoke::RevokeMod,
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

const ALICE: u64 = 100;
const BOB: u64 = 200;
// Build genesis storage according to the mock runtime.
pub fn ext() -> sp_io::TestExternalities {
    let mut ret: sp_io::TestExternalities = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into();
    ret.execute_with(|| {
        let _ = <Test as Config>::Currency::deposit_creating(&ALICE, 100_000_000);
    });
    ret
}

/// generate a random keypair
pub fn gen_kp() -> sr25519::Pair {
    sr25519::Pair::generate_with_phrase(None).0
}
// Create did for `did`. Return the randomly generated signing key.
// The did public key is controlled by some non-existent account (normally a security
// concern), but that doesn't matter for our purposes.
pub fn create_did(origin: u64, did: did::Did) -> sr25519::Pair {
    let kp = gen_kp();
    println!("did pk: {:?}", kp.public().0);
    did::Module::<Test>::new(
        Origin::signed(origin),
        did,
        did::KeyDetail::new(
            [100; 32],
            did::PublicKey::Sr25519(did::Bytes32 {
                value: kp.public().0,
            }),
        ),
    )
    .unwrap();
    kp
}
/// create a did with a random id and random signing key
pub fn newdid(origin: u64) -> (did::Did, sr25519::Pair) {
    let d: did::Did = rand::random();
    (d, create_did(origin, d))
}

pub fn sign(payload: &StateChange, keypair: &sr25519::Pair) -> did::DidSignature {
    did::DidSignature::Sr25519(did::Bytes64 {
        value: keypair.sign(&payload.encode()).0,
    })
}

/// create a random byte array with set len
pub fn random_bytes(len: usize) -> Vec<u8> {
    let ret: Vec<u8> = (0..len).map(|_| rand::random()).collect();
    assert_eq!(ret.len(), len);
    ret
}

// TESTS

use core_mods::StateChange;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

mod tests_did_calls {
    use super::*;
    use did::{Bytes32, Bytes64, DidRemoval, DidSignature, KeyUpdate, PublicKey, DID_BYTE_SIZE};

    #[test]
    fn call_did_new() {
        ext().execute_with(|| {
            let d: did::Did = rand::random();
            let kp = gen_kp();
            let key_detail = did::KeyDetail::new(
                d,
                did::PublicKey::Sr25519(did::Bytes32 {
                    value: kp.public().0,
                }),
            );

            let call = TestCall::DidMod(did::Call::<Test>::new(d.clone(), key_detail));
            FiatFilterModule::execute_call(Origin::signed(ALICE), Box::new(call.clone())).unwrap();

            // TODO assert fee 50
        });
    }
    #[test]
    fn call_did_update_key__OK() {
        ext().execute_with(|| {
            let did_alice = [1; DID_BYTE_SIZE];

            let (pair_1, _, _) = sr25519::Pair::generate_with_phrase(None);
            let pk_1 = pair_1.public().0;
            println!("update did:{:?}", did_alice.to_vec());
            println!("update pk_1:{:?}", pk_1.to_vec());
            let detail = KeyDetail::new(
                did_alice.clone(),
                PublicKey::Sr25519(Bytes32 { value: pk_1 }),
            );

            // Add a DID
            assert_ok!(DidMod::new(
                Origin::signed(ALICE),
                did_alice.clone(),
                detail.clone()
            ));

            let (current_detail, modified_in_block) = DidMod::get_key_detail(&did_alice).unwrap();
            // assert_eq!(current_detail.controller, did);

            // Correctly update DID's key.
            // Prepare a key update
            let (pair_2, _, _) = sr25519::Pair::generate_with_phrase(None);
            let pk_2 = pair_2.public().0;
            println!("update pk_2:{:?}", pk_2.to_vec());
            let key_update = KeyUpdate::new(
                did_alice.clone(),
                PublicKey::Sr25519(Bytes32 { value: pk_2 }),
                None,
                modified_in_block as u32,
            );
            let sig_value = pair_1
                .sign(&StateChange::KeyUpdate(key_update.clone()).encode())
                .0;
            // // println!("update modified_in_block:{:?}", modified_in_block);
            // println!("update sig_value:{:?}", sig_value.to_vec());
            let sig = DidSignature::Sr25519(did::Bytes64 { value: sig_value });

            // Signing with the current key (`pair_1`) to update to the new key (`pair_2`)
            // assert_ok!(DidMod::update_key(
            //     Origin::signed(alice),
            //     key_update,
            //     sig
            // ));
            let call = TestCall::DidMod(did::Call::<Test>::update_key(key_update, sig));
            FiatFilterModule::execute_call(Origin::signed(ALICE), Box::new(call.clone())).unwrap();

            // let (current_detail, modified_in_block) = DidMod::get_key_detail(&did).unwrap();
            // // Since key update passed None for the controller, it should not change
            // assert_eq!(current_detail.controller, did);

            // TODO assert Pays::No
            // TODO assert fee 51
        });
    }
    #[test]
    fn call_did_remove() {
        ext().execute_with(|| {
            let did = [1; DID_BYTE_SIZE];
            let (pair_1, _, _) = sr25519::Pair::generate_with_phrase(None);
            let pk_1 = pair_1.public().0;

            let to_remove = DidRemoval::new(did.clone(), 2u32);
            let sig = DidSignature::Sr25519(Bytes64 {
                value: pair_1
                    .sign(&StateChange::DIDRemoval(to_remove.clone()).encode())
                    .0,
            });

            let call = TestCall::DidMod(did::Call::<Test>::remove(to_remove, sig));
            FiatFilterModule::execute_call(Origin::signed(ALICE), Box::new(call.clone())).unwrap();

            // TODO assert fee 52
        });
    }
}

#[test]
fn call_anchor_deploy() {
    use anchor;

    ext().execute_with(|| {
        let dat = (0..32).map(|_| rand::random()).collect();
        let call = TestCall::AnchorMod(anchor::Call::<Test>::deploy(dat));
        FiatFilterModule::execute_call(Origin::signed(ALICE), Box::new(call.clone())).unwrap();

        // TODO assert fee 60
    });
}

#[test]
fn call_blob_new() {
    use blob::{Blob, BlobId};
    ext().execute_with(|| {
        let id: BlobId = rand::random();
        let noise = random_bytes(999);
        let (author, author_kp) = newdid(ALICE);
        let blob = Blob {
            id,
            blob: noise.clone(),
            author,
        };
        let sig = sign(&StateChange::Blob(blob.clone()), &author_kp);

        let call = TestCall::BlobMod(blob::Call::<Test>::new(blob, sig));
        FiatFilterModule::execute_call(Origin::signed(ALICE), Box::new(call.clone())).unwrap();

        // TODO assert fee 70
    });
}

mod tests_revoke_calls {
    use super::*;
    use did::Did;
    use revoke::{Policy, Registry, RegistryId, RemoveRegistry, Revoke, RevokeId, UnRevoke};

    pub const REG_ID: RegistryId = [3u8; 32];
    pub const REV_ID: RevokeId = [7u8; 32];
    // pub const DID_ALICE: Did = [4u8; 32];
    // pub const DID_BOB: Did = [5u8; 32];

    pub fn policy_oneof(dids: &[Did]) -> Policy {
        Policy::OneOf(dids.iter().cloned().collect())
    }
    pub fn block_no() -> u64 {
        system::Module::<Test>::block_number()
    }
    pub fn new_reg(did: Did) -> Registry {
        let reg = Registry {
            policy: policy_oneof(&[did]),
            add_only: true,
        };
        RevokeMod::new_registry(Origin::signed(ALICE), REG_ID, reg.clone()).unwrap();
        reg
    }

    #[test]
    fn call_revoke_revoke() {
        ext().execute_with(|| {
            let (did_alice, kp_alice) = newdid(ALICE);
            let reg = new_reg(did_alice);
            // let reg = Registry {
            //     policy: policy_oneof(&[did_alice]),
            //     add_only: true,
            // };
            // RevokeMod::new_registry(Origin::signed(ALICE), REG_ID, reg).unwrap();

            let cases: &[&[RevokeId]] = &[
                &[],
                &[random()],
                &[random(), random()],
                &[random(), random(), random()],
                &[REV_ID], // Test idempotence, step 1
                &[REV_ID], // Test idempotence, step 2
            ];
            for ids in cases {
                let revoke = Revoke {
                    registry_id: REG_ID,
                    revoke_ids: ids.iter().cloned().collect(),
                    last_modified: block_no() as u32,
                };
                let proof = std::iter::once((
                    did_alice,
                    sign(&StateChange::Revoke(revoke.clone()), &kp_alice),
                ))
                .collect();

                let call = TestCall::RevokeMod(revoke::Call::<Test>::revoke(revoke, proof));
                FiatFilterModule::execute_call(Origin::signed(ALICE), Box::new(call.clone()))
                    .unwrap();

                // assert!(ids
                //     .iter()
                //     .all(|id| Revocations::contains_key(registry_id, id)));

                // TODO assert fee 81
            }
        });
    }
    #[test]
    fn call_revoke_unrevoke() {
        ext().execute_with(|| {
            let (did_alice, kp_alice) = newdid(ALICE);
            let reg = new_reg(did_alice);
            let last_modified = block_no() as u32;
            // let revoke_ids: BTreeSet<RevokeId> = &[REV_ID].iter().cloned().collect();

            // 1. revoke
            let revoke = Revoke {
                registry_id: REG_ID,
                revoke_ids: vec![REV_ID].iter().cloned().collect(),
                last_modified,
            };
            let proof = std::iter::once((
                did_alice,
                sign(&StateChange::Revoke(revoke.clone()), &kp_alice),
            ))
            .collect();
            RevokeMod::revoke(Origin::signed(ALICE), revoke.clone(), proof).unwrap();
            // TODO assert revoked

            // 2. unrevoke
            let unrevoke = UnRevoke {
                registry_id: REG_ID,
                revoke_ids: revoke.revoke_ids.clone(),
                last_modified,
            };
            let proof = std::iter::once((
                did_alice,
                sign(&StateChange::UnRevoke(unrevoke.clone()), &kp_alice),
            ))
            .collect();
            // RevoMod::unrevoke(Origin::signed(ALICE), unrevoke, proof).unwrap();
            let call = TestCall::RevokeMod(revoke::Call::<Test>::unrevoke(unrevoke, proof));
            FiatFilterModule::execute_call(Origin::signed(ALICE), Box::new(call.clone())).unwrap();
            // TODO assert unrevoked

            // let call = TestCall::BlobMod(blob::Call::<Test>::new(blob));
            // FiatFilterModule::execute_call(Origin::signed(ALICE), Box::new(call.clone())).unwrap();

            // TODO assert fee 82
        });
    }
    #[test]
    fn call_revoke_new_registry() {
        ext().execute_with(|| {
            let (did_alice, _) = newdid(ALICE);
            let (did_bob, _) = newdid(BOB);
            let cases: &[(Policy, bool)] = &[
                (policy_oneof(&[did_alice]), false),
                (policy_oneof(&[did_alice, did_bob]), false),
                (policy_oneof(&[did_alice]), true),
                (policy_oneof(&[did_alice, did_bob]), true),
            ];
            for (policy, add_only) in cases.iter().cloned() {
                let reg_id = random();
                let reg = Registry { policy, add_only };

                // assert!(!Registries::<Test>::contains_key(reg_id));
                // RevokeMod::new_registry(Origin::signed(ALICE), reg_id, reg.clone()).unwrap();
                let call = TestCall::RevokeMod(revoke::Call::<Test>::new_registry(reg_id, reg));
                FiatFilterModule::execute_call(Origin::signed(ALICE), Box::new(call.clone()))
                    .unwrap();
                // assert!(Registries::<Test>::contains_key(reg_id));

                // let (created_reg, created_bloc) = Registries::<Test>::get(reg_id).unwrap();
                // assert_eq!(created_reg, reg);
                // assert_eq!(created_bloc, block_no());

                // TODO assert fee 80
            }
        });
    }
    #[test]
    fn call_revoke_remove_registry() {
        ext().execute_with(|| {
            let (did_alice, kp_alice) = newdid(ALICE);
            let policy = policy_oneof(&[did_alice]);
            // let registry_id = REG_ID;
            let add_only = false;
            let last_modified = block_no() as u32;

            let reg = Registry { policy, add_only };
            RevokeMod::new_registry(Origin::signed(ALICE), REG_ID, reg).unwrap();
            // assert!(Registries::<Test>::contains_key(registry_id));

            // destroy reg
            let rem = RemoveRegistry {
                registry_id: REG_ID,
                last_modified,
            };
            let proof = std::iter::once((
                did_alice,
                sign(&StateChange::RemoveRegistry(rem.clone()), &kp_alice),
            ))
            .collect();
            // RevokeMod::remove_registry(Origin::signed(ALICE), rem, proof).unwrap();

            let call = TestCall::RevokeMod(revoke::Call::<Test>::remove_registry(rem, proof));
            FiatFilterModule::execute_call(Origin::signed(ALICE), Box::new(call.clone())).unwrap();

            // assert not exists
            // assert!(!Registries::<Test>::contains_key(registry_id));

            // TODO assert fee 83
        });
    }
}

mod tests_root_calls {
    use super::*;

    #[test]
    fn root_set_update_freq__OK() {
        ext().execute_with(|| {
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
        ext().execute_with(|| {
            // Ensure the expected error is thrown when no value is present.
            assert_noop!(
                FiatFilterModule::root_set_update_freq(Origin::signed(ALICE), 42u64),
                // Error::<Test>::NoneValue
                DispatchError::BadOrigin
            );
        });
    }

    #[test]
    fn root_set_DockFiatRate__OK() {
        ext().execute_with(|| {
            // Dispatch a signed extrinsic.
            assert_ok!(FiatFilterModule::root_set_dock_fiat_rate(
                RawOrigin::Root.into(),
                Perbill::from_parts(500_000),
            ));
            // Read pallet storage and assert an expected result.
            assert_eq!(
                FiatFilterModule::dock_fiat_rate(),
                Perbill::from_parts(500_000)
            );
        });
    }

    #[test]
    fn root_set_dock_fiat_rate__Err_NotRoot() {
        ext().execute_with(|| {
            // Ensure the expected error is thrown when no value is present.
            assert_noop!(
                FiatFilterModule::root_set_update_freq(Origin::signed(ALICE), 42u64),
                // Error::<Test>::NoneValue
                DispatchError::BadOrigin
            );
        });
    }
}
