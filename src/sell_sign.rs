use crate::{
    map::TILE_SIZE,
    player::{player_movement, Player},
    resource_counter::{CoinResource, ResourceCounter, WoodResource},
    SCALE,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};

pub struct SellSignPlugin;

impl Plugin for SellSignPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_sell_sign)
            .add_system(trigger_key_hint.after(player_movement))
            .add_system(check_sell_action);
    }
}

#[derive(Component)]
pub struct SellSign;

#[derive(Component)]
pub struct KeyHint;

fn spawn_sell_sign(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("sell_sign.png"),
            transform: Transform::from_scale(Vec3::splat(SCALE * 0.5))
                .with_translation(Vec3::new(0.0, 0.0, 50.0)),
            ..Default::default()
        })
        .insert(SellSign);
}

fn trigger_key_hint(
    asset_server: Res<AssetServer>,
    mut commands: Commands,

    player_query: Query<&Transform, (With<Player>, Without<SellSign>)>,
    sign_query: Query<&Transform, (With<SellSign>, Without<Player>)>,
    key_hint_query: Query<Entity, With<KeyHint>>,
) {
    if check_player_near(&player_query, &sign_query) {
        if key_hint_query.iter().count() < 1 {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1.0, 1.0, 1.0, 0.85),
                        ..Default::default()
                    },
                    texture: asset_server.load("E_key.png"),
                    transform: Transform::from_scale(Vec3::splat(SCALE * 0.5))
                        .with_translation(Vec3::new(0.0, TILE_SIZE * SCALE * 0.5, 50.0)),
                    ..Default::default()
                })
                .insert(KeyHint);
        }
    } else {
        if key_hint_query.iter().count() > 0 {
            let key_hint = key_hint_query.single();
            commands.entity(key_hint).despawn();
        }
    }
}

fn check_player_near(
    player_query: &Query<&Transform, (With<Player>, Without<SellSign>)>,
    sign_query: &Query<&Transform, (With<SellSign>, Without<Player>)>,
) -> bool {
    let player_transform = player_query.single();
    let sign_transform = sign_query.single();

    match collide(
        player_transform.translation,
        Vec2::new(9.0 * SCALE, 12.0 * SCALE), // player size
        sign_transform.translation,
        Vec2::splat(TILE_SIZE * SCALE * 0.8),
    ) {
        Some(_) => true,
        None => false,
    }
}

fn check_sell_action(
    keys: Res<Input<KeyCode>>,
    player_query: Query<&Transform, (With<Player>, Without<SellSign>)>,
    sign_query: Query<&Transform, (With<SellSign>, Without<Player>)>,
    mut coins_res_query: Query<&mut ResourceCounter, (With<CoinResource>, Without<WoodResource>)>,
    mut wood_res_query: Query<&mut ResourceCounter, (With<WoodResource>, Without<CoinResource>)>,
) {
    if check_player_near(&player_query, &sign_query) {
        if keys.just_pressed(KeyCode::E) {
            let mut wood_count = wood_res_query.single_mut();
            let mut coins_count = coins_res_query.single_mut();

            coins_count.0 += wood_count.0 * 3;
            wood_count.0 = 0;
        }
    }
}
