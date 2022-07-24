use crate::{SCALE, TILE_SIZE};
use bevy::prelude::*;

pub struct SpritePopupPlugin;

impl Plugin for SpritePopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_sprite_popup);
    }
}

#[derive(Component)]
struct SpritePopup(pub Timer);

pub fn trigger_sprite_popup(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    pos: Vec3,
    sprite_path: &str,
) {
    let texture_handle = asset_server.load(sprite_path);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.9),
                ..Default::default()
            },
            texture: texture_handle,
            transform: Transform::from_scale(Vec3::splat(SCALE / 2.0))
                .with_translation(pos + Vec3::new(0.0, 2.0 * TILE_SIZE, 0.0)),
            ..Default::default()
        })
        .insert(SpritePopup(Timer::from_seconds(0.5, true)));
}

fn update_sprite_popup(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut SpritePopup), With<SpritePopup>>,
) {
    for (entity_id, mut transform, mut sprite_popup) in query.iter_mut() {
        sprite_popup.0.tick(time.delta());
        if sprite_popup.0.finished() {
            commands.entity(entity_id).despawn();
        }
        transform.translation.y += 1.5 * TILE_SIZE * time.delta_seconds();
        transform.scale -= Vec3::new(0.02, 0.02, 0.0);
    }
}
