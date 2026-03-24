use bevy_asset::RecursiveDependencyLoadState;
use bevy_derive::Deref;
use bevy_ecs::resource::Resource;

#[derive(Resource, Deref)]
pub struct LocalizationAssetsLoadState(pub(crate) RecursiveDependencyLoadState);

impl Default for LocalizationAssetsLoadState {
    fn default() -> Self {
        Self(RecursiveDependencyLoadState::NotLoaded)
    }
}
