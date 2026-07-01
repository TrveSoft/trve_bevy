use bevy_app::{App, Plugin, Update};
use bevy_asset::{AssetApp, AssetEvent, AssetServer, Assets};
use bevy_derive::Deref;
use bevy_ecs::{
    message::MessageReader,
    resource::Resource,
    schedule::{IntoScheduleConfigs, common_conditions::on_message},
    system::{Res, ResMut},
};
use bevy_platform::collections::HashMap;

use crate::prelude::{AttributionAsset, AttributionAssetLoader, AttributionMetadata};

pub struct AttributionPlugin;

impl Plugin for AttributionPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AttributionAsset>();
        app.init_asset_loader::<AttributionAssetLoader>();
        app.init_resource::<Attributions>();
        app.add_systems(
            Update,
            on_asset_loaded.run_if(on_message::<AssetEvent<AttributionAsset>>),
        );
    }
}

#[derive(Resource, Default, Deref)]
pub struct Attributions(HashMap<String, AttributionMetadata>);

fn on_asset_loaded(
    mut messages: MessageReader<AssetEvent<AttributionAsset>>,
    mut attributions: ResMut<Attributions>,
    assets: Res<Assets<AttributionAsset>>,
    asset_server: Res<AssetServer>,
) {
    for message in messages.read() {
        if let AssetEvent::LoadedWithDependencies { id } = *message
            && let Some(asset) = assets.get(id)
        {
            let key = asset_server
                .get_path(id)
                .map_or(asset.meta.name.clone(), |path| path.to_string());

            if !attributions.contains_key(&key) {
                #[cfg(debug_assertions)]
                bevy_log::info!("Attribution file for \"{key}\" asset was loaded");
                attributions.0.insert(key, asset.meta.clone());
            }
        }
    }
}
