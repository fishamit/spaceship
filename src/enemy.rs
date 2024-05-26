use bevy::prelude::*;
use crate::physics::{Position, Velocity};
use crate::spaceship::{GunTimer, Spaceship, SpaceshipState};

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies);
    }
}

fn setup_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(EnemyBundle {
        sprite: SpriteBundle {
            texture: asset_server.load("enemy1.png"),
            ..default()
        },
        velocity: Velocity(Vec2::ZERO),
        position: Position { current: Vec2::ZERO, previous: Vec2::ZERO },
        marker: Enemy,
    });
}

#[derive(Component)]
struct Enemy;

#[derive(Bundle)]
struct EnemyBundle {
    pub sprite: SpriteBundle,
    pub velocity: Velocity,
    pub position: Position,
    pub marker: Enemy,
}