use bevy::{math::Vec3, prelude::*};

pub const BALL_SIZE: f32 = 30.0;
pub const BALL_VELOCITY: f32 = 400.0;

pub struct Ball {
    pub velocity: Vec3,
}

pub struct BallSystem;

impl BallSystem {
    pub fn setup(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands
        .spawn_bundle(
            SpriteBundle {
                material: materials.add(Color::hex("FF5A5F").unwrap().into()),
                transform: Transform::from_xyz(0.0, -50.0, 1.0),
                sprite: Sprite::new(Vec2::new(BALL_SIZE, BALL_SIZE)),
                ..Default::default()
            }
        )
        .insert(Ball {
            velocity: BALL_VELOCITY * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
    }

    pub fn run(
        time: Res<Time>,
        mut ball_query: Query<(&Ball, &mut Transform)>
    ) {
        let delta_seconds = f32::min(0.2, time.delta_seconds());

        if let Ok((ball, mut transform)) = ball_query.single_mut() {
            transform.translation += ball.velocity * delta_seconds;
        }
    }
}