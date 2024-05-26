use bevy::prelude::*;
use bevy::math::Vec2;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, update_positions)
            .add_systems(Update, interpolate);
    }
}

fn interpolate(
    mut query: Query<(&mut Transform, &Position), With<Position>>,
    time: Res<Time<Fixed>>
) {
    let overstep = time.overstep_fraction();
    for (mut transform, position) in &mut query {
        transform.translation.x = position.previous.x + (position.current.x - position.previous.x) * overstep;
        transform.translation.y = position.previous.y + (position.current.y - position.previous.y) * overstep;
    }
}

fn update_positions
(
    mut query: Query<(&mut Position, &Velocity)>,
    time: Res<Time<Fixed>>
) {
    for (mut position, velocity) in &mut query {
        position.previous = position.current;
        position.current.x += velocity.0.x * time.delta_seconds();
        position.current.y += velocity.0.y * time.delta_seconds();
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Position {
    pub current: Vec2,
    pub previous: Vec2
}

