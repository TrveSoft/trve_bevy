use bevy_app::{App, Plugin, Startup, Update};
use bevy_asset::{AssetServer, Assets, LoadedFolder};
use bevy_ecs::{
    message::MessageWriter,
    schedule::common_conditions::{not, on_message},
    schedule::{IntoScheduleConfigs, SystemCondition},
    system::{Commands, Res, ResMut},
};
use bevy_fluent::LocalizationBuilder;

use crate::{
    events::LocalizationAssetsFinishedLoading,
    prelude::{
        LocalizationAssetFolder, localization_assets_loaded, localization_assets_loading_failed,
    },
    resources::{LocalizationAssetsLoadState, LocalizationFolderHandle},
};

pub(crate) struct LocalizationAssetLoadingPlugin;

impl Plugin for LocalizationAssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LocalizationAssetsFinishedLoading>();

        app.init_resource::<LocalizationAssetsLoadState>();

        app.add_systems(Startup, (setup_resources, load_assets).chain());
        app.add_systems(
            Update,
            (
                update_assets_load_state.run_if(
                    not(localization_assets_loaded)
                        .and_then(not(localization_assets_loading_failed)),
                ),
                init_localization_resource.run_if(on_message::<LocalizationAssetsFinishedLoading>),
            )
                .chain(),
        );
    }
}

fn setup_resources(mut commands: Commands) {
    commands.init_resource::<LocalizationAssetFolder>();
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    folder: Res<LocalizationAssetFolder>,
) {
    commands.insert_resource(LocalizationFolderHandle(
        asset_server.load_folder(&**folder),
    ));
}

fn update_assets_load_state(
    mut message_writer: MessageWriter<LocalizationAssetsFinishedLoading>,
    mut load_state: ResMut<LocalizationAssetsLoadState>,
    asset_server: Res<AssetServer>,
    folder_handle: Res<LocalizationFolderHandle>,
) {
    load_state.0 = asset_server.recursive_dependency_load_state(folder_handle.id());

    if load_state.is_loaded() {
        message_writer.write_default();
    }
}

fn init_localization_resource(
    mut commands: Commands,
    folders: Res<Assets<LoadedFolder>>,
    folder_handle: Res<LocalizationFolderHandle>,
    localization_builder: LocalizationBuilder,
) {
    if let Some(folder) = folders.get(folder_handle.id()) {
        if folder.handles.is_empty() {
            bevy_log::warn!(
                "Folder `{}` is empty or does not exist",
                folder_handle
                    .path()
                    .map(|p| p.to_string())
                    .unwrap_or_default()
            );
        } else {
            commands.insert_resource(localization_builder.build(&folder_handle));
        }
    }
}
