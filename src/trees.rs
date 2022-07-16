use crate::{
    map::{spawn_map, Map},
    SCALE, TILE_COUNT_X, TILE_COUNT_Y, TILE_SIZE,
};
use bevy::prelude::*;
use rand::{self, Rng};

// TREE_SIZE: Vec2 = Vec2::new(23.0, 32.0);

pub const TREE_AMOUNT: usize = 10;

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tree.after(spawn_map));
    }
}

#[derive(Component)]
pub struct Tree;

pub fn spawn_tree(
    mut commands: Commands,
    mut map: ResMut<Map>,
    asset_server: Res<AssetServer>,
    tree_query: Query<&Transform, With<Tree>>,
) {
    let texture_handle = asset_server.load("tree.png");
    let mut tree_amount = tree_query.iter().count();
    let mut rng = rand::thread_rng();

    loop {
        if tree_amount >= TREE_AMOUNT {
            break;
        }

        let (x, y) = (
            rng.gen_range(-(TILE_COUNT_X as i32)..=TILE_COUNT_X as i32),
            rng.gen_range(-(TILE_COUNT_Y as i32)..=TILE_COUNT_Y as i32),
        );

        if check_tree_position(Vec2::new(x as f32, y as f32), &tree_query) {
            continue;
        }

        map.push(
            commands
                .spawn_bundle(SpriteBundle {
                    texture: texture_handle.clone(),
                    transform: Transform::from_scale(Vec3::splat(SCALE)).with_translation(
                        Vec3::new(
                            x as f32 * SCALE * TILE_SIZE,
                            y as f32 * SCALE * TILE_SIZE,
                            20.0,
                        ),
                    ),
                    ..Default::default()
                })
                .insert(Tree)
                .id(),
        );
        tree_amount += 1;
    }
}

pub fn check_tree_position(pos: Vec2, tree_query: &Query<&Transform, With<Tree>>) -> bool {
    for transform in tree_query.iter() {
        if transform.translation.x == pos.x && transform.translation.y == pos.y {
            return true;
        }
    }
    false
}
