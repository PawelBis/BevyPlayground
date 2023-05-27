use std::fmt::{Display, Formatter};
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

#[derive(Resource, Default, Deref, DerefMut)]
struct InputAxis2(Vec2);

impl Display for InputAxis2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(InputAxis2::default())
        .add_startup_system(setup)
        .add_system(print_kb)
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
    commands.spawn(SpriteSheetBundle {
        texture_atlas,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_scale(Vec3::splat(2.)),
        ..default()
    });
}

fn print_kb(kb_input: Res<Input<KeyCode>>, mut input_axis2: ResMut<InputAxis2>) {
    let mut input_direction = Vec2::ZERO;
    if kb_input.pressed(KeyCode::A) {
        input_direction.x += -1.;
    }
    if kb_input.pressed(KeyCode::D) {
        input_direction.x += 1.;
    }
    if kb_input.pressed(KeyCode::W) {
        input_direction.y += -1.;
    }
    if kb_input.pressed(KeyCode::S) {
        input_direction.y += 1.;
    }

    if input_direction != Vec2::ZERO {
        input_direction = input_direction.normalize();
    }
    *input_axis2 = InputAxis2(input_direction);
}
