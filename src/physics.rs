use bevy::prelude::*;
use bevy::math::Vec2;
use crate::bullet::Bullet;
use crate::enemy::{Collider, Damage, Enemy};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, (update_positions, handle_collisions))
            .add_systems(Update, interpolate);
    }
}

fn interpolate(
    mut query: Query<(&mut Transform, &Position), With<Position>>,
    time: Res<Time<Fixed>>,
    // time_unfixed: Res<Time>
) {
    let overstep = time.overstep_fraction();
    for (mut transform, position) in &mut query {
        transform.translation.x = position.previous.x + (position.current.x - position.previous.x) * overstep;
        transform.translation.y = position.previous.y + (position.current.y - position.previous.y) * overstep;
    }
    // dbg!(1. / time_unfixed.delta_seconds());
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

fn handle_collisions
(
    mut commands: Commands,
    mut q_bullets: Query<(Entity, &Position), With<Bullet>>,
    mut e_damage: EventWriter<Damage>,
    q_enemies: Query<(Entity, &Position, &Collider), With<Enemy>>
) {
    for (enemy_entity, enemy_position, enemy_collider) in q_enemies.iter() {
        for (bullet_ent, bullet_pos) in q_bullets.iter_mut() {
            if (bullet_pos.current - enemy_position.current).length_squared() <= enemy_collider.0.radius.powi(2) {
                // TODO despawning twice bug? processing collision over and over
                commands.entity(bullet_ent).despawn();
                e_damage.send(Damage(enemy_entity, 100.));
            }
        }
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component, Copy, Clone)]
pub struct Position {
    pub current: Vec2,
    pub previous: Vec2
}
