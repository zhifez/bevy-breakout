mod systems;

use bevy::{DefaultPlugins, prelude::*, render::camera::Camera};

use systems::{BallSystem, CollisionSystem, MainMenuSystem, PaddleSystem, RootSystem, Scoreboard, ScoreboardSystem};

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    MainMenu,
    LevelSelect,
    Playing,
}

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
    .add_state(GameState::MainMenu)
    .add_startup_system(setup.system())
    .add_system(RootSystem::run.system())
    // Menu state
    .add_system_set(
        SystemSet::on_enter(GameState::MainMenu)
        .with_system(MainMenuSystem::setup.system())
    )
    .add_system_set(
        SystemSet::on_update(GameState::MainMenu)
    )
    .add_system_set(
        SystemSet::on_exit(GameState::MainMenu)
        .with_system(teardown.system())
    )
    // Playing state
    .add_system_set( 
        SystemSet::on_enter(GameState::Playing)
        .with_system(PaddleSystem::setup.system())
        .with_system(BallSystem::setup.system())
        .with_system(ScoreboardSystem::setup.system())
        .with_system(CollisionSystem::setup.system())
    )
    .add_system_set(
        SystemSet::on_update(GameState::Playing)
        .with_system(PaddleSystem::run.system())
        .with_system(BallSystem::run.system())
        .with_system(ScoreboardSystem::run.system())
        .with_system(CollisionSystem::run.system())
    )
    .add_system_set(
        SystemSet::on_exit(GameState::Playing)
        .with_system(teardown.system())
    )
    .run();
}

fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}