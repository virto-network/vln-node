use frame_support::traits::VoteTally;

pub type VoteWeight = sp_runtime::Perbill;

/// This structure holds a governance strategy. This defines how to behave
/// when ensuring privileged calls and deciding executing
/// calls
#[derive(TypeInfo, Encode, Decode, MaxEncodedLen, Clone, Eq, PartialEq, Debug)]
#[scale_info(skip_type_params(AccountId, AssetId))]
pub enum CommunityGovernanceStrategy<AccountId, AssetId> {
	/// The community governance lies in the shoulders of the admin of it.
	///
	/// This is equivalent to `RawOrigin::Member` on collectives-pallet, or
	/// `BodyPart::Voice` on XCM.
	AdminBased(AccountId),
	/// The community governance relies on a member count-based (one member,
	/// one vote) poll.
	///
	/// This is equivalent to `RawOrigin::Members` on collectives-pallet, or
	/// `BodyPart::Members` on XCM.
	MemberCountPoll { min: VoteWeight },
	/// The community governance relies on an asset-weighed (one token,
	/// one vote) poll.
	///
	/// This is equivalent to `RawOrigin::Members` on collectives-pallet, or
	/// `BodyPart::Fraction` on XCM.
	AssetWeighedPoll {
		asset_id: AssetId,
		min_approval: VoteWeight,
	},
	/// The community governance relies on an ranked-weighed (one member vote,
	/// the number of votes corresponding to the rank of member) poll,
	///
	/// This is equivalent to `RawOrigin::Members` on collectives-pallet, or
	/// `BodyPart::Fraction` on XCM.
	RankedWeighedPoll { min_approval: VoteWeight },
}

///
#[derive(TypeInfo, Encode, Decode, Debug, PartialEq, Clone)]
pub enum Vote<AssetId, AssetBalance> {
	AssetBalance(bool, AssetId, AssetBalance),
	Standard(bool),
}

impl<A, B> From<Vote<A, B>> for VoteWeight {
	fn from(_value: Vote<A, B>) -> Self {
		todo!()
	}
}

///
#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: Config))]
pub struct Tally<T>(core::marker::PhantomData<T>);

impl<T: Config> VoteTally<VoteWeight, T::CommunityId> for Tally<T> {
	fn new(_: T::CommunityId) -> Self {
		todo!()
	}

	fn ayes(&self, _cid: T::CommunityId) -> VoteWeight {
		todo!()
	}

	fn support(&self, _cid: T::CommunityId) -> sp_runtime::Perbill {
		todo!()
	}

	fn approval(&self, _cid: T::CommunityId) -> sp_runtime::Perbill {
		todo!()
	}
}
