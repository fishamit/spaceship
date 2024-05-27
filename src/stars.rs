use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use bevy::prelude::*;
use crate::camera::VisibleSpace;
use std::collections::hash_map::DefaultHasher;

const STARS_DENSITY: f32 = 20.;
const STARS_AMOUNT: u32 = 1000;

const VISIBLE_SPACE_MARGINS: f32 = STARS_DENSITY * 2.;

pub struct StarsPlugin;


#[derive(Component)]
pub struct Star;

#[derive(Component)]
pub struct StarKey((i32, i32));

#[derive(Component, Debug)]
pub struct VisibleStarField {
    top_left: Vec2,
    bottom_right: Vec2
}

#[derive(Component)]
struct StarmapPosition(Vec2);

#[derive(Component)]
struct StarMap(HashMap<(i32, i32), bool>);

impl Plugin for StarsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_stars)
            .add_systems(Update, (
                update_visible_star_field,
                (handle_star_spawning, handle_star_despawning).run_if(run_if_visible_starfield_changed)
            ));

    }
}

fn setup_stars(mut commands: Commands) {
    commands.spawn(VisibleStarField {
        top_left: Vec2::ZERO,
        bottom_right: Vec2::ZERO,
    });
    commands.spawn(StarMap(HashMap::new()));
}


fn update_visible_star_field(
    visible_space: Query<&VisibleSpace>,
    mut visible_star_field: Query<&mut VisibleStarField>

) {
    let visible_space = visible_space.get_single();
    if visible_space.is_err() {
        return
    }
    let visible_space = visible_space.unwrap();
    let mut visible_star_field = visible_star_field.single_mut();
    let new_top_left =  ((visible_space.top_left / STARS_DENSITY).round() * STARS_DENSITY) + Vec2::new(-VISIBLE_SPACE_MARGINS, VISIBLE_SPACE_MARGINS);
    let new_bottom_right =  (visible_space.bottom_right / STARS_DENSITY).round() * STARS_DENSITY  + Vec2::new(VISIBLE_SPACE_MARGINS, -VISIBLE_SPACE_MARGINS);
    if visible_star_field.top_left != new_top_left {
        visible_star_field.top_left = new_top_left;
    }
    if visible_star_field.bottom_right != new_bottom_right {
        visible_star_field.bottom_right = new_bottom_right;
    }
}

fn run_if_visible_starfield_changed(
    q_visible_starfield: Query<Ref<VisibleStarField>>
) -> bool {
    q_visible_starfield.single().is_changed()
}

fn handle_star_spawning(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_visible_starfield: Query<&VisibleStarField>,
    mut q_starmap: Query<&mut StarMap>
) {
    dbg!("spawning stars");
    let mut starmap = q_starmap.single_mut();
    let VisibleStarField {
        top_left,
        bottom_right
    } = q_visible_starfield.single();
    dbg!(top_left);
    let mut v_index = top_left.clone();
    loop {
        let star_key = (v_index.x as i32, v_index.y as i32);
        if starmap.0.get(&star_key).is_none() {
                starmap.0.insert(star_key, true);
                let (is_star, x_offset, y_offset, scale) = generate_star_properties(star_key, STARS_DENSITY, 2.);
                if !is_star { continue };
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("star.png"),
                        transform: Transform {
                            translation: Vec3::new(v_index.x + x_offset, v_index.y + y_offset, -0.),
                            scale: Vec3::new(scale, scale, 1.),
                            ..default()
                        },
                        ..Default::default()
                    },
                    Star,
                    StarKey(star_key)
                ));
        }
        v_index.x += STARS_DENSITY;
        if v_index.x > bottom_right.x {
            if v_index.y < bottom_right.y { break }
            v_index.x = top_left.x;
            v_index.y -= STARS_DENSITY;
        }
    }
}

fn handle_star_despawning(
    mut commands: Commands,
    q_stars: Query<(Entity, &Transform, &StarKey), With<Star>>,
    q_visible_starfield: Query<&VisibleStarField>,
    mut q_starmap: Query<&mut StarMap>
) {

    let visible_starfield = q_visible_starfield.single();
    let mut starmap = q_starmap.single_mut();
    dbg!(visible_starfield.top_left);
    for (entity, transform, star_key) in q_stars.iter() {
        if transform.translation.x < visible_starfield.top_left.x
        || transform.translation.x > visible_starfield.bottom_right.x
        || transform.translation.y > visible_starfield.top_left.y
        || transform.translation.y < visible_starfield.bottom_right.y {
            commands.entity(entity).despawn();
            starmap.0.remove(&star_key.0);
        }
    }
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn generate_star_properties(key: (i32, i32), max_offset: f32, max_scale: f32) -> (bool, f32, f32, f32) {
    let hash = hash(&key);
    let normalized_value = (hash % 1000000) as f32 / 1000000.0;
    let is_star_threshold = 0.1;
    let is_star = normalized_value < is_star_threshold;
    let x_offset = (hash % 1000000) as f32 / 1000000.0 * max_offset;
    let y_offset = ((hash / 1000000) % 1000000) as f32 / 1000000.0 * max_offset;
    let scale = 1.0 + ((hash / 1000000 / 1000000) % 1000000) as f32 / 1000000.0 * (max_scale - 1.0);
    (is_star, x_offset, y_offset, scale)
}

