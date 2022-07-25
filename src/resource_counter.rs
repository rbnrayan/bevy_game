use bevy::prelude::*;

pub struct ResourceCounterPlugin;

impl Plugin for ResourceCounterPlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(ResourcesToCount(vec![
        //     (Box::new(CoinResource), String::from("coin.png")),
        //     (Box::new(WoodLogResource), String::from("wood_log.png")),
        // ]))
        // .add_startup_system(spawn_resource_counter);
        app
            .insert_resource(ResourceToCount(CoinResource, String::from("coin.png")))
            .insert_resource(ResourceToCount(WoodLogResource, String::from("wood_log.png")))
            .add_startup_system(spawn_resource_counter::<CoinResource>)
            .add_startup_system(spawn_resource_counter::<WoodLogResource>);
    }
}

type SpritePath = String;

pub struct ResourceToCount<T: Component + Clone>(T, SpritePath);

#[derive(Component)]
pub struct ResourceCounter(pub u32);

#[derive(Component, Clone)]
pub struct CoinResource;

#[derive(Component, Clone)]
pub struct WoodLogResource;

fn spawn_resource_counter<T: Component + Clone>(
    mut commands: Commands,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
    resource: Res<ResourceToCount<T>>,
) {
    let window = windows.primary();

    let pos_y = window.height() / 2.1;
    let pos_x = window.width() / 2.2;

    let resource_sprite = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.9),
                ..Default::default()
            },
            texture: asset_server.load(&resource.1),
            transform: Transform::from_xyz(pos_x, pos_y, 50.0),
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
        .id();

    commands
        .entity(resource_sprite)
        .add_child(resource_text)
        .insert(resource.0.clone());
}
