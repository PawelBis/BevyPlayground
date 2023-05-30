use bevy::prelude::*;
use std::fmt::{Display, Formatter};

/// Raw input velocity
#[derive(Component, Default)]
pub struct MoveSpeed(pub f32);

impl MoveSpeed {
    pub fn as_f32(&self) -> f32 {
        self.0
    }
}

#[derive(Component, Default)]
pub struct MoveDirection(pub Vec2);

impl MoveDirection {
    pub fn as_vec2(&self) -> Vec2 {
        self.0
    }
}

impl Display for MoveDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}


/// Update positions based on MoveDirection and MoveSpeed components
pub fn movement_system(
    mut query: Query<(&mut Transform, &MoveDirection, &MoveSpeed)>,
    time: Res<Time>,
) {
    for (mut transform, direction, speed) in query.iter_mut() {
        let move_vector = (direction.as_vec2() * speed.as_f32()).extend(0.);
        transform.translation += move_vector * time.delta_seconds();
    }
}

