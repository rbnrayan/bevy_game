use crate::{
    animations::{Animation, AnimationTimer, Animations},
    texture_atlas::AtlasHandle,
    SCALE, TILE_COUNT_X, TILE_COUNT_Y, TILE_SIZE,
};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(animate_sprite);
    }
}

pub enum PlayerState {
    Standing(Direction),
    Moving(Direction),
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
    pub direction: Direction,
    pub speed: f32,
}

pub fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let (mut transform, mut player) = query.single_mut();

    player.state = PlayerState::Standing(player.direction);

    if keys.pressed(KeyCode::Z) {
        if transform.translation.y < TILE_SIZE * SCALE * TILE_COUNT_Y as f32 {
            transform.translation.y += player.speed * time.delta_seconds();
        }
        player.state = PlayerState::Moving(Direction::Up);
    }
    if keys.pressed(KeyCode::S) {
        if transform.translation.y > -(TILE_SIZE * SCALE * TILE_COUNT_Y as f32) {
            transform.translation.y -= player.speed * time.delta_seconds();
        }
        player.state = PlayerState::Moving(Direction::Down);
    }
    if keys.pressed(KeyCode::D) {
        if transform.translation.x < TILE_SIZE * SCALE * TILE_COUNT_X as f32 {
            transform.translation.x += player.speed * time.delta_seconds();
        }
        player.state = PlayerState::Moving(Direction::Right);
        player.direction = Direction::Right;
    }
    if keys.pressed(KeyCode::Q) {
        if transform.translation.x > -(TILE_SIZE * SCALE * TILE_COUNT_X as f32) {
            transform.translation.x -= player.speed * time.delta_seconds();
        }
        player.state = PlayerState::Moving(Direction::Left);
        player.direction = Direction::Left;
    }
}

pub fn spawn_player(mut commands: Commands, texture_atlas_handle: Res<AtlasHandle>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(SCALE))
                .with_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..Default::default()
        })
        .insert(Player {
            state: PlayerState::Standing(Direction::Right),
            direction: Direction::Right,
            speed: 85.0 * SCALE,
        })
        .insert(Animations {
            animations: vec![
                // index 0: running->right
                Animation {
                    frames: vec![1, 2],
                    current_frame: 0,
                    timer: AnimationTimer(Timer::from_seconds(0.2, true)),
                },
                // index 1: running->left
                Animation {
                    frames: vec![4, 5],
                    current_frame: 0,
                    timer: AnimationTimer(Timer::from_seconds(0.2, true)),
                },
                // index 2: running->up/down right
                Animation {
                    frames: vec![6, 7],
                    current_frame: 0,
                    timer: AnimationTimer(Timer::from_seconds(0.2, true)),
                },
                // index 3: running->up/down left
                Animation {
                    frames: vec![8, 9],
                    current_frame: 0,
                    timer: AnimationTimer(Timer::from_seconds(0.2, true)),
                },
            ],
        });
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&Player, &mut TextureAtlasSprite, &mut Animations)>,
) {
    for (player, mut sprite, mut animations) in query.iter_mut() {
        match player.state {
            PlayerState::Moving(Direction::Right) => {
                let animation = &mut animations.animations[0];
                animation.update(&time, &mut sprite);
            }
            PlayerState::Moving(Direction::Left) => {
                let animation = &mut animations.animations[1];
                animation.update(&time, &mut sprite);
            }
            PlayerState::Moving(Direction::Up) | PlayerState::Moving(Direction::Down) => {
                let animation = match player.direction {
                    Direction::Right => &mut animations.animations[2],
                    Direction::Left => &mut animations.animations[3],
                    _ => panic!("Unexpected direction for the player"),
                };
                animation.update(&time, &mut sprite);
            }
            PlayerState::Standing(Direction::Right) => sprite.index = 0,
            PlayerState::Standing(Direction::Left) => sprite.index = 3,
            _ => {}
        }
    }
}