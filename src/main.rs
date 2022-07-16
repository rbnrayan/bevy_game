use bevy::{prelude::*, window::PresentMode};

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const SCALE: f32 = 5.0;
pub const TILE_SIZE: f32 = 32.0;
pub const TILE_COUNT_X: isize = 8;

mod animations;
mod map;
mod player;
mod texture_atlas;

use map::MapPlugin;
use player::*;
use texture_atlas::AtlasPlugin;

fn main() {
    let height = 900.0;

    App::new()
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height,
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AtlasPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
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

    if player_transform.translation.x < TILE_SIZE * SCALE * (TILE_COUNT_X / 2) as f32
        && player_transform.translation.x > TILE_SIZE * SCALE * -(TILE_COUNT_X / 2) as f32 {
        camera_transform.translation.x = player_transform.translation.x;
    }
}
