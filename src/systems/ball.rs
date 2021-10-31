use bevy::{math::Vec3, prelude::*};

use crate::WINDOW_HEIGHT;

use super::Scoreboard;

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
        mut ball_query: Query<(&mut Ball, &mut Transform)>,
        mut scoreboard: ResMut<Scoreboard>,
        keyboard_input: Res<Input<KeyCode>>,
    ) {
        let delta_seconds = f32::min(0.2, time.delta_seconds());

        if let Ok((mut ball, mut transform)) = ball_query.single_mut() {
            if scoreboard.lives > 0 {
                if scoreboard.score < scoreboard.maxScores {
                    transform.translation += ball.velocity * delta_seconds;
                }

                if transform.translation.y.abs() >= WINDOW_HEIGHT / 2.0 {
                    scoreboard.lives -= 1;
                    transform.translation = Vec3::new(0.0, -50.0, 1.0);
                    let mut x_velocity = 0.5;
                    if scoreboard.lives % 2 == 0 {
                        x_velocity = -0.5;
                    }
                    ball.velocity = BALL_VELOCITY * Vec3::new(x_velocity, -0.5, 0.0).normalize();
                }
            }

            if keyboard_input.just_pressed(KeyCode::R) {
                transform.translation = Vec3::new(0.0, -50.0, 1.0);
                let mut x_velocity = 0.5;
                if scoreboard.lives % 2 == 0 {
                    x_velocity = -0.5;
                }
                ball.velocity = BALL_VELOCITY * Vec3::new(x_velocity, -0.5, 0.0).normalize();
            }
        }
    }
}