use bevy_asset::{Handle, LoadedFolder};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::resource::Resource;

#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct LocalizationFolderHandle(pub(crate) Handle<LoadedFolder>);
