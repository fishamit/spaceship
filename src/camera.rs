use bevy::app::{App, Startup};
use bevy::prelude::*;
use bevy::prelude::TimerMode::Repeating;
use bevy::render::camera::ScalingMode;
use rand::Rng;
use crate::spaceship::{Spaceship};
use crate::input::{InputState};
use crate::physics::Position;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(FixedUpdate, (camera_follow, move_camera, update_visible_space).chain());

    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(360.0);
    commands.spawn(VisibleSpace {
        top_left: Vec2::ZERO,
        bottom_right: Vec2::ZERO,
    });
    commands.spawn((
        camera,
        Camera,
        Position { current: Vec2::ZERO, previous: Vec2::ZERO },
        CameraData {
            target_scale: 0.9,
            zoom_speed: 0.75,
            move_speed: 500.,
            shake: false,
            shake_timer: Timer::from_seconds(0.03, Repeating),
            max_zoom: 1.5,
            min_zoom: 0.9
        }
    ));
}

#[derive(Component)]
pub struct Camera;

#[derive(Component)]
pub struct CameraData {
    pub target_scale: f32,
    pub zoom_speed: f32,
    pub move_speed: f32,
    pub shake: bool,
    pub shake_timer: Timer,
    pub max_zoom: f32,
    pub min_zoom: f32
}

#[derive(Component, Debug)]
pub struct VisibleSpace {
    pub top_left: Vec2,
    pub bottom_right: Vec2,
}


fn move_camera(
    mut q_cam: Query<(&mut OrthographicProjection, &mut CameraData, &mut Position), With<CameraData>>,
    time: Res<Time>,
    input_state: Query<&InputState>
) {
    let (mut projection, mut camera_data, mut position) = q_cam.single_mut();
    let input_state = input_state.single();
    let target_scale = camera_data.target_scale;


    if input_state.boost {
        camera_data.target_scale = camera_data.max_zoom;
        camera_data.shake = true;
    } else {
        camera_data.target_scale = camera_data.min_zoom;
        camera_data.shake = false;
    }

    // handle camera zoom
    if projection.scale != target_scale {
        projection.scale += camera_data.zoom_speed * time.delta_seconds() * (target_scale - projection.scale).signum();
        if target_scale == camera_data.min_zoom && projection.scale < camera_data.min_zoom { projection.scale = camera_data.min_zoom }
        else if target_scale == camera_data.max_zoom && projection.scale > camera_data.max_zoom { projection.scale = camera_data.max_zoom }
    }

    if camera_data.shake && camera_data.shake_timer.tick(time.delta()).finished() {
        let x = rand::thread_rng().gen_range(-7f32..7f32);
        let y = rand::thread_rng().gen_range(-20f32..20f32);
        position.previous = position.current;
        position.current.x += x;
        position.current.y += y;
    }
}

fn camera_follow(
    mut camera_position: Query<&mut Position, With<Camera>>,
    player_position: Query<&Position, (With<Spaceship>, Without<Camera>)>,
    time: Res<Time>,
) {
    let mut camera_position = camera_position.single_mut();
    let player_position = player_position.single();
    camera_position.previous = camera_position.current;
    // camera_position.current.y = camera_position.current.y + (player_position.current.y - camera_position.current.y) * 8. * time.delta_seconds();
    // camera_position.current.x = camera_position.current.x + (player_position.current.x - camera_position.current.x) * 8. * time.delta_seconds();
    camera_position.current = camera_position.current + (player_position.current - camera_position.current) * 4. * time.delta_seconds() + Vec2::new(0.0, 5.0);
}

pub fn update_visible_space(
    q_camera: Query<(&Transform, &OrthographicProjection), With<bevy::prelude::Camera>>,
    mut visible_space: Query<&mut VisibleSpace>
) {
    let (camera_transform, camera_projection) = q_camera.single();
    let mut visible_space = visible_space.single_mut();
    let top_left = Vec2::new(
        (camera_transform.translation.x - camera_projection.area.max.x * camera_projection.scale) ,
        (camera_transform.translation.y + camera_projection.area.max.y * camera_projection.scale),
    );
    let bottom_right = Vec2::new(
        (camera_transform.translation.x + camera_projection.area.max.x * camera_projection.scale),
        (camera_transform.translation.y - camera_projection.area.max.y * camera_projection.scale),
    );
    // mutate component only when value is changed
    if visible_space.top_left != top_left {
        visible_space.top_left = top_left;
    }
    if visible_space.bottom_right != bottom_right {
        visible_space.bottom_right = bottom_right;
    }
}