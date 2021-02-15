#![cfg_attr(not(feature = "std"), no_std)]

use core_mods::{anchor, blob, did, revoke};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::{
        DispatchError, DispatchErrorWithPostInfo, DispatchResult, DispatchResultWithPostInfo,
        Dispatchable, PostDispatchInfo,
    },
    sp_runtime::{Perbill, Permill},
    traits::{Currency, ExistenceRequirement, Get, IsSubType, WithdrawReasons},
    weights::{GetDispatchInfo, Pays, Weight},
    Parameter,
};
use frame_system::{self as system, ensure_root, ensure_signed};
use sp_std::boxed::Box;

#[cfg(test)]
#[allow(non_snake_case)]
mod tests;

/// Config option for updating the DockFiatRate
pub trait UpdaterDockFiatRate {
    /// Handler for updating the DockFiatRate
    fn update_dock_fiat_rate() -> Result<(), &'static str>;
}

// /// The pallet's configuration trait
// /// Configure the pallet by specifying the parameters and types on which it depends.
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
        pub DockFiatRate get(fn dock_fiat_rate) config(): Perbill = Perbill::from_parts(22511120);
        /// price update frequency (in number of blocks)
        pub UpdateFreq get(fn update_freq) config(): <T as system::Config>::BlockNumber = 10u32.into();
        // /// block number of last DockUsdRate update
        pub LastUpdatedAt get(fn last_updated_at): <T as system::Config>::BlockNumber;
    }
}

// Pallets use events to inform users when important changes are made.
decl_event! {
    pub enum Event<T> where
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
        /// Failed calculation because of numeric type overflow
        ArithmeticOverflow,
        /// checked_div: tried to divide by zero
        DivideByZero,
    }
}

// The module's callable functions (Dispatchable functions)
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>; // TODO still works if commented-out. Why ?
        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;
        /// `on_initialize` gets called on every new block
        fn on_initialize(n: T::BlockNumber) -> Weight {
            // update DockUsdRate if more than N blocks
            let current_block = <system::Module<T>>::block_number();
            if current_block - Self::last_updated_at() > Self::update_freq() {
                if let Err(e) = T::UpdaterDockFiatRate::update_dock_fiat_rate() {
                    sp_runtime::print(e);
                }
            };
            return 0;
        }

        /// Execute a Call. Must be a DID or Blob call
        #[weight = (10_000 + T::DbWeight::get().writes(1), Pays::No)]
        pub fn execute_call(origin, call: Box<<T as Config>::Call>) -> DispatchResultWithPostInfo {
            Self::execute_call_(origin, &call)?;
            Ok(Pays::No.into())
        }

        /// Set the Update Frequency through Root
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn root_set_update_freq(origin, new_update_freq: T::BlockNumber) -> DispatchResult {
            ensure_root(origin)?;
            // Update storage.
            UpdateFreq::<T>::put(new_update_freq);
            // Emit an event.
            Self::deposit_event(RawEvent::UpdateFreqUpdated(new_update_freq));
            Ok(())
        }

        /// Set DockFiatRate through Root
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn root_set_dock_fiat_rate(origin, value: Perbill) -> DispatchResult {
            ensure_root(origin)?;
            // Update storage
            DockFiatRate::put(value);
            // on storage updated, emit an event
            Self::deposit_event(RawEvent::DockFiatRateUpdated(value));

            Ok(())
        }
    }
}

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as system::Config>::AccountId>>::Balance;

type AmountUsd = Permill;

// private helper functions
impl<T: Config> Module<T> {
    fn get_call_fee_fiat_(call: &<T as Config>::Call) -> AmountUsd {
        match call.is_sub_type() {
            Some(did::Call::new(_did, _detail)) => return Permill::from_percent(50),
            Some(did::Call::update_key(_key_update, _sig)) => return Permill::from_percent(51),
            Some(did::Call::remove(_to_remove, _sig)) => return Permill::from_percent(52),
            _ => {}
        };
        match call.is_sub_type() {
            Some(anchor::Call::deploy(_bytes)) => return Permill::from_percent(60),
            _ => {}
        };
        match call.is_sub_type() {
            Some(blob::Call::new(_blob, _sig)) => return Permill::from_percent(70),
            _ => {}
        };
        match call.is_sub_type() {
            Some(revoke::Call::new_registry(_id, _registry)) => return Permill::from_percent(80),
            Some(revoke::Call::revoke(_revoke, _proof)) => return Permill::from_percent(81),
            Some(revoke::Call::unrevoke(_unrevoke, _proof)) => return Permill::from_percent(82),
            Some(revoke::Call::remove_registry(_rm, _proof)) => return Permill::from_percent(83),
            _ => {}
        };
        return Permill::from_percent(500);
    }
    fn compute_call_fee_dock_(call: &<T as Config>::Call) -> Result<BalanceOf<T>, DispatchError> {
        use sp_std::convert::TryInto;
        let fee_fiat_permill: AmountUsd = Self::get_call_fee_fiat_(call);
        let fee_fiat_perbill: u32 = fee_fiat_permill.deconstruct() * 1000;

        let dock_fiat_rate_perbill: u32 = Self::dock_fiat_rate().deconstruct();
        let fee_dock_permill_u64: u64 = (fee_fiat_perbill as u64 * 1_000_000_u64)
            .checked_div(dock_fiat_rate_perbill as u64)
            .ok_or(Error::<T>::DivideByZero)?;
        let fee_dock_permill_u32: u32 = fee_dock_permill_u64
            .try_into()
            .or(Err(Error::<T>::ArithmeticOverflow))?;

        // The token has 6 decimal places (defined in the runtime)
        // pub const DOCK: Balance = 1_000_000;
        // T::Balance is already expressed in DOCK_Permill or microdock
        let fee_microdock = <BalanceOf<T>>::from(fee_dock_permill_u32);
        Ok(fee_microdock)
    }

    fn charge_fees_(who: T::AccountId, amount: BalanceOf<T>) -> Result<(), DispatchError> {
        let _ = <T::Currency>::withdraw(
            &who,
            amount,
            WithdrawReasons::FEE,
            ExistenceRequirement::KeepAlive,
        )?;
        Ok(())
    }

    fn execute_call_(origin: T::Origin, call: &<T as Config>::Call) -> DispatchResultWithPostInfo {
        let weight = call.get_dispatch_info().weight;
        // check signature before charging any fees
        let sender = ensure_signed(origin.clone())?;

        // calculate fee based on type of call
        let fee_dock = Self::compute_call_fee_dock_(&call)?;
        // deduct fees based on Currency::Withdraw
        Self::charge_fees_(sender, fee_dock)?;

        let dispatch_result = call.clone().dispatch(origin);
        return match dispatch_result {
            Ok(post_dispatch_info) => {
                sp_runtime::print(post_dispatch_info);
                Ok(PostDispatchInfo {
                    actual_weight: Some(weight),
                    pays_fee: Pays::No,
                })
            }
            Err(e) => {
                sp_runtime::print(e);
                Err(DispatchErrorWithPostInfo {
                    post_info: PostDispatchInfo {
                        actual_weight: Some(weight),
                        pays_fee: Pays::No,
                    },
                    error: e.error,
                })
            }
        };
    }
}

mod amount_fiat {
    // TODO intention is to replace AmountUsd with a trait AmountFiat
    // AmountFiat is a fraction (e.g. Perbill) and has a unit name (e.g. usd)

    // trait AmountFiat {
    //     const per_unit: u64;
    //     fn currency() -> FiatCurrency;
    // }
    // struct FiatCurrency {
    //     pub name: String,
    //     pub code: String,
    // }
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
}
