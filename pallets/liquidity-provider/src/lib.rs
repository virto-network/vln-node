#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod crypto;
mod liquidity_provider_balance;
#[cfg(test)]
mod mock;
mod module_impl;
#[cfg(test)]
mod tests;

use alloc::vec::Vec;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, traits::Get,
};
use frame_system::{
    ensure_none, ensure_signed,
    offchain::{AppCrypto, SendTransactionTypes, SigningTypes},
};
use orml_traits::{MultiCurrency, MultiReservableCurrency};
use valiu_node_commons::{AccountRate, Asset, DistributionStrategy, OfferRate, PairPrice};

pub use crypto::*;
pub use liquidity_provider_balance::*;
pub use module_impl::module_impl_offchain::*;

type AccountRateTy<T> = AccountRate<<T as frame_system::Trait>::AccountId, Balance<T>>;
type Balance<T> =
    <<T as Trait>::Collateral as MultiCurrency<<T as frame_system::Trait>::AccountId>>::Balance;
type OfferRateTy<T> = OfferRate<Balance<T>>;
type ProviderMembers = pallet_membership::DefaultInstance;

pub trait Trait:
    SendTransactionTypes<Call<Self>> + SigningTypes + pallet_membership::Trait<ProviderMembers>
where
    Balance<Self>: LiquidityProviderBalance,
{
    type Asset: MultiCurrency<Self::AccountId, Balance = Balance<Self>, CurrencyId = Asset>;
    type Collateral: MultiReservableCurrency<Self::AccountId, CurrencyId = Asset>;
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type OffchainAuthority: AppCrypto<Self::Public, Self::Signature>;
    type OffchainUnsignedGracePeriod: Get<Self::BlockNumber>;
    type OffchainUnsignedInterval: Get<Self::BlockNumber>;
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        Balance = Balance<T>,
    {
        Attestation(AccountId, Asset),
        Transfer(AccountId, AccountId, Balance),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        MustNotBeUsdv,
        NoFunds
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call
    where
        origin: T::Origin
    {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 0]
        pub fn attest(
            origin,
            asset: Asset,
            balance: Balance<T>,
            offer_rates: Vec<OfferRateTy<T>>
        ) -> DispatchResult
        {
            match asset {
                Asset::Usdv => Err(crate::Error::<T>::MustNotBeUsdv.into()),
                Asset::Collateral(collateral) => {
                    let who = ensure_signed(origin)?;
                    Self::update_account_rates(&who, asset, offer_rates);
                    Self::do_attest(who.clone(), Asset::Usdv, balance)?;
                    T::Collateral::deposit(collateral.into(), &who, balance)?;
                    T::Collateral::reserve(collateral.into(), &who, balance)?;
                    Self::deposit_event(RawEvent::Attestation(who, collateral.into()));
                    Ok(())
                },
                Asset::Btc | Asset::Cop | Asset::Ves => {
                    todo!()
                }
            }
        }

        #[weight = 0]
        pub fn submit_pair_prices(
            origin,
            pair_prices: Vec<PairPrice<Balance<T>>>,
            _signature: T::Signature,
        ) -> DispatchResult {
            ensure_none(origin)?;
            <PairPrices<T>>::mutate(|old_pair_prices| {
                if Self::incoming_pair_prices_are_valid(&pair_prices) {
                    old_pair_prices.clear();
                    old_pair_prices.extend(pair_prices);
                }
            });
            let current_block = <frame_system::Module<T>>::block_number();
            <NextUnsignedAt<T>>::put(current_block + T::OffchainUnsignedInterval::get());
            Ok(())
        }

        #[weight = 0]
        pub fn transfer(
            origin,
            to: <T as frame_system::Trait>::AccountId,
            to_amount: Balance<T>,
            ds: DistributionStrategy
        ) -> DispatchResult
        {
            let from = ensure_signed(origin)?;
            match ds {
                DistributionStrategy::Evenly => Self::transfer_evenly(from, to, to_amount)?
            }
            Ok(())
        }

        #[weight = 0]
        pub fn update_offer_rates(
            origin,
            asset: Asset,
            offer_rates: Vec<OfferRateTy<T>>
        ) -> DispatchResult
        {
            let who = ensure_signed(origin)?;
            Self::update_account_rates(&who, asset, offer_rates);
            Ok(())
        }

        fn offchain_worker(block_number: T::BlockNumber) {
            let _ = Self::fetch_pair_prices_and_submit_tx(block_number);
        }
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as LiquidityProviderStorage {
        pub AccountRates get(fn account_rates):
            double_map hasher(twox_64_concat) Asset,
            hasher(twox_64_concat) Asset => Vec<AccountRateTy<T>>;

        pub NextUnsignedAt get(fn next_unsigned_at): T::BlockNumber;

        pub PairPrices get(fn prices): Vec<PairPrice<Balance<T>>>
    }
}
