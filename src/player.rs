use crate::{
    animations::{Animation, AnimationTimer, Animations},
    texture_atlas::AtlasHandle,
    trees::Tree,
    SCALE, TILE_COUNT_X, TILE_COUNT_Y, TILE_SIZE,
};
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

// PLAYER_SIZE: Vec2 = Vec2::new(9.0, 12.0);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(player_action)
            .add_system(animate_sprite.after(player_movement));
    }
}

pub enum ActionState {
    Perform,
    Recover,
    Ready,
}

#[derive(Component)]
pub struct PlayerAction {
    pub state: ActionState,
    pub action_timer: Timer,
    pub recover_timer: Timer,
}

pub enum PlayerState {
    Stand(Direction),
    Move(Direction),
    Chop(Direction),
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
    pub dmg: i16,
}

pub fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    tree_query: Query<&Transform, (With<Tree>, Without<Player>)>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let (mut transform, mut player) = query.single_mut();

    match player.state {
        PlayerState::Chop(_) => {}
        _ => player.state = PlayerState::Stand(player.direction),
    }

    let mut y_delta = 0.0;
    if keys.pressed(KeyCode::Z) {
        y_delta += player.speed * time.delta_seconds();
        player.state = PlayerState::Move(Direction::Up);
    }
    if keys.pressed(KeyCode::S) {
        y_delta -= player.speed * time.delta_seconds();
        player.state = PlayerState::Move(Direction::Down);
    }

    let mut x_delta = 0.0;
    if keys.pressed(KeyCode::D) {
        x_delta += player.speed * time.delta_seconds();
        player.state = PlayerState::Move(Direction::Right);
        player.direction = Direction::Right;
    }
    if keys.pressed(KeyCode::Q) {
        x_delta -= player.speed * time.delta_seconds();
        player.state = PlayerState::Move(Direction::Left);
        player.direction = Direction::Left;
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);

    if target.y < TILE_SIZE * SCALE * TILE_COUNT_Y as f32
        && target.y > -(TILE_SIZE * SCALE * TILE_COUNT_Y as f32)
        && !tree_collision(target, &tree_query)
    {
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);

    if target.x < TILE_SIZE * SCALE * TILE_COUNT_X as f32
        && target.x > -(TILE_SIZE * SCALE * TILE_COUNT_X as f32)
        && !tree_collision(target, &tree_query)
    {
        transform.translation = target;
    }
}

fn player_action(
    time: Res<Time>,
    mouse_btn: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut PlayerAction, &Transform)>,

    mut tree_query: Query<(Entity, &mut Tree, &Transform)>,
) {
    let (mut player, mut action, player_transform) = player_query.single_mut();

    match action.state {
        ActionState::Perform => {
            action.action_timer.tick(time.delta());
            if action.action_timer.finished() {
                action.state = ActionState::Recover;
                action.action_timer.reset();

                player.state = PlayerState::Stand(player.direction);
            }
        }
        ActionState::Recover => {
            action.recover_timer.tick(time.delta());
            if action.recover_timer.finished() {
                action.state = ActionState::Ready;
                action.recover_timer.reset();
            }
        }
        ActionState::Ready => {
            if mouse_btn.just_pressed(MouseButton::Left) {
                player.state = PlayerState::Chop(player.direction);

                action.state = ActionState::Perform;

                for (tree_entity, mut tree_struct, tree_transform) in tree_query.iter_mut() {
                    if player_can_chop_tree(player_transform.translation, tree_transform.translation) {
                        tree_struct.health -= player.dmg;
                        if tree_struct.health <= 0 {
                            commands.entity(tree_entity).despawn();
                        }
                    }
                }
            }
        }
    }
}

fn player_can_chop_tree(
    player_pos: Vec3,
    tree_pos: Vec3,
) -> bool {
    let collide = collide(
        player_pos,
        Vec2::new(9.0 * SCALE, 12.0 * SCALE), // player size
        tree_pos,
        Vec2::splat(32.0 * SCALE), // full tree sprite size
    );
    match collide {
        Some(Collision::Right) | Some(Collision::Left) | Some(Collision::Inside) => true,
        _ => false,
    }
}

pub fn tree_collision(
    target_player_pos: Vec3,
    tree_query: &Query<&Transform, (With<Tree>, Without<Player>)>,
) -> bool {
    for tree_transform in tree_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::new(9.0 * SCALE, 12.0 * SCALE), // character real size: 9x12
            tree_transform.translation + Vec3::new(0.0, -11.0 * SCALE, 0.0), // collide only with
            // the tree root
            Vec2::new(12.0 * SCALE, 5.0 * SCALE), // adjust the tree size to match only the root
        );
        if collision.is_some() {
            return true;
        }
    }
    false
}

fn spawn_player(mut commands: Commands, texture_atlas_handle: Res<AtlasHandle>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(SCALE))
                .with_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..Default::default()
        })
        .insert(Player {
            state: PlayerState::Stand(Direction::Right),
            direction: Direction::Right,
            speed: 80.0 * SCALE,
            dmg: 40,
        })
        .insert(PlayerAction {
            state: ActionState::Ready,
            action_timer: Timer::from_seconds(0.2, false),
            recover_timer: Timer::from_seconds(0.2, false),
        })
        .insert(Animations {
            animations: vec![
                // index 0: running->right
                Animation {
                    frames: vec![2, 3],
                    current_frame: 0,
                    timer: AnimationTimer(Timer::from_seconds(0.2, true)),
                },
                // index 1: running->left
                Animation {
                    frames: vec![7, 8],
                    current_frame: 0,
                    timer: AnimationTimer(Timer::from_seconds(0.2, true)),
                },
                // index 2: running->up/down right
                Animation {
                    frames: vec![10, 11],
                    current_frame: 0,
                    timer: AnimationTimer(Timer::from_seconds(0.2, true)),
                },
                // index 3: running->up/down left
                Animation {
                    frames: vec![12, 13],
                    current_frame: 0,
                    timer: AnimationTimer(Timer::from_seconds(0.2, true)),
                },
            ],
        });
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&Player, &mut TextureAtlasSprite, &mut Animations)>,
) {
    for (player, mut sprite, mut animations) in query.iter_mut() {
        match player.state {
            PlayerState::Move(Direction::Right) => {
                let animation = &mut animations.animations[0];
                animation.update(&time, &mut sprite);
            }
            PlayerState::Move(Direction::Left) => {
                let animation = &mut animations.animations[1];
                animation.update(&time, &mut sprite);
            }
            PlayerState::Move(Direction::Up) | PlayerState::Move(Direction::Down) => {
                let animation = match player.direction {
                    Direction::Right => &mut animations.animations[2],
                    Direction::Left => &mut animations.animations[3],
                    _ => panic!("Player: Unexpected direction"),
                };
                animation.update(&time, &mut sprite);
            }
            PlayerState::Chop(Direction::Right) => sprite.index = 1,
            PlayerState::Chop(Direction::Left) => sprite.index = 6,
            PlayerState::Stand(Direction::Right) => sprite.index = 0,
            PlayerState::Stand(Direction::Left) => sprite.index = 5,
            _ => {}
        }
    }
}
