use bevy_asset::AssetPath;
use bevy_derive::Deref;
use bevy_ecs::resource::Resource;

/// Determines the name of the directory (within the `assets` directory) from where assets will be loaded.
///
/// By default, this is set to [`Self::DEFAULT_FOLDER_NAME`].
#[derive(Resource, Deref)]
pub struct LocalizationAssetFolder(AssetPath<'static>);

impl LocalizationAssetFolder {
    const DEFAULT_FOLDER_NAME: &'static str = "locale";

    pub fn new(path: impl Into<AssetPath<'static>>) -> Self {
        Self(path.into())
    }
}

impl Default for LocalizationAssetFolder {
    fn default() -> Self {
        Self(Self::DEFAULT_FOLDER_NAME.into())
    }
}

impl core::fmt::Display for LocalizationAssetFolder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
