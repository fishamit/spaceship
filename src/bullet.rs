use bevy::prelude::*;
use crate::physics::{Position, Velocity};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, handle_bullets);
    }
}
#[derive(Bundle)]
pub struct BulletBundle {
    pub sprite: SpriteBundle,
    pub velocity: Velocity,
    pub position: Position,
    pub marker: Bullet,
    pub timer: BulletTimer
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletTimer(pub Timer);

#[derive(Component)]
struct BulletSpawnerTimer(pub Timer);

fn handle_bullets(
    mut q_bullets: Query<(Entity, &mut BulletTimer), With<Bullet>>,
    mut commands: Commands,
    time: Res<Time>
) {
    for (entity, mut timer) in q_bullets.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
