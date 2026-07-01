use bevy_asset::Asset;
use bevy_reflect::TypePath;
use serde::Deserialize;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct AttributionAsset {
    pub(crate) meta: AttributionMetadata,
}

impl AttributionAsset {
    pub fn metadata(&self) -> &AttributionMetadata {
        &self.meta
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AttributionMetadata {
    /// The asset's name.
    pub(crate) name: String,
    /// The asset's author name or pseudonym.
    pub(crate) author: String,
    /// A list of applicable licenses for this asset.
    ///
    /// e.g.: "CC-BY-SA 3.0", "GPL 2.0", etc.
    pub(crate) license: Vec<String>,
    /// An optional link to the asset or
    /// the author's website.
    /// Its usage is encouraged.
    ///
    /// If left blank (i.e. `link = ""`)
    /// this field will have a [`None`] value.
    pub(crate) link: Option<String>,
    /// An optional description for the asset.
    /// Could be used for internal purposes,
    /// more detailed licencing or attribution text, etc.
    ///
    /// If left blank (i.e. `description = ""`)
    /// this field will have a [`None`] value.
    pub(crate) description: Option<String>,
}

impl AttributionMetadata {
    pub fn author(&self) -> &String {
        &self.author
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn license(&self) -> &Vec<String> {
        &self.license
    }

    pub fn link(&self) -> &Option<String> {
        &self.link
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
