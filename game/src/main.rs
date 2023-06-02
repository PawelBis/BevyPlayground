use crate::animation::AnimationPlayerLoader;
use bevy::prelude::*;

pub mod animation;
pub mod input;
pub mod movement;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_system(setup_animation_player)
        .add_asset::<animation::AnimationPlayer>()
        .init_asset_loader::<AnimationPlayerLoader>()
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

fn setup_animation_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<Entity, With<input::PlayerController>>,
    animation_players: ResMut<Assets<animation::AnimationPlayer>>,
) {
    let animation_player: Handle<animation::AnimationPlayer> = asset_server.load("adventurer.ron");
    let ap = animation_players.get(&animation_player);
    info!("{:?}", ap);

    for entity in query.iter_mut() {
        //commands.entity(entity).insert(ap.clone());
    }
}
