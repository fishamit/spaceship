use crate::bullet::{Bullet, BulletBundle, BulletTimer};
use crate::enemy::setup_enemies;
use crate::input::InputState;
use crate::physics::{Position, Velocity};
use bevy::math::Vec2;
use bevy::prelude::*;
use rand::Rng;

const ACCELERATION: f32 = 2500.;
const MAX_VELOCITY: f32 = 400.;

const ZERO_VELOCITY: f32 = 0.;

const IDLE_BREAK_SPEED: f32 = 1200.;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship.before(setup_enemies))
            .add_systems(
                FixedUpdate,
                (
                    handle_spaceship_movement,
                    velocity_guard.after(handle_spaceship_movement),
                ),
            );
    }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpaceshipBundle {
            sprite: SpriteBundle {
                texture: asset_server.load("spaceship.png"),
                transform: Transform::from_xyz(100., 0., 1.),
                ..Default::default()
            },
            state: SpaceshipState {
                shot_ready: true
            },
            velocity: Velocity(Vec2::ZERO),
            position: Position {
                current: Vec2::ZERO,
                previous: Vec2::ZERO,
            },
            marker: Spaceship { shot_ready: true },
            gun_timer: GunTimer(Timer::from_seconds(0.02, TimerMode::Repeating)),
        })
        .with_children(|parent| {
            parent.spawn(FireBundle {
                sprite: SpriteBundle {
                    texture: asset_server.load("fire2.png"),
                    visibility: Visibility::Hidden,
                    transform: Transform::from_xyz(0., -32., 2.),
                    ..Default::default()
                },
                marker: Fire,
            });
            parent.spawn(BoostFireBundle {
                sprite: SpriteBundle {
                    texture: asset_server.load("fire.png"),
                    visibility: Visibility::Hidden,
                    transform: Transform::from_xyz(0., -32., 2.),
                    ..Default::default()
                },
                marker: BoostFire,
            });
        });
}

fn velocity_guard(
    mut query: Query<&mut Velocity, With<Spaceship>>,
    input_state: Query<&InputState>,
    time: Res<Time>,
) {
    let mut velocity = query.single_mut();
    let input_state = input_state.single();
    let is_idle = input_state.idle;
    if velocity.0.y > MAX_VELOCITY {
        velocity.0.y = MAX_VELOCITY
    };
    if velocity.0.y < -MAX_VELOCITY {
        velocity.0.y = -MAX_VELOCITY
    };
    if velocity.0.x > MAX_VELOCITY {
        velocity.0.x = MAX_VELOCITY
    };
    if velocity.0.x < -MAX_VELOCITY {
        velocity.0.x = -MAX_VELOCITY
    };

    if is_idle {
        let mut new_velocity = velocity.0;
        // x axis
        if new_velocity.x < 0. {
            new_velocity.x += IDLE_BREAK_SPEED * time.delta_seconds();
            if new_velocity.x > 0. {
                new_velocity.x = 0.
            }
        } else if new_velocity.x > 0. {
            new_velocity.x -= IDLE_BREAK_SPEED * time.delta_seconds();
            if new_velocity.x < 0. {
                new_velocity.x = 0.
            }
        }
        // y axis
        if new_velocity.y < ZERO_VELOCITY {
            new_velocity.y += IDLE_BREAK_SPEED * time.delta_seconds();
            if new_velocity.y > ZERO_VELOCITY {
                new_velocity.y = ZERO_VELOCITY
            }
        } else if new_velocity.y > ZERO_VELOCITY {
            new_velocity.y -= IDLE_BREAK_SPEED * time.delta_seconds();
            if new_velocity.y < ZERO_VELOCITY {
                new_velocity.y = ZERO_VELOCITY
            }
        }

        if velocity.0 != new_velocity {
            velocity.0 = new_velocity
        };
    }
}

fn handle_spaceship_movement(
    time: Res<Time>,
    mut q_spaceship: Query<
        (
            &mut Position,
            &mut Velocity,
            &mut GunTimer,
            &mut SpaceshipState,
        ),
        With<Spaceship>,
    >,
    mut q_fire: Query<&mut Visibility, (With<Fire>, Without<Spaceship>, Without<BoostFire>)>,
    mut q_boost_fire: Query<&mut Visibility, (With<BoostFire>, Without<Spaceship>, Without<Fire>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input_state: Query<&InputState>,
) {
    let (mut position, mut velocity, mut gun_timer, mut spaceship_state) = q_spaceship.single_mut();
    let mut vis_fire = q_fire.single_mut();
    let mut vis_boost_fire = q_boost_fire.single_mut();
    let input_state = input_state.single();

    *vis_fire = if input_state.up && !input_state.boost {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    *vis_boost_fire = if input_state.up && input_state.boost {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    if input_state.up {
        velocity.0.y += ACCELERATION * time.delta_seconds();
    }
    if input_state.down {
        velocity.0.y -= ACCELERATION * time.delta_seconds();
    }
    if input_state.right {
        velocity.0.x += ACCELERATION * time.delta_seconds();
    }
    if input_state.left {
        velocity.0.x -= ACCELERATION * time.delta_seconds();
    }
    if input_state.shooting {
        handle_fire(
            &mut commands,
            &mut gun_timer,
            &asset_server,
            &mut position,
            &mut velocity,
            &mut spaceship_state,
        );
    }

    if gun_timer.0.tick(time.delta()).finished() {
        spaceship_state.shot_ready = true;
    }
}

fn handle_fire(
    mut commands: &mut Commands,
    mut gun_timer: &mut GunTimer,
    asset_server: &AssetServer,
    mut position: &mut Position,
    mut velocity: &mut Velocity,
    mut spaceship_state: &mut SpaceshipState,
) {
    if spaceship_state.shot_ready {
        gun_timer.0.reset();
        let x_right = rand::thread_rng().gen_range(-40.0..40.);
        let x_left = rand::thread_rng().gen_range(-40.0..40.);
        let bullet_velocity = 400. + if velocity.0.y > 0. { velocity.0.y } else { 0. };
        commands.spawn(BulletBundle {
            sprite: SpriteBundle {
                texture: asset_server.load("bullet.png"),
                ..Default::default()
            },
            position: Position {
                current: Vec2::new(position.current.x + 10., position.current.y + 8.),
                previous: Vec2::new(position.current.x + 10., position.current.y + 8.),
            },
            velocity: Velocity(Vec2::new(x_right, bullet_velocity)),
            marker: Bullet,
            timer: BulletTimer(Timer::from_seconds(10.5, TimerMode::Once)),
        });
        commands.spawn(BulletBundle {
            sprite: SpriteBundle {
                texture: asset_server.load("bullet.png"),
                ..Default::default()
            },
            velocity: Velocity(Vec2::new(x_left, bullet_velocity)),
            position: Position {
                current: Vec2::new(position.current.x - 10., position.current.y + 8.),
                previous: Vec2::new(position.current.x - 10., position.current.y + 8.),
            },
            marker: Bullet,
            timer: BulletTimer(Timer::from_seconds(10.5, TimerMode::Once)),
        });
        spaceship_state.shot_ready = false;
    }
}

#[derive(Component)]
pub struct Spaceship {
    shot_ready: bool,
}

#[derive(Component)]
pub struct SpaceshipState {
    shot_ready: bool
}

struct Movement {
    up: bool,
    right: bool,
    left: bool,
    down: bool,
    idle: bool,
}

#[derive(Component)]
pub struct Fire;

#[derive(Component)]
pub struct BoostFire;

#[derive(Component)]
pub struct GunTimer(Timer);

#[derive(Bundle)]
pub struct SpaceshipBundle {
    pub sprite: SpriteBundle,
    pub velocity: Velocity,
    pub position: Position,
    pub marker: Spaceship,
    pub gun_timer: GunTimer,
    pub state: SpaceshipState,
}

#[derive(Bundle)]
pub struct FireBundle {
    pub sprite: SpriteBundle,
    pub marker: Fire,
}

#[derive(Bundle)]
pub struct BoostFireBundle {
    pub sprite: SpriteBundle,
    pub marker: BoostFire,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpaceshipStartupSet;
