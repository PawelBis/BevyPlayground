use bevy::prelude::*;
use crate::movement::MoveDirection;

#[derive(Component)]
pub struct PlayerController;

pub fn input_system(
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

