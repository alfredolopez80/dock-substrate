#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_imports)] // TODO remove

use core_mods::did::KeyDetail;
use core_mods::{anchor, blob, did, revoke};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::{
        CallMetadata, DispatchResult, DispatchResultWithPostInfo, Dispatchable, GetCallMetadata,
    },
    ensure,
    sp_runtime::{DispatchError, Perbill, Permill},
    traits::{
        Currency, ExistenceRequirement, Get, IsSubType, UnfilteredDispatchable, WithdrawReasons,
    },
    weights::{GetDispatchInfo, Pays},
    Parameter,
};
use frame_system::{self as system, ensure_root, ensure_signed};
use sp_std::fmt::Debug;

#[cfg(test)]
#[allow(non_snake_case)]
mod tests;

/// Config option for updating the DockFiatRate
pub trait UpdaterDockFiatRate {
    /// Handler for updating the DockFiatRate
    fn update_dock_fiat_rate() -> DispatchResultWithPostInfo;
}

// /// The pallet's configuration trait
// /// Configure the pallet by specifying the parameters and types on which it depends.
// pub trait Config: frame_system::Config {
//     /// Because this pallet emits events, it depends on the runtime's definition of an event.
//     type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;
//     // type Call: Parameter + UnfilteredDispatchable<Origin = Self::Origin> + GetDispatchInfo;
//     type UpdaterDockFiatRate: UpdaterDockFiatRate;
//     // type BlockNumber: Get<<Self as frame_system::Config>::BlockNumber>;
// }
pub trait Config:
    system::Config + did::Trait + anchor::Trait + blob::Trait + revoke::Trait
{
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as system::Config>::Event>;
    /// Config option for updating the DockFiatRate
    type UpdaterDockFiatRate: UpdaterDockFiatRate;
    /// The outer call (dispatchable) that this module is called with. It is possible to use another type here, but
    /// it's expected that your runtime::Call will be used.
    /// Since we need to match by origin crate, we implement `IsSubType` for each Call type we want to match for.
    type Call: Parameter
        + Dispatchable<Origin = Self::Origin>
        + GetDispatchInfo
        + IsSubType<did::Call<Self>>
        + IsSubType<anchor::Call<Self>>
        + IsSubType<blob::Call<Self>>
        + IsSubType<revoke::Call<Self>>;
    /// The module's Currency type definition
    type Currency: Currency<Self::AccountId>;
}

// The pallet's runtime storage items.
decl_storage! {
    // A unique name is used to ensure that the pallet's storage items are isolated.
    // This name may be updated, but each pallet in the runtime must use a unique name.
    // ----------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Config> as FiatFilterModule {
        /// price of one DOCK in fiat (for now, only USD)
        pub DockFiatRate get(fn dock_fiat_rate) config(): Perbill = Perbill::from_fraction(0.02251112);
        /// price update frequency (in number of blocks)
        pub UpdateFreq get(fn update_freq) config(): <T as system::Config>::BlockNumber = 10u32.into();
        // /// block number of last DockUsdRate update
        pub LastUpdatedAt get(fn last_updated_at): <T as system::Config>::BlockNumber;
    }
}

// Pallets use events to inform users when important changes are made.
decl_event! {
    pub enum Event<T> where
        // AccountId = <T as frame_system::Config>::AccountId, // TODO remove bound
        <T as frame_system::Config>::BlockNumber,
    {
        /// on set_dock_usd_rate executed
        /// event parameters: [new_dock_usd_rate]
        DockFiatRateUpdated(Perbill),
        /// on root_set_update_freq executed
        /// event parameters: [new_update_frequency_blocks]
        UpdateFreqUpdated(BlockNumber),
    }
}

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Config> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }
}

// The module's callable functions (Dispatchable functions)
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // // Errors must be initialized if they are used by the pallet.
        // type Error = Error<T>;
        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        /// Execute a Call. Must be a DID or Blob call
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn execute_call(origin, call: Box<<T as Config>::Call>) -> DispatchResultWithPostInfo {
            Self::execute_call_(origin,&call)?;

            // LH: This check and update should happen in `on_initialize`
            // TODO update DockUsdRate if more than N blocks
            let current_block = <system::Module<T>>::block_number(); // TODO check safety of saturated_into
            // TODO type of current block, vs type of updated_at, update_freq
            if current_block - Self::last_updated_at() > Self::update_freq() {
                T::UpdaterDockFiatRate::update_dock_fiat_rate()?;
            }



            // TODO return Pays::No in PostDispatchInfo
            Ok(Pays::No.into())
        }

        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn root_set_update_freq(origin, new_update_freq: T::BlockNumber) -> DispatchResult {
            // ensure!(sender == system::RawOrigin::Root.into(), "only Root can force-update the update frequency");
            ensure_root(origin)?;
            // Update storage.
            UpdateFreq::<T>::put(new_update_freq);
            // Emit an event.
            Self::deposit_event(RawEvent::UpdateFreqUpdated(new_update_freq));
            // Return a successful DispatchResult
            Ok(())
        }

        // /// An example dispatchable that may throw a custom error.
        // #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        // pub fn cause_error(origin) -> dispatch::DispatchResult {
        // 	let _who = ensure_signed(origin)?;

        // 	// Read a value from storage.
        // 	match Something::get() {
        // 		// Return an error if the value has not been set.
        // 		None => Err(Error::<T>::NoneValue)?,
        // 		Some(old) => {
        // 			// Increment the value read from storage; will error in the event of overflow.
        // 			let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
        // 			// Update the value in storage with the incremented result.
        // 			Something::put(new);
        // 			Ok(())
        // 		},
        // 	}
        // }
    }
}

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as system::Config>::AccountId>>::Balance;

// trait AmountFiat {
//     const per_unit: u64;
//     fn currency() -> FiatCurrency;
// }
// struct FiatCurrency {
//     pub name: String,
//     pub code: String,
// }
type AmountUsd = Permill;
// impl AmountFiat for AmountUsd {
//     const per_unit: u64 = 1_000_000;
//     fn currency() -> FiatCurrency {
//         FiatCurrency {
//             name: "US dollar".to_string(),
//             code: "USD".to_string(),
//         }
//     }
// }

// trait TryIntoAmountDock {
//     fn try_into_amount_dock(&self,dock_fiat_rate:Perbill) -> Result<u64,&'static str>;
// }
// impl TryIntoAmountDock for AmountUsd {
//     fn try_into_amount_dock(&self,dock_fiat_rate:Perbill) -> Result<u64,&'static str> {
//         let fee_dock: T::Balance = fee_usdcent as f64.checked_div(Self::dock_fiat_rate() as f64) // TODO safe math and type conversion
//         .ok_or("checked_div err: Dock usd rate is zero")?;
//         let fee_dock:T::Balance =
//         Ok(0)
//     }
// }

// private helper functions
impl<T: Config> Module<T> {
    fn get_call_fee_fiat_(call: &<T as Config>::Call) -> AmountUsd {
        match call.is_sub_type() {
            Some(did::Call::new(did, detail)) => return Permill::from_percent(50),
            Some(did::Call::update_key(key_update, sig)) => return Permill::from_percent(51),
            Some(did::Call::remove(to_remove, sig)) => return Permill::from_percent(52),
            _ => {}
        };
        match call.is_sub_type() {
            Some(anchor::Call::deploy(bytes)) => return Permill::from_percent(60),
            _ => {}
        };
        match call.is_sub_type() {
            Some(blob::Call::new(blob, sig)) => return Permill::from_percent(70),
            _ => {}
        };
        match call.is_sub_type() {
            Some(revoke::Call::new_registry(id, registry)) => return Permill::from_percent(80),
            Some(revoke::Call::revoke(revoke, proof)) => return Permill::from_percent(81),
            Some(revoke::Call::unrevoke(unrevoke, proof)) => return Permill::from_percent(82),
            Some(revoke::Call::remove_registry(rm, proof)) => return Permill::from_percent(83),
            _ => {}
        };
        return Permill::from_percent(500);
    }
    fn compute_call_fee_dock_(call: &<T as Config>::Call) -> Result<BalanceOf<T>, &'static str> {
        let fee_fiat_permill: AmountUsd = Self::get_call_fee_fiat_(call);
        let fee_fiat_perbill = Perbill::from_parts(fee_fiat_permill.deconstruct() * 1000);

        let fee_dock_perbill: Perbill = fee_fiat_perbill / Self::dock_fiat_rate();
        let fee_dock_permill: u32 = fee_dock_perbill
            .deconstruct()
            .checked_div(1000)
            .ok_or("checked_div err: DockFiatRate is zero")?;

        // The token has 6 decimal places (defined in the runtime)
        // pub const DOCK: Balance = 1_000_000;
        // T::Balance is already expressed in DOCK_Permill
        let fee_dock = <BalanceOf<T>>::from(fee_dock_permill);
        Ok(fee_dock)
    }

    fn charge_fees_(who: T::AccountId, amount: BalanceOf<T>) -> Result<(), &'static str> {
        let _ = <T::Currency>::withdraw(
            &who,
            amount,
            WithdrawReasons::FEE,
            ExistenceRequirement::KeepAlive,
        )?;
        Ok(())
    }

    fn execute_call_(origin: T::Origin, call: &<T as Config>::Call) -> DispatchResultWithPostInfo {
        let sender = ensure_signed(origin.clone())?;

        // calculate fee based on type of call
        let fee_dock = Self::compute_call_fee_dock_(&call)?;
        // deduct fees based on Currency::Withdraw
        Self::charge_fees_(sender, fee_dock)?;

        let dispatch_result = call.clone().dispatch(origin);

        Ok(Pays::No.into())
    }

    // /// update the dock_usd_rate
    // pub fn set_dock_usd_rate(origin, value: u64) -> DispatchResultWithPostInfo {
    //     let sender= ensure_signed(origin)?;

    //     ////////////////
    //     // TODO check that it's the price fedd contract that is updating the value

    //     DockUsdRate::put(value);
    //     Self::deposit_event(RawEvent::DockUsdRateSet(value));
    // }

    fn compute_call_fee_dock_OLD(call: &<T as Config>::Call) -> Result<u64, &'static str> {
        // TODO get type of call
        // let dispatch_info = call.get_dispatch_info();
        // let weight = dispatch_info.weight;
        // // let call: &<T as Config>::Call = <T as Config>::Call::from_ref(call);
        // // Get the pallet name and function name

        // // TODO: Get arguments
        // /*if let (a, b) = did::Call::new(d, det) {
        //     sp_runtime::print("something matched");
        // }*/
        // /*if let did::Call::new(d, det) = call {
        //     sp_runtime::print("something matched");
        // }*/
        // runtime_print!("{:?}", &call);
        // println!("{:?}", &call);

        // // TODO: Unless i find a better way, will need a flag to track if anything matched the call
        // let mut found = false;

        // if (!found) {
        //     match call.is_sub_type() {
        //         Some(did::Call::new(d, k)) => {
        //             found = true;
        //             sp_runtime::print("DID new match");
        //             println!("DID new match");
        //             runtime_print!("DID new match {:?}", call);
        //             // call
        //         },
        //         _ => (),
        //     };
        // }

        // if (!found) {
        //     match call.is_sub_type() {
        //         Some(anchor::Call::deploy(b)) => {
        //             found = true;
        //             sp_runtime::print("Anchor deploy match");
        //             println!("Anchor deploy match. Data length is {}", b.len());
        //             runtime_print!("Anchor deploy match {:?}", call);
        //             // call
        //         },
        //         _ => (),
        //     };
        // }

        /*let c2 = match *call1 {
            Call::anchor::Call::deploy(d) => {
                runtime_print!("Second match {:?}", d);
            },
            Call::did::Call::new(d, k) => {
                runtime_print!("Second match {:?}", d);
            },
            _ => unimplemented!()
        };*/

        /*if let Some(local_call) = call.is_sub_type() {
            match local_call {
                did::Call::<T>::remove(_, _) => (),
                _ => (),
            }
        }*/

        // let _ = did::Call::new(..);
        // if (!matches!(call, <T as Config>::Call(did::Call::new(..)))) {
        /*if (matches!(call, did::Call::new(..) ) ) {
            sp_runtime::print("Matched new")
        }*/

        // if let Some(local_call) = call.is_sub_type() {
        //     match local_call {
        //         Call::remove(_, _) => (),
        //         // Call::<Self>::new_registry(_, _) => (),
        //         // <T as Config>::Call::DID => (),
        //         // Call::DID => (),
        //         // did::Call::<T>::remove(_, _) => (),
        //         // Migrator can make only these 2 calls without paying fees
        //         // Call::migrate(..) | Call::give_bonuses(..) => {
        //         //     if !<Migrators<T>>::contains_key(who) {
        //         //         // If migrator not registered, don't include transaction in block
        //         //         return InvalidTransaction::Custom(1).into();
        //         //     }
        //         // }
        //         _ => (),
        //     }
        // }

        // match type: get USD price
        // let fee_usdcent = match call_type {
        //     _ => 50,
        // };

        // convert to DOCKs
        // TODO check conversion to f64 and division
        // let fee_dock: T::Balance = fee_usdcent as f64.checked_div(Self::dock_fiat_rate() as f64) // TODO safe math and type conversion
        // // TODO what is minimum unit for DOCKs
        // .ok_or("checked_div err: Dock usd rate is zero")?;

        let fee_dock = 0;
        Ok(fee_dock)
    }
}
