use bevy::prelude::*;

use crate::systems::collision::WALL_SIZE;

pub struct Scoreboard {
    pub score: usize,
}

pub struct ScoreboardSystem;

impl ScoreboardSystem {
    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        commands
        .spawn_bundle(
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Score:".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/square.ttf"),
                                font_size: 40.0,
                                color: Color::hex("3A0CA3").unwrap(),
                            },
                        },
                        TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/square.ttf"),
                                font_size: 40.0,
                                color: Color::hex("3A0CA3").unwrap(),
                            },
                        },
                    ],
                    ..Default::default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(WALL_SIZE * 2.0),
                        left: Val::Px(WALL_SIZE * 2.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            }
        );
    }

    pub fn run(
        scoreboard: Res<Scoreboard>, 
        mut query: Query<&mut Text>,
    ) {
        let mut text = query.single_mut().unwrap();
        text.sections[0].value = format!("Score: {}", scoreboard.score);
    }
}