use bevy::prelude::*;

pub mod input;
pub mod movement;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_system(input::input_system)
        .add_system(movement::movement_system)
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
        .insert(movement::MoveDirection::default());
}
