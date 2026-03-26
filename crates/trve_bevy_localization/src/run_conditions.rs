use bevy_ecs::system::Res;

use crate::resources::LocalizationAssetsLoadState;

pub fn localization_assets_loaded(load_state: Res<LocalizationAssetsLoadState>) -> bool {
    load_state.is_loaded()
}

pub fn localization_assets_loading(load_state: Res<LocalizationAssetsLoadState>) -> bool {
    load_state.is_loading()
}

pub fn localization_assets_loading_failed(load_state: Res<LocalizationAssetsLoadState>) -> bool {
    load_state.is_failed()
}
