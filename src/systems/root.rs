use bevy::{app::AppExit, prelude::*};

use crate::AppState;

pub struct RootSystem;

impl RootSystem {
    pub fn run(
        keyboard_input: Res<Input<KeyCode>>,
        mut app_exit_events: EventWriter<AppExit>,
        mut game_state: ResMut<State<AppState>>,
    ) {
        if keyboard_input.pressed(KeyCode::Escape) {
            app_exit_events.send(AppExit);
            return;
        }

        match game_state.current() {
            AppState::MainMenu => {
                if keyboard_input.pressed(KeyCode::Space) {
                    game_state.set(AppState::Playing).unwrap();
                }
            },
            _ => {}
        }
    }
}