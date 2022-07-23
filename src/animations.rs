use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub struct Animation {
    pub frames: Vec<usize>,
    pub current_frame: usize,
    pub timer: AnimationTimer,
}

impl Animation {
    pub fn update(&mut self, time: &Time, sprite: &mut TextureAtlasSprite) {
        self.timer.tick(time.delta());

        if self.timer.just_finished() {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            sprite.index = self.frames[self.current_frame];
        }
    }
}

#[derive(Component)]
pub struct Animations {
    pub animations: Vec<Animation>,
}
