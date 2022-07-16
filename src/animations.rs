use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Animation {
    pub frames: Vec<usize>,
    pub current_frame: usize,
}

#[derive(Component)]
pub struct Animations {
    pub animations: Vec<Animation>,
}
