use bevy::{prelude::*, window::PresentMode};

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const SCALE: f32 = 5.0;
pub const TILE_SIZE: f32 = 32.0;

// Map size: 8 * 5
pub const TILE_COUNT_X: usize = 8;
pub const TILE_COUNT_Y: usize = 5;

mod animations;
mod camera;
mod coins;
mod map;
mod player;
mod sprite_popup;
mod texture_atlas;
mod trees;

use camera::CameraPlugin;
use coins::CoinsPlugin;
use map::MapPlugin;
use player::PlayerPlugin;
use sprite_popup::SpritePopupPlugin;
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
        .add_plugin(CameraPlugin)
        .add_plugin(AtlasPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TreePlugin)
        .add_plugin(SpritePopupPlugin)
        .add_plugin(CoinsPlugin)
        .run();
}
