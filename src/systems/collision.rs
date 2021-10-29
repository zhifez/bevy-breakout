use bevy::{prelude::*, sprite::collide_aabb::{Collision, collide}};

use crate::{Collider, WINDOW_HEIGHT, WINDOW_WIDTH};

use super::*;

pub const WALL_SIZE: f32 = 10.0;
pub const BRICK_ROWS: i32 = 4;
pub const BRICK_COLUMNS: i32 = 5;
pub const BRICK_SPACING: f32 = 20.0;
pub const BRICK_WIDTH: f32 = 100.0;
pub const BRICK_HEIGHT: f32 = 30.0;

pub struct CollisionSystem;

impl CollisionSystem {
    pub fn setup(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        // ADD WALLS
        let wall_material = materials.add(Color::hex("3C3C3C").unwrap().into());

        // LEFT
        commands
        .spawn_bundle(
            SpriteBundle {
                material: wall_material.clone(),
                transform: Transform::from_xyz(-WINDOW_WIDTH / 2.0 + WALL_SIZE / 2.0, 0.0, 0.0),
                sprite: Sprite::new(Vec2::new(WALL_SIZE, WINDOW_HEIGHT)),
                ..Default::default()
            }
        )
        .insert(Collider::Solid);

        // RIGHT
        commands
        .spawn_bundle(
            SpriteBundle {
                material: wall_material.clone(),
                transform: Transform::from_xyz(WINDOW_WIDTH / 2.0 - WALL_SIZE / 2.0, 0.0, 0.0),
                sprite: Sprite::new(Vec2::new(WALL_SIZE, WINDOW_HEIGHT)),
                ..Default::default()
            }
        )
        .insert(Collider::Solid);

        // TOP
        commands
        .spawn_bundle(
            SpriteBundle {
                material: wall_material.clone(),
                transform: Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - WALL_SIZE / 2.0, 0.0),
                sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, WALL_SIZE)),
                ..Default::default()
            }
        )
        .insert(Collider::Solid);

        // BOTTOM
        commands
        .spawn_bundle(
            SpriteBundle {
                material: wall_material.clone(),
                transform: Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + WALL_SIZE / 2.0, 0.0),
                sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, WALL_SIZE)),
                ..Default::default()
            }
        )
        .insert(Collider::Solid);

        // ADD BRICKS
        let bricks_width = BRICK_COLUMNS as f32 * (BRICK_WIDTH + BRICK_SPACING) - BRICK_SPACING;
        let bricks_offset = Vec3::new(
            -(bricks_width / 2.0 - BRICK_WIDTH / 2.0), 
            WINDOW_HEIGHT / 2.0 - BRICK_HEIGHT / 2.0 - WALL_SIZE * 2.0, 
            0.0
        );
        let brick_material = materials.add(Color::hex("C1839F").unwrap().into());

        for row in 0..BRICK_ROWS {
            let y_position = row as f32 * (BRICK_HEIGHT + BRICK_SPACING);
            for column in 0..BRICK_COLUMNS {
                let brick_position = Vec3::new(
                    column as f32 * (BRICK_WIDTH + BRICK_SPACING),
                    -y_position,
                    0.0,
                ) + bricks_offset;
                commands
                .spawn_bundle(
                    SpriteBundle {
                        material: brick_material.clone(),
                        sprite: Sprite::new(Vec2::new(BRICK_WIDTH, BRICK_HEIGHT)),
                        transform: Transform::from_translation(brick_position),
                        ..Default::default()
                    }
                )
                .insert(Collider::Scorable);
            }
        }
    }

    pub fn run(
        mut commands: Commands,
        mut scoreboard: ResMut<Scoreboard>,
        mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
        collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
    ) {
        if let Ok((mut ball, ball_transform, sprite)) = ball_query.single_mut() {
            let ball_size = sprite.size;
            let velocity = &mut ball.velocity;

            for (collider_entity, collider, transform, sprite) in collider_query.iter() {
                let collision = collide(
                    ball_transform.translation,
                    ball_size,
                    transform.translation,
                    sprite.size,
                );

                // If hit block, despawn/destroy it
                if let Some(collision) = collision {
                    if let Collider::Scorable = *collider {
                        scoreboard.score += 1;
                        commands.entity(collider_entity).despawn();
                    }

                    let mut reflect_x = false;
                    let mut reflect_y = false;

                    match collision {
                        Collision::Left => reflect_x = velocity.x > 0.0,
                        Collision::Right => reflect_x = velocity.x < 0.0,
                        Collision::Top => reflect_y = velocity.y < 0.0,
                        Collision::Bottom => reflect_y = velocity.y > 0.0,
                    }

                    if reflect_x {
                        velocity.x = -velocity.x;
                    }

                    if reflect_y {
                        velocity.y = -velocity.y;
                    }

                    if let Collider::Solid = *collider {
                        break;
                    }
                }
            }
        }
    }
}