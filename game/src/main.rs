use crate::animation::AnimationPlayerLoader;
use bevy::prelude::*;
use engine::assets::animation_descriptor::{AnimationDescriptor, AnimationDescriptorLoader};
use engine::assets::asset_store::{animation_descriptor_loader, animation_sprite_sheet_loader, AnimationDescriptorLoadQueue, AssetStore};

pub mod animation;
pub mod input;
pub mod movement;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(AssetStore::default())
        .insert_resource(AnimationDescriptorLoadQueue {
            queue: vec!["adventurer.ron".to_owned()],
        })
        .add_startup_system(setup)
        .add_asset::<AnimationDescriptor>()
        .init_asset_loader::<AnimationDescriptorLoader>()
        .add_system(animation_descriptor_loader)
        .add_system(animation_sprite_sheet_loader.after(animation_descriptor_loader))
        .add_system(input::input_system)
        .add_system(movement::movement_system)
        .add_system(animation::animation_system)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture = asset_server.load("adventurer.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(50., 37.), 7, 11, None, None);
    let texture_atlas = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(2.)),
            ..default()
        })
        .insert(input::PlayerController)
        .insert(movement::MoveSpeed(50.))
        .insert(movement::MoveDirection::default())
        .insert(animation::AnimationTimer(Timer::from_seconds(
            1.,
            TimerMode::Repeating,
        )))
        .insert(animation::Animation {
            length: 1.,
            first_frame: 2,
            frame_count: 2,
            looping: true,
        });
}
