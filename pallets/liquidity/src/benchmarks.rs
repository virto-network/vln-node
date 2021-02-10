use crate::{
    mock::{Origin, ProviderMembers, TestProvider, USD_ASSET},
    Balance, Call, Module, Trait,
};
use alloc::{boxed::Box, vec, vec::Vec};
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_runtime::traits::{IdentifyAccount, Verify};
use vln_commons::{Asset, Collateral, Destination};

type AccountId = <<sp_core::sr25519::Signature as Verify>::Signer as IdentifyAccount>::AccountId;

benchmarks! {
    where_clause {
        where
            <T as frame_system::Trait>::AccountId: AsRef<[u8; 32]>,
    }

    _ {}

    attest {
        let origin = gen_member_and_attest::<T>(USD_ASSET);
        let balance = Balance::<T>::from(100u32);
    }: attest(RawOrigin::Signed(origin), Asset::Collateral(Collateral::Usd), balance, Vec::new())
    verify {
    }

    members {
        let (origin, _) = gen_member::<T>();
    }: members(RawOrigin::Signed(origin))
    verify {
    }

    transfer {
        let from = gen_member_and_attest::<T>(USD_ASSET);
        let to: T::AccountId = whitelisted_caller();
        let to_amount = Balance::<T>::from(100u32);
    }: transfer(RawOrigin::Signed(from), Destination::Vln(to), to_amount)
    verify {
    }

    update_offer_rates {
        let from = gen_member_and_attest::<T>(USD_ASSET);
    }: update_offer_rates(RawOrigin::Signed(from), Asset::Btc, Vec::new())
    verify {
    }
}

fn gen_member<T>() -> (T::AccountId, AccountId)
where
    <T as frame_system::Trait>::AccountId: AsRef<[u8; 32]>,
    T: Trait,
{
    let from: T::AccountId = whitelisted_caller();
    let from_public = AccountId::from_raw(*from.as_ref());
    ProviderMembers::add_member(Origin::root(), from_public).unwrap();
    (from, from_public)
}

fn gen_member_and_attest<T>(asset: Asset) -> T::AccountId
where
    <T as frame_system::Trait>::AccountId: AsRef<[u8; 32]>,
    T: Trait,
{
    let (from, from_public) = gen_member::<T>();
    TestProvider::attest(Origin::signed(from_public), asset, 100, Default::default()).unwrap();
    from
}

#[cfg(test)]
mod tests {
    use crate::{
        benchmarks::{
            test_benchmark_attest, test_benchmark_transfer, test_benchmark_update_offer_rates,
        },
        mock::Test,
        tests::new_test_ext,
    };
    use frame_support::assert_ok;

    #[test]
    fn benchmarks_generate_unit_tests() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_attest::<Test>());
            assert_ok!(test_benchmark_transfer::<Test>());
            assert_ok!(test_benchmark_update_offer_rates::<Test>());
        });
    }
}
