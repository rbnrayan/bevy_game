use crate::{
    animations::{Animation, AnimationTimer, Animations},
    map::{spawn_map, Map},
    texture_atlas::AtlasHandle,
    map::{TILE_COUNT_X, TILE_COUNT_Y, TILE_SIZE},
    SCALE, 
};
use bevy::prelude::*;
use rand::{self, Rng};

// TREE_SIZE: Vec2 = Vec2::new(23.0, 32.0);

pub const TREE_AMOUNT: usize = 20;

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_trees.after(spawn_map))
            .add_system(check_tree_amount)
            .add_system(animate_tree);
    }
}

#[derive(Component, Clone, Copy)]
pub struct Tree {
    pub health: i16,
}

#[derive(Component)]
pub struct TreeTimer(pub Timer);

pub fn spawn_trees(
    mut commands: Commands,
    mut map: ResMut<Map>,
    texture_atlas_handle: Res<AtlasHandle>,
    tree_query: Query<&Transform, With<Tree>>,
) {
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
            match spawn_tree(&mut commands, &texture_atlas_handle, &tree_query) {
                Some(e) => e,
                None => continue,
            }
        );
        tree_amount += 1;
    }
    commands.spawn().insert(TreeTimer(Timer::from_seconds(60.0, true)));
}

fn spawn_tree(
    commands: &mut Commands,
    texture_atlas_handle: &Res<AtlasHandle>,
    tree_query: &Query<&Transform, With<Tree>>,
) -> Option<Entity> {
    let texture_sprite = TextureAtlasSprite::new(15);
    let mut rng = rand::thread_rng();

    let (x, y) = (
        rng.gen_range(-(TILE_COUNT_X as i32)..=TILE_COUNT_X as i32),
        rng.gen_range(-(TILE_COUNT_Y as i32)..=TILE_COUNT_Y as i32),
    );

    if check_tree_position(Vec2::new(x as f32, y as f32), &tree_query) {
        return None;
    }

    Some(commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: texture_sprite.clone(),
            texture_atlas: (*texture_atlas_handle).clone(),
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
        .id())
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

fn check_tree_amount(
    time: Res<Time>,
    texture_atlas_handle: Res<AtlasHandle>,
    mut map: ResMut<Map>,
    mut commands: Commands,
    mut timer_query: Query<&mut TreeTimer>,
    tree_query: Query<&Transform, With<Tree>>,
) {
    let mut tree_timer = timer_query.single_mut();

    tree_timer.0.tick(time.delta());

    if tree_timer.0.finished() && tree_query.iter().count() < TREE_AMOUNT {
        if let Some(e) = spawn_tree(&mut commands, &texture_atlas_handle, &tree_query) {
            map.push(e);
        }
    }
}
