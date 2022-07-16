use bevy::{prelude::*, window::PresentMode};

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const SCALE: f32 = 5.0;
pub const TILE_SIZE: f32 = 32.0;
// Map size: 8 * 5
pub const TILE_COUNT_X: usize = 8;
pub const TILE_COUNT_Y: usize = 5;

mod animations;
mod map;
mod player;
mod texture_atlas;
mod trees;

use map::MapPlugin;
use player::*;
use texture_atlas::AtlasPlugin;
use trees::TreePlugin;

fn main() {
    let height = 900.0;

    App::new()
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height,
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(AtlasPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TreePlugin)
        .add_startup_system(camera_setup)
        .add_system(camera_follow_player.after(player_movement))
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn camera_follow_player(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();

    if player_transform.translation.x < TILE_SIZE * SCALE * (TILE_COUNT_X as f32 / 2.0)
        && player_transform.translation.x > TILE_SIZE * SCALE * -(TILE_COUNT_X as f32 / 2.0)
    {
        camera_transform.translation.x = player_transform.translation.x;
    }
    if player_transform.translation.y < TILE_SIZE * SCALE * (TILE_COUNT_Y as f32 / 1.5)
        && player_transform.translation.y > TILE_SIZE * SCALE * -(TILE_COUNT_Y as f32 / 1.5)
    {
        camera_transform.translation.y = player_transform.translation.y;
    }
}
