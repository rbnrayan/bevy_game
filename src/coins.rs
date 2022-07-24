use bevy::prelude::*;

pub struct CoinsPlugin;

impl Plugin for CoinsPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Coins(u32);

fn display_coins(
    coins_query: Query<&Coins, With<impl Component>>
) {
    todo!()
}