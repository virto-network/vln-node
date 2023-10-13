use super::*;

impl<T: Config> Pallet<T> {
	pub(crate) fn community_exists(community_id: &CommunityIdOf<T>) -> bool {
		Self::community(community_id).is_some()
	}

	/// Stores an initial info about the community
	/// Sets the caller as the community admin, the initial community state
	/// to its default value(awaiting)
	pub(crate) fn do_register_community(who: &AccountIdOf<T>, community_id: &CommunityIdOf<T>) -> DispatchResult {
		// Check that the community doesn't exist
		if Self::community_exists(community_id) {
			return Err(Error::<T>::CommunityAlreadyExists.into());
		}

		Info::<T>::insert(
			community_id,
			CommunityInfo {
				state: Default::default(),
			},
		);
		GovernanceStrategy::<T>::insert(community_id, CommunityGovernanceStrategy::AdminBased(who.clone()));

		Self::do_insert_member(community_id, who)?;

		Ok(())
	}

	pub(crate) fn do_set_metadata(community_id: &CommunityIdOf<T>, value: CommunityMetadata<T>) -> DispatchResult {
		Metadata::<T>::try_mutate(community_id, |metadata| {
			*metadata = Some(value);

			Ok(())
		})
	}
}
