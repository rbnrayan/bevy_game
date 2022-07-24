use crate::{SCALE, TILE_SIZE, TILE_COUNT_Y, TILE_COUNT_X, player::{Player, player_movement}};
use bevy::prelude::*;

pub struct CoinsPlugin;

impl Plugin for CoinsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_coins_amount)
            .add_system(display_coins.after(player_movement));
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Coins(u32);

#[derive(Component)]
struct CoinsAmount;

pub fn spawn_coins_amount(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    let texture_handler = asset_server.load("coin.png");
    let window = windows.primary_mut();

    let coin_amount_sprite = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.9),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                window.width() / 2.1, window.height() / 2.2, 50.0
            )
            .with_scale(Vec3::splat(SCALE * 0.5)),
            texture: texture_handler,
            ..Default::default()
        })
        .id();
    let coin_amount_text = commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "000",
                TextStyle {
                    font: asset_server.load("fonts/Fixedsys Excelsior 3.01 Regular.ttf"),
                    font_size: 15.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            transform: Transform::from_xyz(
                -25.0,
                8.0,
                50.0,
            ),
            ..Default::default()
        })
        .id();
    commands
        .entity(coin_amount_sprite)
        .add_child(coin_amount_text)
        .insert(Coins(0));
}

fn display_coins(
    player_query: Query<&Transform, (With<Player>, Without<Coins>)>,
    mut windows: ResMut<Windows>,
    mut coins_query: Query<&mut Transform, (With<Coins>, Without<Player>)>
) {
    let window = windows.primary_mut();
    let player_transform = player_query.single();

    for mut coins_tranform in coins_query.iter_mut() {
        if player_transform.translation.x < TILE_SIZE * SCALE * (TILE_COUNT_X as f32 / 2.0)
            && player_transform.translation.x > TILE_SIZE * SCALE * -(TILE_COUNT_X as f32 / 2.0)
        {
            coins_tranform.translation.x = player_transform.translation.x + window.width() / 2.1;
        }
        if player_transform.translation.y < TILE_SIZE * SCALE * (TILE_COUNT_Y as f32 / 1.5)
            && player_transform.translation.y > TILE_SIZE * SCALE * -(TILE_COUNT_Y as f32 / 1.5)
        {
            coins_tranform.translation.y = player_transform.translation.y + window.height() / 2.2;
        }
    }
}
