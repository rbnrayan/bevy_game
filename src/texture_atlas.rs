use bevy::prelude::*;

pub struct AtlasPlugin;

impl Plugin for AtlasPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup);
    }
}

#[derive(Deref)]
pub struct AtlasHandle(pub Handle<TextureAtlas>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprite_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::splat(32.0), 5, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(AtlasHandle(texture_atlas_handle));
}
