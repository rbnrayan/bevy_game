use crate::{
    animations::{Animation, AnimationTimer, Animations},
    sprite_popup::trigger_sprite_popup,
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

// Movements related components/structs:
#[derive(Component)]
pub enum PlayerState {
    Stand(Direction),
    Move(Direction),
    Chop(Direction),
}
#[derive(Component, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}
#[derive(Component)]
pub struct Speed(pub f32);

// Define an action and it's behavior
#[derive(Component)]
struct PlayerAction {
    pub state: ActionState,
    pub action_timer: Timer,
    pub recover_timer: Timer,
}
enum ActionState {
    Perform,
    Recover,
    Ready,
}
// Represent how much damage the player inflicts to a tree
#[derive(Component)]
pub struct Strength(u32);

// The Player itself
#[derive(Component)]
pub struct Player;

// - check which direction key (Z,Q,S,D) is pressed, then compute how much to move on the x and y axis
// - change the player state according to the direction
// - check if the player encounter a wall/tree and move him according to collisions
pub fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    tree_query: Query<&Transform, (With<Tree>, Without<Player>)>,
    mut player_query: Query<
        (&mut Transform, &mut PlayerState, &mut Direction, &Speed),
        With<Player>,
    >,
) {
    let (mut player_transform, mut player_state, mut player_direction, player_speed) =
        player_query.single_mut();

    match *player_state {
        PlayerState::Chop(_) => {}
        _ => *player_state = PlayerState::Stand(*player_direction),
    }

    let mut y_delta = 0.0;
    if keys.pressed(KeyCode::Z) {
        y_delta += player_speed.0 * time.delta_seconds();
        *player_state = PlayerState::Move(Direction::Up);
    }
    if keys.pressed(KeyCode::S) {
        y_delta -= player_speed.0 * time.delta_seconds();
        *player_state = PlayerState::Move(Direction::Down);
    }

    let mut x_delta = 0.0;
    if keys.pressed(KeyCode::D) {
        x_delta += player_speed.0 * time.delta_seconds();
        *player_state = PlayerState::Move(Direction::Right);
        *player_direction = Direction::Right;
    }
    if keys.pressed(KeyCode::Q) {
        x_delta -= player_speed.0 * time.delta_seconds();
        *player_state = PlayerState::Move(Direction::Left);
        *player_direction = Direction::Left;
    }

    let target = player_transform.translation + Vec3::new(0.0, y_delta, 0.0);

    if target.y < TILE_SIZE * SCALE * TILE_COUNT_Y as f32
        && target.y > -(TILE_SIZE * SCALE * TILE_COUNT_Y as f32)
        && !tree_collision(target, &tree_query)
    {
        player_transform.translation = target;
    }

    let target = player_transform.translation + Vec3::new(x_delta, 0.0, 0.0);

    if target.x < TILE_SIZE * SCALE * TILE_COUNT_X as f32
        && target.x > -(TILE_SIZE * SCALE * TILE_COUNT_X as f32)
        && !tree_collision(target, &tree_query)
    {
        player_transform.translation = target;
    }
}

// match on the player action state (Ready, Perform, Recover)
// if the state is 'Ready', check if the player collide with a tree (player_can_chop_tree)
// if so, trigger a sprite to pop above the player and perform the action (damage the tree)
fn player_action(
    asset_server: Res<AssetServer>,

    time: Res<Time>,
    mouse_btn: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut player_query: Query<(
        &mut PlayerAction,
        &mut PlayerState,
        &Direction,
        &Transform,
        &Strength,
    )>,

    mut tree_query: Query<(Entity, &mut Tree, &Transform)>,
) {
    let (mut action, mut player_state, player_direction, player_transform, player_strength) =
        player_query.single_mut();

    match action.state {
        ActionState::Perform => {
            action.action_timer.tick(time.delta());
            if action.action_timer.finished() {
                action.state = ActionState::Recover;
                action.action_timer.reset();

                *player_state = PlayerState::Stand(*player_direction);
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
                *player_state = PlayerState::Chop(*player_direction);

                action.state = ActionState::Perform;

                // check each tree if it can be chopped
                for (tree_entity, mut tree_struct, tree_transform) in tree_query.iter_mut() {
                    if player_can_chop_tree(
                        player_transform.translation,
                        tree_transform.translation,
                    ) {
                        trigger_sprite_popup(
                            &mut commands,
                            &asset_server,
                            player_transform.translation + Vec3::new(0.0, 1.8 * TILE_SIZE, 0.0),
                            "wood log.png",
                        );
                        // chop the tree, inflict damage to the target tree
                        // (move to a fn?)
                        tree_struct.health -= player_strength.0 as i16;
                        if tree_struct.health <= 0 {
                            commands.entity(tree_entity).despawn();
                        }
                    }
                }
            }
        }
    }
}

// check if the player position collide with a tree
// return true if so, and only if it collide on the Right|Left or Inside
fn player_can_chop_tree(player_pos: Vec3, tree_pos: Vec3) -> bool {
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

// check for a collision between the player position and a tree
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

// spawn the player with the texture atlas for animations, scale him, and increase the z axis
// insert all the components needed and the animations
fn spawn_player(mut commands: Commands, texture_atlas_handle: Res<AtlasHandle>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(SCALE))
                .with_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..Default::default()
        })
        .insert(Player)
        .insert(Strength(40))
        .insert(Speed(3.0 * TILE_SIZE * SCALE))
        .insert(PlayerState::Stand(Direction::Right))
        .insert(Direction::Right)
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

// Animate the player sprite according to the state and direction
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<
        (
            &PlayerState,
            &Direction,
            &mut TextureAtlasSprite,
            &mut Animations,
        ),
        With<Player>,
    >,
) {
    for (player_state, player_direction, mut sprite, mut animations) in query.iter_mut() {
        match *player_state {
            PlayerState::Move(Direction::Right) => {
                let animation = &mut animations.animations[0];
                animation.update(&time, &mut sprite);
            }
            PlayerState::Move(Direction::Left) => {
                let animation = &mut animations.animations[1];
                animation.update(&time, &mut sprite);
            }
            PlayerState::Move(Direction::Up) | PlayerState::Move(Direction::Down) => {
                let animation = match player_direction {
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
