use bevy::prelude::*;
use std::collections::HashMap;
use bevy::asset::LoadState;
use crate::assets::animation_descriptor::AnimationDescriptor;

/// List of `AnimationDescriptors` that should be loaded
/// `animation_descriptor_loader` is going through this list and schedules loading
#[derive(Resource)]
pub struct AnimationDescriptorLoadQueue {
    pub queue: Vec<String>,
}

#[derive(Resource)]
pub struct AssetStore {
    pub animation_descriptors: HashMap<String, Handle<AnimationDescriptor>>,
}

/// Go through `AnimationDescriptorLoadQueue` and load all pending assets
pub fn animation_descriptor_loader(
    mut load_queue: ResMut<AnimationDescriptorLoadQueue>,
    asset_store: ResMut<AssetStore>,
    asset_server: Res<AssetServer>,
) {
    for animation_path in &load_queue.queue.into_iter() {
        let adventurer_handle: Handle<AnimationDescriptor> = asset_server.load(animation_path);
        asset_store.animation_descriptors.insert(animation_path, adventurer_handle);
    }
}

/// Load sprites used by `SpriteAnimationPlayer` and create corresponding `TextureAtlas`
pub fn animation_sprite_sheet_loader(
    asset_store: Res<AssetStore>,
    asset_server: Res<AssetServer>,
    mut animation_descriptors: ResMut<Assets<AnimationDescriptor>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for animation_descriptor_handle in asset_store.animation_descriptors.values() {
        match asset_server.get_load_state(animation_descriptor_handle) {
            LoadState::Loaded => {
                let mut animation_descriptor = animation_descriptors
                    .get_mut(animation_descriptor_handle)
                    .expect("Asset is loaded, we checked the state");
                let texture_h = asset_server.load(&animation_descriptor.source_sprite);
                let texture_atlas = TextureAtlas::from_grid(
                    texture_h,
                    animation_descriptor.frame_size,
                    animation_descriptor.columns as usize,
                    animation_descriptor.rows as usize,
                    None,
                    None,
                );
                let atlas_h = texture_atlases.add(texture_atlas);
                animation_descriptor.atlas_handle = Some(atlas_h);
            }
            // TODO: Remove handles that failed to load
            LoadState::Failed => {

            }
            _ => {}
        }
    }
}
