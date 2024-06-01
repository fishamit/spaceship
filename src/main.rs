mod camera;
mod spaceship;
mod input;
mod bullet;
mod stars;
mod physics;
mod enemy;
mod resource_manager;
mod explosion;

use crate::input::InputPlugin;
use bevy::prelude::*;
use crate::bullet::BulletPlugin;
use crate::camera::CameraPlugin;
use crate::enemy::EnemiesPlugin;
use crate::explosion::ExplosionsPlugin;
use crate::physics::PhysicsPlugin;
use crate::spaceship::SpaceshipPlugin;
use crate::stars::StarsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins((
            StarsPlugin,
            InputPlugin,
            CameraPlugin,
            SpaceshipPlugin,
            ExplosionsPlugin,
            EnemiesPlugin,
            BulletPlugin,
            PhysicsPlugin
        ))
        .run();
}







