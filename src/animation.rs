use bevy::prelude::*;

pub struct AnimationPlugin;

/// Plays a sprite's atlas frames `first..=last` once, then rests on `idle`.
/// Call `play()` to start or restart it. Knows nothing about what it animates;
/// the owner decides when to play.
#[derive(Component)]
pub struct SpriteAnimation {
    first: usize,
    last: usize,
    idle: usize,
    frame: usize,
    timer: Timer,
    playing: bool,
}

impl SpriteAnimation {
    pub fn new(first: usize, last: usize, idle: usize, seconds_per_frame: f32) -> Self {
        Self {
            first,
            last,
            idle,
            frame: idle,
            timer: Timer::from_seconds(seconds_per_frame, TimerMode::Repeating),
            playing: false,
        }
    }

    pub fn play(&mut self) {
        self.frame = self.first;
        self.timer.reset();
        self.playing = true;
    }
}

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprites);
    }
}

fn animate_sprites(time: Res<Time>, mut query: Query<(&mut SpriteAnimation, &mut Sprite)>) {
    for (mut animation, mut sprite) in &mut query {
        if animation.playing && animation.timer.tick(time.delta()).just_finished() {
            if animation.frame < animation.last {
                animation.frame += 1;
            } else {
                animation.playing = false;
                animation.frame = animation.idle;
            }
        }

        // Only write when it differs: writing to Sprite marks it changed,
        // which would make Bevy redo work for it every frame.
        if let Some(atlas) = &mut sprite.texture_atlas
            && atlas.index != animation.frame
        {
            atlas.index = animation.frame;
        }
    }
}
