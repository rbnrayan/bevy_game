use crate::{
    player::{player_movement, Player},
    map::{TILE_COUNT_X, TILE_COUNT_Y, TILE_SIZE},
    SCALE,
};
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_setup)
            .add_system(camera_follow_player.after(player_movement));
    }
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

    if player_transform.translation.x < TILE_SIZE * SCALE * (TILE_COUNT_X as f32 / 1.5)
        && player_transform.translation.x > TILE_SIZE * SCALE * -(TILE_COUNT_X as f32 / 1.5)
    {
        camera_transform.translation.x = player_transform.translation.x;
    }
    if player_transform.translation.y < TILE_SIZE * SCALE * (TILE_COUNT_Y as f32 / 1.2)
        && player_transform.translation.y > TILE_SIZE * SCALE * -(TILE_COUNT_Y as f32 / 1.2)
    {
        camera_transform.translation.y = player_transform.translation.y;
    }
}
