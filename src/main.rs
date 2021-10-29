mod systems;

use bevy::{DefaultPlugins, prelude::*};

use systems::{BallSystem, CollisionSystem, PaddleSystem, Scoreboard, ScoreboardSystem};

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

pub enum Collider {
    Solid,
    Scorable,
    Paddle,
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn main() {
    App::build()
    .insert_resource(WindowDescriptor {
        title: "Breakout".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        vsync: true,
        resizable: false,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::hex("F5F5F5").unwrap()))
    .insert_resource(Scoreboard { score: 0 })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .add_startup_system(PaddleSystem::setup.system())
    .add_startup_system(BallSystem::setup.system())
    .add_startup_system(ScoreboardSystem::setup.system())
    .add_startup_system(CollisionSystem::setup.system())
    .add_system(PaddleSystem::run.system())
    .add_system(BallSystem::run.system())
    .add_system(ScoreboardSystem::run.system())
    .add_system(CollisionSystem::run.system())
    .run();
}