use crate::SCALE;
use bevy::prelude::*;

pub const TILE_SIZE: f32 = 32.0;

// Map size: 8 * 5
pub const TILE_COUNT_X: usize = 12;
pub const TILE_COUNT_Y: usize = 8;

pub struct MapPlugin;

#[derive(Deref, DerefMut)]
pub struct Map(pub Vec<Entity>);

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map(Vec::new()))
            .add_startup_system(spawn_map);
    }
}

pub fn spawn_map(mut commands: Commands, mut map: ResMut<Map>, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("ground.png");

    for y in -(TILE_COUNT_Y as i32)..=TILE_COUNT_Y as i32 {
        for x in -(TILE_COUNT_X as i32)..=TILE_COUNT_X as i32 {
            map.push(
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: texture_handle.clone(),
                        transform: Transform::from_scale(Vec3::splat(SCALE)).with_translation(
                            Vec3::new(
                                TILE_SIZE * (SCALE * x as f32),
                                TILE_SIZE * (SCALE * y as f32),
                                1.0,
                            ),
                        ),
                        ..Default::default()
                    })
                    .id(),
            );
        }
    }

    commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&map[..]);
}
