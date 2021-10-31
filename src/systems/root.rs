use bevy::{app::AppExit, prelude::*};

use crate::AppState;

use super::Scoreboard;

pub struct RootSystem;

impl RootSystem {
    pub fn run(
        keyboard_input: Res<Input<KeyCode>>,
        mut app_exit_events: EventWriter<AppExit>,
        mut app_state: ResMut<State<AppState>>,
        mut scoreboard: ResMut<Scoreboard>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            match app_state.current() {
                AppState::MainMenu => {
                    app_exit_events.send(AppExit);
                },
                AppState::LevelSelect => {
                    app_state.set(AppState::MainMenu).unwrap();
                },
                AppState::Playing => {
                    app_state.set(AppState::LevelSelect).unwrap();
                },
            }
            return;
        }
        else if keyboard_input.just_pressed(KeyCode::Space) {
            match app_state.current() {
                AppState::MainMenu => {
                    app_state.set(AppState::LevelSelect).unwrap();
                },
                AppState::LevelSelect => {
                    scoreboard.score = 0;
                    scoreboard.lives = 3;
                    app_state.set(AppState::Playing).unwrap();
                },
                _ => {}
            }
        }
        else if keyboard_input.just_pressed(KeyCode::R) {
            match app_state.current() {
                AppState::Playing => {
                    scoreboard.score = 0;
                    scoreboard.lives = 3;
                },
                _ => {}
            }
        }
    }
}