mod events;
mod loading;
mod localization_manager;
mod plugin;
mod resources;
mod run_conditions;
mod text;

const DEFAULT_LANG_ID: unic_langid::LanguageIdentifier = unic_langid::langid!("en-US");

pub mod prelude {
    use super::*;

    pub use events::LocaleChanged;
    pub use localization_manager::LocalizationManager;
    pub use plugin::LocalizationPlugin;
    pub use resources::{
        DefaultLocale, LocalizationAssetFolder, LocalizationAssetsLoadState, SupportedLocales,
    };
    pub use run_conditions::{
        localization_assets_loaded, localization_assets_loading, localization_assets_loading_failed,
    };
    pub use text::{LocalizedText, LocalizedTextSpan};
}

pub use unic_langid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_lang_id_to_string() {
        assert_eq!(DEFAULT_LANG_ID.to_string(), String::from("en-US"));
    }
}
