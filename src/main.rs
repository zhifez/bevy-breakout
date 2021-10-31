mod systems;

use bevy::{DefaultPlugins, prelude::*, render::camera::Camera};

use systems::{BallSystem, CollisionSystem, LevelSelectSystem, MainMenuSystem, PaddleSystem, RootSystem, Scoreboard, ScoreboardSystem};

use bevy_reflect::TypeUuid;

use bevy_asset_ron::*;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

pub struct GameState {
    pub selected_level: usize,
}

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "c2073af1-de7e-4126-a074-3ac5779a6d8b"]
pub struct GameLevelAsset {
    pub name: String,
    pub bricks: Vec<Vec<usize>>,
}

// #[derive(Component)]
pub struct GameLevel {
    pub handle: HandleUntyped,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
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
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let handles = asset_server.load_folder("levels").unwrap();
    for handle in handles {
        commands.spawn_bundle((GameLevel { handle },));
    }
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
    .insert_resource(GameState { selected_level: 0 })
    .insert_resource(
        Scoreboard { 
            score: 0,
            lives: 3,
        }
    )
    .add_plugins(DefaultPlugins)
    .add_plugin(RonAssetPlugin::<GameLevelAsset>::new(&["level"]))
    .add_state(AppState::MainMenu)
    .add_startup_system(setup.system())
    .add_system(RootSystem::run.system())
    // MainMenu state
    .add_system_set(
        SystemSet::on_enter(AppState::MainMenu)
        .with_system(MainMenuSystem::setup.system())
    )
    .add_system_set(
        SystemSet::on_update(AppState::MainMenu)
        .with_system(MainMenuSystem::run.system())
    )
    .add_system_set(
        SystemSet::on_exit(AppState::MainMenu)
        .with_system(teardown.system())
    )
    // LevelSelect state
    .add_system_set(
        SystemSet::on_enter(AppState::LevelSelect)
        .with_system(LevelSelectSystem::setup.system())
    )
    .add_system_set(
        SystemSet::on_update(AppState::LevelSelect)
        .with_system(LevelSelectSystem::run.system())
    )
    .add_system_set(
        SystemSet::on_exit(AppState::LevelSelect)
        .with_system(teardown.system())
    )
    // Playing state
    .add_system_set( 
        SystemSet::on_enter(AppState::Playing)
        .with_system(PaddleSystem::setup.system())
        .with_system(BallSystem::setup.system())
        .with_system(ScoreboardSystem::setup.system())
        .with_system(CollisionSystem::setup.system())
    )
    .add_system_set(
        SystemSet::on_update(AppState::Playing)
        .with_system(PaddleSystem::run.system())
        .with_system(BallSystem::run.system())
        .with_system(ScoreboardSystem::run.system())
        .with_system(CollisionSystem::run.system())
    )
    .add_system_set(
        SystemSet::on_exit(AppState::Playing)
        .with_system(teardown.system())
    )
    .run();
}

fn teardown(mut commands: Commands, entities: Query<Entity, (Without<Camera>, Without<GameLevel>)>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}