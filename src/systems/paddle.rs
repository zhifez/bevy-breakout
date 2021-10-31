use bevy::{core::Time, input::Input, prelude::*};

use crate::{Collider, WINDOW_HEIGHT, WINDOW_WIDTH};

use super::collision::WALL_SIZE;

pub const PADDLE_WIDTH: f32 = 150.0;
pub const PADDLE_HEIGHT: f32 = 20.0;

pub struct Paddle {
    pub speed: f32,
}

pub struct PaddleSystem;

impl PaddleSystem {
    pub fn setup(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands
        .spawn_bundle(
            SpriteBundle {
                material: materials.add(Color::hex("3C3C3C").unwrap().into()),
                transform: Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + PADDLE_HEIGHT / 2.0 + WALL_SIZE, 0.0),
                sprite: Sprite::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                ..Default::default()
            }
        )
        .insert(Paddle { speed: 500.0 })
        .insert(Collider::Paddle);
    }

    pub fn run(
        time: Res<Time>, 
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<(&Paddle, &mut Transform)>
    ) {
        if let Ok((paddle, mut transform)) = query.single_mut() {
            let mut direction = 0.0;
            if keyboard_input.pressed(KeyCode::Left) {
                direction -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::Right) {
                direction += 1.0;
            }

            let edge_x = WINDOW_WIDTH / 2.0 - PADDLE_WIDTH / 2.0 - WALL_SIZE;
            let translation = &mut transform.translation;
            translation.x += time.delta_seconds() * direction * paddle.speed;
            translation.x = translation.x
            .min(edge_x)
            .max(-edge_x);
        }
    }
}