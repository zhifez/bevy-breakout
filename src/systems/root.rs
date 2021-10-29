use bevy::{app::AppExit, prelude::*};

use crate::GameState;

pub struct RootSystem;

impl RootSystem {
    pub fn run(
        keyboard_input: Res<Input<KeyCode>>,
        mut app_exit_events: EventWriter<AppExit>,
        mut game_state: ResMut<State<GameState>>,
    ) {
        if keyboard_input.pressed(KeyCode::Escape) {
            app_exit_events.send(AppExit);
            return;
        }

        match game_state.current() {
            GameState::MainMenu => {
                if keyboard_input.pressed(KeyCode::Space) {
                    game_state.set(GameState::Playing).unwrap();
                }
            },
            _ => {}
        }
    }
}