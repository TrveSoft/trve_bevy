use bevy_asset::{AssetLoader, LoadContext, io::Reader};
use bevy_log::warn;
use bevy_reflect::TypePath;
use thiserror::Error;

use crate::{prelude::AttributionAsset, valid_licenses::VALID_LICENSES};

#[derive(Default, TypePath)]
pub struct AttributionAssetLoader;

/// Possible errors that can be produced by [`AttributionAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AttributionAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [TOML](toml) Error
    #[error("Could not parse TOML: {0}")]
    TomlError(#[from] toml::de::Error),
}

impl AssetLoader for AttributionAssetLoader {
    type Asset = AttributionAsset;
    type Settings = ();
    type Error = AttributionAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();

        reader.read_to_end(&mut bytes).await?;

        let mut asset = toml::de::from_slice::<AttributionAsset>(&bytes)?;
        let asset_name = asset.meta.name();

        for license in asset.meta.license.iter() {
            if !VALID_LICENSES.contains(&license.as_str()) {
                warn!("Invalid license \"{license}\" for asset \"{asset_name}\"");
            }
        }

        let is_empty_string = |s: &String| s.is_empty();

        if asset.meta.description.as_ref().is_some_and(is_empty_string) {
            asset.meta.description = None;
        }

        if asset.meta.link.as_ref().is_some_and(is_empty_string) {
            asset.meta.link = None;
        }

        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["attr.toml"]
    }
}
