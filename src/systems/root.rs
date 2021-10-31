use bevy::{app::AppExit, prelude::*};

use crate::AppState;

pub struct RootSystem;

impl RootSystem {
    pub fn run(
        keyboard_input: Res<Input<KeyCode>>,
        mut app_exit_events: EventWriter<AppExit>,
        mut game_state: ResMut<State<AppState>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            match game_state.current() {
                AppState::MainMenu => {
                    app_exit_events.send(AppExit);
                },
                AppState::LevelSelect => {
                    game_state.set(AppState::MainMenu).unwrap();
                },
                AppState::Playing => {
                    game_state.set(AppState::LevelSelect).unwrap();
                },
            }
            return;
        }
        else if keyboard_input.just_pressed(KeyCode::Space) {
            match game_state.current() {
                AppState::MainMenu => {
                    game_state.set(AppState::LevelSelect).unwrap();
                },
                AppState::LevelSelect => {
                    game_state.set(AppState::Playing).unwrap();
                },
                _ => {}
            }
        }
    }
}