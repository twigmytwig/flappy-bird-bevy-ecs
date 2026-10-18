use bevy::prelude::*;

use crate::animation::SpriteAnimation;
use crate::movement::Velocity;
use crate::pipe::{Pipe, ScoredAlready, TopPipe};
use crate::scoring::PointScored;
use crate::state::GameState;

pub struct BirdPlugin;

// Pixels per second, and pixels per second per second.
const FLAP_STRENGTH: f32 = 350.0;
const GRAVITY: f32 = 1000.0;
// 2x the 16px art. Whole-number scales keep pixel art evenly sized.
const BIRD_SIZE: f32 = 32.0;
// Frame shown between flaps. Try 0-3 and pick the best gliding pose.
const BIRD_IDLE_FRAME: usize = 0;

#[derive(Component)]
pub struct Bird;

/// Sent every time the bird flaps. Anything that reacts to a flap
/// (animation now, a sound later) reads this instead of checking input.
#[derive(Message)]
pub struct Flapped;

impl Plugin for BirdPlugin{
    fn build(&self, app: &mut App){
        app.add_message::<Flapped>();
        app.add_systems(OnEnter(GameState::PreGame), spawn_bird);
        app.add_systems(Update, (apply_gravity, check_collision, check_pipe_passed).run_if(in_state(GameState::Playing)));
        // Flap also runs in PreGame so the Space press that starts the round
        // gives the first jump. The state only changes at the end of the frame,
        // so a Playing-only flap would miss that press.
        // chain() runs start_flap_animation after flap, so the animation
        // starts the same frame instead of one frame late.
        app.add_systems(
            Update,
            (flap, start_flap_animation)
                .chain()
                .run_if(in_state(GameState::PreGame).or_else(in_state(GameState::Playing))),
        );
    }
}

fn spawn_bird(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
){
    let image = asset_server.load("Flappy Bird Assets/Player/StyleBird1/Bird1-1.png");
    // The image is a strip of 4 frames, each 16x16: 4 columns, 1 row.
    let layout = layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(16), 4, 1, None, None));

    commands.spawn((
        Bird,
        Velocity(Vec2::ZERO),
        Transform::from_xyz(-100.0, 0.0, 0.0),
        Sprite {
            // Without custom_size, check_collision's let-else skips everything.
            custom_size: Some(Vec2::splat(BIRD_SIZE)),
            ..Sprite::from_atlas_image(image, TextureAtlas { layout, index: BIRD_IDLE_FRAME })
        },
        // Frames 0 to 3, 0.06s each, resting on the idle frame.
        SpriteAnimation::new(0, 3, BIRD_IDLE_FRAME, 0.06),
        // Spawned in PreGame but tagged Playing: survives PreGame -> Playing,
        // removed on Playing -> GameOver.
        DespawnOnExit(GameState::Playing),
    ));
}

// Sets the speed, doesn't move the bird. apply_velocity does the moving.
fn flap(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Bird>>,
    mut flapped: MessageWriter<Flapped>,
){
    if keys.just_pressed(KeyCode::Space){
        for mut velocity in &mut query{
            velocity.0.y = FLAP_STRENGTH;
            flapped.write(Flapped);
        }
    }
}

fn start_flap_animation(
    mut flapped: MessageReader<Flapped>,
    mut query: Query<&mut SpriteAnimation, With<Bird>>,
){
    // count() reads (and so uses up) every message. Several flaps in one
    // frame should still restart the animation only once.
    if flapped.read().count() > 0 {
        for mut animation in &mut query {
            animation.play();
        }
    }
}

fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity, With<Bird>>){
    for mut velocity in &mut query{
        velocity.0.y -= GRAVITY * time.delta_secs();
    }
}

fn check_pipe_passed(
    mut commands: Commands,
    mut message: MessageWriter<PointScored>,
    bird: Single<&Transform, With<Bird>>,
    pipes: Query<(Entity, &Transform), (With<Pipe>, Without<TopPipe>, Without<ScoredAlready>)>,
){
    let bird_transform = *bird;
    for (pipe_entity, pipe_transform) in &pipes {
        if bird_transform.translation.x >= pipe_transform.translation.x {
            message.write(PointScored);
            // Commands apply after this system finishes, so the
            // Without<ScoredAlready> filter skips this pipe from next frame on.
            commands.entity(pipe_entity).insert(ScoredAlready);
        }
    }
}

fn check_collision(
    bird: Single<(&Transform, &Sprite), With<Bird>>,
    pipes: Query<(&Transform, &Sprite), With<Pipe>>,
    mut next: ResMut<NextState<GameState>>,
    window: Single<&Window>
){
    // `*` reaches through the Single wrapper so the tuple can be destructured.
    let (bird_transform, bird_sprite) = *bird;

    // let-else: take the value out of Some, or run the else block and leave.
    let Some(bird_size) = bird_sprite.custom_size else { return; };

    // World (0, 0) is the screen center, so each edge is half the height away.
    let screen_top = window.height() / 2.0;
    let screen_bottom = -window.height() / 2.0;
    let bird_top = bird_transform.translation.y + bird_size.y / 2.0;
    let bird_bottom = bird_transform.translation.y - bird_size.y / 2.0;

    if bird_bottom <= screen_bottom || bird_top >= screen_top {
        next.set(GameState::GameOver);
        return;
    }

    let bird_center = bird_transform.translation.truncate();

    for (pipe_transform, pipe_sprite) in &pipes {
        let Some(pipe_size) = pipe_sprite.custom_size else { continue; };
        let pipe_center = pipe_transform.translation.truncate();

        if rects_overlap(bird_center, bird_size, pipe_center, pipe_size) {
            next.set(GameState::GameOver);
        }
    }
}

//TODO: move this out of bird, it is a general function that could be reused eventually
// AABB: two unrotated rectangles touch only if they overlap on x AND on y.
// Positions are centers; sizes are full width/height.
fn rects_overlap(a_center: Vec2, a_size: Vec2, b_center: Vec2, b_size: Vec2) -> bool {
    let a_left = a_center.x - a_size.x / 2.0;
    let a_right = a_center.x + a_size.x / 2.0;
    let a_bottom = a_center.y - a_size.y / 2.0;
    let a_top = a_center.y + a_size.y / 2.0;

    let b_left = b_center.x - b_size.x / 2.0;
    let b_right = b_center.x + b_size.x / 2.0;
    let b_bottom = b_center.y - b_size.y / 2.0;
    let b_top = b_center.y + b_size.y / 2.0;

    let overlap_x = a_left < b_right && a_right > b_left;
    let overlap_y = a_bottom < b_top && a_top > b_bottom;

    overlap_x && overlap_y
}
