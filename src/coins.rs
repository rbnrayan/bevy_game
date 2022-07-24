use crate::{
    player::{player_movement, Player},
    SCALE, TILE_COUNT_X, TILE_COUNT_Y, TILE_SIZE,
};
use bevy::prelude::*;

// ================================================================= //
// TODO: Refactor this file and make it generic over some resources. //
// TODO: Make a sort of inventory: Coins, Wood logs, ???             //
// ================================================================= //

pub struct CoinsPlugin;

impl Plugin for CoinsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_coins_amount)
            .add_system(display_coins.after(player_movement));
    }
}

#[derive(Component)]
pub struct Coins;

#[derive(Component)]
pub struct CoinsAmount(pub u32);

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
            transform: Transform::from_xyz(window.width() / 2.1, window.height() / 2.2, 50.0)
                .with_scale(Vec3::splat(SCALE * 0.5)),
            texture: texture_handler,
            ..Default::default()
        })
        .id();
    let coin_amount_text = commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "0",
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
            transform: Transform::from_xyz(-25.0, 9.0, 50.0),
            ..Default::default()
        })
        .insert(CoinsAmount(0))
        .id();
    commands
        .entity(coin_amount_sprite)
        .add_child(coin_amount_text)
        .insert(Coins);
}

pub fn add_coins(
    amount: u32,
    coins_amount_query: &mut Query<(&mut CoinsAmount, &mut Text)>,
) {
    let (mut coins_amount, mut coins_text) = coins_amount_query.single_mut();
    coins_amount.0 += amount;
    *coins_text = Text::with_section(
        coins_amount.0.to_string(),
        (*coins_text).sections[0].style.clone(),
        (*coins_text).alignment,
    );
}

fn display_coins(
    player_query: Query<&Transform, (With<Player>, Without<Coins>)>,
    mut windows: ResMut<Windows>,
    mut coins_query: Query<&mut Transform, (With<Coins>, Without<Player>)>,
) {
    let window = windows.primary_mut();
    let player_transform = player_query.single();

    let mut coins_transform = coins_query.single_mut();

    if player_transform.translation.x < TILE_SIZE * SCALE * (TILE_COUNT_X as f32 / 2.0)
        && player_transform.translation.x > TILE_SIZE * SCALE * -(TILE_COUNT_X as f32 / 2.0)
    {
        coins_transform.translation.x = player_transform.translation.x + window.width() / 2.1;
    }
    if player_transform.translation.y < TILE_SIZE * SCALE * (TILE_COUNT_Y as f32 / 1.5)
        && player_transform.translation.y > TILE_SIZE * SCALE * -(TILE_COUNT_Y as f32 / 1.5)
    {
        coins_transform.translation.y = player_transform.translation.y + window.height() / 2.2;
    }
}
