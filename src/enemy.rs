use bevy::prelude::*;
use crate::physics::{Position, Velocity};
use crate::spaceship::Spaceship;

pub struct EnemiesPlugin;

const ENEMIES_AMOUNT: u32 = 10;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Damage>()
            .add_systems(Startup, setup_enemies.in_set(EnemyStartupSet))
            .add_systems(FixedUpdate, handle_damage);
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
        for _ in 0..10 {
            commands.spawn(EnemyBundle {
                sprite: SpriteBundle {
                    texture: asset_server.load("enemy1.png"),
                    ..default()
                },
                velocity: Velocity(Vec2::ZERO),
                position: Position { current: spawn_pos, previous: spawn_pos },
                marker: Enemy,
                health: Health(100.0),
                collider: Collider(Circle::new(16.0))
            });
            spawn_pos += Vec2::new(64., 0.);
        }
        spawn_pos.x = ENEMIES_AMOUNT as f32 * 64. / -2.;
        spawn_pos.y += 64.;
    }

}

pub fn handle_damage(
    mut commands: Commands,
    mut events: EventReader<Damage>,
    mut q_enemies: Query<&mut Health, With<Enemy>>
) {
    for Damage(entity, damage) in events.read() {
        let mut health = q_enemies.get_mut(*entity);
        if let Ok(mut health) = health {
            health.0 -= damage;
            if health.0 <= 0.0 {
                commands.entity(*entity).despawn();
            }
        }
    }
}


#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health(f32);

#[derive(Event)]
pub struct Damage(pub Entity, pub f32);

#[derive(Component)]
pub struct Collider(pub Circle);

#[derive(Bundle)]
struct EnemyBundle {
    pub sprite: SpriteBundle,
    pub velocity: Velocity,
    pub position: Position,
    pub marker: Enemy,
    pub health: Health,
    pub collider: Collider
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemyStartupSet;