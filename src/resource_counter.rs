use crate::{
    player::{Player, player_movement},
    SCALE,
    map::{TILE_SIZE, TILE_COUNT_X, TILE_COUNT_Y},
};
use bevy::prelude::*;

pub struct ResourceCounterPlugin;

impl Plugin for ResourceCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_resources)
            .add_system(update_resource_pos.after(player_movement))
            .add_system(update_res_count);
    }
}

type SpritePath = String;

// Struct to pass to create a new resource counter (Component, "assets/sprite.png".to_string())
pub struct ResourceToCount<T: Component + Clone>(T, SpritePath);

// Represent the Resource sprite + the text (the actual 'counter')
#[derive(Component)]
struct GameResource;

// Keep track of the count
#[derive(Component)]
pub struct ResourceCounter(pub u32);

// Represent the resources:
#[derive(Component, Clone)]
pub struct CoinResource;
#[derive(Component, Clone)]
pub struct WoodResource;

// spawn all the wanted resources to be counted
fn setup_resources(mut commands: Commands, windows: Res<Windows>, asset_server: Res<AssetServer>) {
    new_resource_counter(
        &mut commands,
        &windows,
        &asset_server,
        ResourceToCount(CoinResource, String::from("coin.png")),
        0.0,
    );
    new_resource_counter(
        &mut commands,
        &windows,
        &asset_server,
        ResourceToCount(WoodResource, String::from("wood_log.png")),
        45.0,
    );
}

fn new_resource_counter<T: Component + Clone>(
    commands: &mut Commands,
    windows: &Res<Windows>,
    asset_server: &Res<AssetServer>,
    resource: ResourceToCount<T>,
    pos_y_offset: f32,
) {
    let window = windows.primary();

    let pos_y = window.height() / 2.15 - pos_y_offset;
    let pos_x = window.width() / 2.1;

    let resource_sprite = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.9),
                ..Default::default()
            },
            texture: asset_server.load(&resource.1),
            transform: Transform::from_xyz(pos_x, pos_y, 50.0).with_scale(Vec3::splat(SCALE * 0.5)),
            ..Default::default()
        })
        .id();
    let resource_text = commands
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
        .insert(ResourceCounter(0))
        .insert(resource.0.clone())
        .id();

    commands.entity(resource_sprite).add_child(resource_text).insert(GameResource);
}

fn update_resource_pos(
    windows: Res<Windows>,
    player_query: Query<&Transform, (With<Player>, Without<ResourceCounter>)>,
    mut resource_query: Query<&mut Transform, (With<GameResource>, Without<Player>)>,
) {
    let mut pos_y_offset = 0.0;

    let window = windows.primary();
    let pos_x = window.width() / 2.1;

    let player_transform = player_query.single();

    for mut resource_transform in resource_query.iter_mut() {
        let pos_y = window.height() / 2.15 - pos_y_offset;

        if player_transform.translation.x < TILE_SIZE * SCALE * (TILE_COUNT_X as f32 / 1.5)
            && player_transform.translation.x > TILE_SIZE * SCALE * -(TILE_COUNT_X as f32 / 1.5)
        {
            resource_transform.translation.x = pos_x + player_transform.translation.x;
        }
        if player_transform.translation.y < TILE_SIZE * SCALE * (TILE_COUNT_Y as f32 / 1.2)
            && player_transform.translation.y > TILE_SIZE * SCALE * -(TILE_COUNT_Y as f32 / 1.2)
        {
            resource_transform.translation.y = pos_y + player_transform.translation.y;
        }

        pos_y_offset += 45.0;
    }
}

fn update_res_count(
    mut resource_counter: Query<(&mut Text, &ResourceCounter)>,
) {
    for (mut resource_text, resource_count) in resource_counter.iter_mut() {
        resource_text.sections[0].value = resource_count.0.to_string();
    }
}
