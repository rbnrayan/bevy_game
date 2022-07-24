use bevy::prelude::*;
use crate::SCALE;

pub struct CoinsPlugin;

impl Plugin for CoinsPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Coins(u32);

fn spawn_coins_amount(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Res<Window>,
    coins_query: Query<&Coins, With<impl Component>>,
) {
    let texture_handler = asset_server.load("coin.png");
    let coin_sprite = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.9),
                ..Default::default()
            },
            transform: Transform::from_xyz(window.width() - 2.0 * SCALE, window.height() - 2.0 * SCALE, 50.0).with_scale(Vec3::splat(SCALE)),
            texutre: texture_handler,
            ..Default::default()
        })
        .id();
    let coin_amount = commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position: Rect {
                    bottom: Val::Px(20.0),
                    right: Val::Px(5.0),
                    ..Default::defaul()
                },
                ..Default::defaul()
            },
            text: Text::with_section(
                "000",
                TextStyle {

                },
                TextAlignment {

                }
            ),
            ..Default::default()
        })
        .id();
    todo!()
}

fn display_coins(
    coins_query: Query<&Coins, With<impl Component>>
) {
    todo!()
}