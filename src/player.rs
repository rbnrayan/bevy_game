use crate::{
    animations::{Animation, AnimationTimer, Animations},
    texture_atlas::AtlasHandle,
    SCALE,
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
    Standing(PlayerDirection),
    Moving(PlayerDirection),
}

#[derive(Clone, Copy)]
pub enum PlayerDirection {
    Right,
    Left,
}

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
    pub direction: PlayerDirection,
    pub speed: f32,
}

pub fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let (mut transform, mut player) = query.single_mut();

    player.state = PlayerState::Standing(player.direction);

    if keys.pressed(KeyCode::D) {
        transform.translation.x += player.speed * time.delta_seconds();
        player.state = PlayerState::Moving(PlayerDirection::Right);
        player.direction = PlayerDirection::Right;
    }
    if keys.pressed(KeyCode::Q) {
        transform.translation.x -= player.speed * time.delta_seconds();
        player.state = PlayerState::Moving(PlayerDirection::Left);
        player.direction = PlayerDirection::Left;
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
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)))
        .insert(Player {
            state: PlayerState::Standing(PlayerDirection::Right),
            direction: PlayerDirection::Right,
            speed: 85.0 * SCALE,
        })
        .insert(Animations {
            animations: vec![
                // index 0: running->right
                Animation {
                    frames: vec![1, 2],
                    current_frame: 0,
                },
                // index 1: running->left
                Animation {
                    frames: vec![4, 5],
                    current_frame: 0,
                },
            ],
        });
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &Player,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Animations,
    )>,
) {
    for (player, mut timer, mut sprite, mut animations) in query.iter_mut() {
        match player.state {
            PlayerState::Moving(PlayerDirection::Right) => {
                let mut animation = &mut animations.animations[0];

                timer.tick(time.delta());

                if timer.just_finished() {
                    animation.current_frame =
                        (animation.current_frame + 1) % animation.frames.len();
                    sprite.index = animation.frames[animation.current_frame];
                }
            }
            PlayerState::Moving(PlayerDirection::Left) => {
                let mut animation = &mut animations.animations[1];

                timer.tick(time.delta());

                if timer.just_finished() {
                    animation.current_frame =
                        (animation.current_frame + 1) % animation.frames.len();
                    sprite.index = animation.frames[animation.current_frame];
                }
            }
            PlayerState::Standing(PlayerDirection::Right) => sprite.index = 0,
            PlayerState::Standing(PlayerDirection::Left) => sprite.index = 3,
        }
    }
}
