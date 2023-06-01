use bevy::prelude::*;
use std::collections::HashMap;
use bevy::asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset};
use bevy::reflect::TypeUuid;
use serde::Deserialize;

/// Component used for frame time tracking
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/// Animation component flips sprite from `first_frame` to `last_frame`(inclusive) with option
/// to repeat on finish.
/// Requires a valid `TextureAtlas` and `AnimationTimer`
#[derive(Component, Debug, Deserialize, Clone)]
pub struct Animation {
    /// Determines length and frame time of the animation
    /// frame_time = length / (last_frame - first_frame)
    pub length: f32,
    /// Index of the first frame in the `TextureAtlas`
    pub first_frame: u8,
    /// Index of the last frame in the `TextureAtlas`
    pub frame_count: u8,
    /// Whenever the animation should repeat after it finishes
    pub looping: bool,
}

impl Animation {
    fn last_frame(&self) -> u8 {
        self.first_frame + self.frame_count
    }
}

/// Stores data about the animations and provides an interface for animations manipulation
#[derive(Component, Debug, Deserialize, TypeUuid, Clone)]
#[uuid = "3072233a-9066-44dc-9d21-03e361a3c1f8"]
pub struct AnimationPlayer {
    pub source_sprite: String,
    pub animations: HashMap<String, Animation>,
}

#[derive(Default)]
pub struct AnimationPlayerLoader;

impl AssetLoader for AnimationPlayerLoader {
    fn load<'a>(&'a self, bytes: &'a [u8], load_context: &'a mut LoadContext) -> BoxedFuture<'a, anyhow::Result<(), Error>> {
        Box::pin(async move {
            let animation_player = ron::de::from_bytes::<AnimationPlayer>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(animation_player));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

/// System responsible for advancing `AnimationTimer` and flipping sprite sheet
pub fn animation_system(
    time: Res<Time>,
    mut query: Query<(&Animation, &mut AnimationTimer, &mut TextureAtlasSprite)>
) {
    for (animation, mut timer, mut atlas) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let is_last_frame = atlas.index == animation.last_frame() as usize;
            if is_last_frame && !animation.looping {
                return;
            }
        }

        let frame_offset = (timer.elapsed_secs() / animation.length) * animation.frame_count as f32;
        atlas.index = animation.first_frame as usize + frame_offset as usize;
    }
}