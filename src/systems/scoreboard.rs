use bevy::prelude::*;

use crate::systems::collision::WALL_SIZE;

pub struct Scoreboard {
    pub score: usize,
    pub maxScores: usize,
    pub lives: usize,
}
pub struct ScoreboardLabel(String);

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
                                color: Color::hex("F5F5F5").unwrap(),
                            },
                        },
                    ],
                    ..Default::default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(WALL_SIZE / 2.0),
                        left: Val::Px(WALL_SIZE),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            }
        )
        .insert(ScoreboardLabel("score_label".to_string()));

        commands
        .spawn_bundle(
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Lives:".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/square.ttf"),
                                font_size: 40.0,
                                color: Color::hex("F5F5F5").unwrap(),
                            },
                        },
                    ],
                    ..Default::default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(WALL_SIZE / 2.0),
                        right: Val::Px(WALL_SIZE),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            }
        )
        .insert(ScoreboardLabel("lives_label".to_string()));
    }

    pub fn run(
        scoreboard: Res<Scoreboard>, 
        mut query: Query<(&mut Text, &ScoreboardLabel)>,
    ) {
        for (mut text, label_name) in query.iter_mut() {
            if label_name.0 == "score_label" {
                text.sections[0].value = format!("Score: {}", scoreboard.score);
            }
            else if label_name.0 == "lives_label" {
                text.sections[0].value = format!("Lives: {}", scoreboard.lives);
            }
        }
    }
}