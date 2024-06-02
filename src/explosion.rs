use bevy::prelude::*;
use crate::physics::Position;

pub struct ExplosionsPlugin;

impl Plugin for ExplosionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ExplosionEvent>()
            .add_systems(Update, (spawn_explosions, handle_explosions));
    }
}


fn spawn_explosions(
    mut commands: Commands,
    mut e_explosions: EventReader<ExplosionEvent>,
    mut asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {


    for event in e_explosions.read() {
        let texture = asset_server.load("explosion.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::splat(32.0), 15, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        // Use only the subset of sprites in the sheet that make up the run animation
        let animation_indices = AnimationIndices { first: 2, last: 14 };
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(event.0.current.x, event.0.current.y, 0.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                texture,
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.06, TimerMode::Repeating)),
            Explosion
        ));
    }
}

fn handle_explosions(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TextureAtlas, &AnimationIndices, &mut AnimationTimer), With<Explosion>>,
    time: Res<Time>
) {
    for (entity, mut atlas, indices, mut timer) in query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            if atlas.index < indices.last {
                atlas.index += 1;
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}


#[derive(Bundle)]
pub struct ExplosionBundle {
    sprite_sheet_bundle: SpriteSheetBundle,
    marker: Explosion,
    timer: AnimationTimer
}

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Event)]
pub struct ExplosionEvent(pub Position);