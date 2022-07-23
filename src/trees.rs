use crate::{
    animations::{Animation, AnimationTimer, Animations},
    map::{spawn_map, Map},
    texture_atlas::AtlasHandle,
    SCALE, TILE_COUNT_X, TILE_COUNT_Y, TILE_SIZE,
};
use bevy::prelude::*;
use rand::{self, Rng};

// TREE_SIZE: Vec2 = Vec2::new(23.0, 32.0);

pub const TREE_AMOUNT: usize = 20;

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tree.after(spawn_map))
            .add_system(animate_tree);
    }
}

#[derive(Component, Clone, Copy)]
pub struct Tree {
    pub health: i16,
}

pub fn spawn_tree(
    mut commands: Commands,
    mut map: ResMut<Map>,
    texture_atlas_handle: Res<AtlasHandle>,
    tree_query: Query<&Transform, With<Tree>>,
) {
    let texture_sprite = TextureAtlasSprite::new(15);
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
                .spawn_bundle(SpriteSheetBundle {
                    sprite: texture_sprite.clone(),
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_scale(Vec3::splat(SCALE)).with_translation(
                        Vec3::new(
                            x as f32 * SCALE * TILE_SIZE,
                            y as f32 * SCALE * TILE_SIZE,
                            20.0,
                        ),
                    ),
                    ..Default::default()
                })
                .insert(Tree { health: 100 })
                .insert(Animations {
                    animations: vec![Animation {
                        frames: vec![15, 16, 17, 18, 19],
                        current_frame: 0,
                        timer: AnimationTimer(Timer::from_seconds(0.5, true)),
                    }],
                })
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

fn animate_tree(
    time: Res<Time>,
    mut tree_query: Query<(&mut TextureAtlasSprite, &mut Animations), With<Tree>>,
) {
    for (mut sprite, mut animations) in tree_query.iter_mut() {
        animations.animations[0].update(&time, &mut sprite);
    }
}
