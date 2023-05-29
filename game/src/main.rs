use bevy::prelude::*;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Component)]
struct PlayerController;

/// Raw input velocity
#[derive(Component, Default, Deref, DerefMut)]
struct MoveSpeed(f32);

#[derive(Component, Default, Deref, DerefMut)]
struct MoveDirection(Vec2);

impl Display for MoveDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_system(input_system)
        .add_system(input_movement_system)
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
        .insert(PlayerController)
        .insert(MoveSpeed(50.))
        .insert(MoveDirection::default());
}

fn input_system(
    kb_input: Res<Input<KeyCode>>,
    mut query: Query<&mut MoveDirection, With<PlayerController>>,
) {
    let mut input_direction = Vec2::ZERO;
    if kb_input.pressed(KeyCode::A) {
        input_direction.x += -1.;
    }
    if kb_input.pressed(KeyCode::D) {
        input_direction.x += 1.;
    }
    if kb_input.pressed(KeyCode::W) {
        input_direction.y += 1.;
    }
    if kb_input.pressed(KeyCode::S) {
        input_direction.y += -1.;
    }

    if input_direction != Vec2::ZERO {
        input_direction = input_direction.normalize();
    }

    for mut md in query.iter_mut() {
        *md = MoveDirection(input_direction);
    }
}

/// Update positions based on MoveDirection and MoveSpeed components
fn input_movement_system(
    mut query: Query<(&mut Transform, &MoveDirection, &MoveSpeed)>,
    time: Res<Time>,
) {
    for (mut transform, direction, speed) in query.iter_mut() {
        let move_vector = (*direction.deref() * *speed.deref()).extend(0.);
        transform.translation += move_vector * time.delta_seconds();
    }
}
