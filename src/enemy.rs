use bevy::prelude::*;
use crate::physics::{Position, Velocity};
use crate::spaceship::Spaceship;

pub struct EnemiesPlugin;

const ENEMIES_AMOUNT: u32 = 10;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies.in_set(EnemyStartupSet));
    }
}
pub fn setup_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_spaceship: Query<&Position, With<Spaceship>>
) {
    let player_position = q_spaceship.single();
    let mut spawn_pos = player_position.current + Vec2::new(ENEMIES_AMOUNT as f32 * 64. / -2., 200.);

    for _ in 0..10 {
        commands.spawn(EnemyBundle {
            sprite: SpriteBundle {
                texture: asset_server.load("enemy1.png"),
                ..default()
            },
            velocity: Velocity(Vec2::ZERO),
            position: Position { current: spawn_pos, previous: spawn_pos },
            marker: Enemy,
        });
        spawn_pos += Vec2::new(64., 0.);
    }

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

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemyStartupSet;