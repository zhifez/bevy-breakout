use bevy::prelude::*;

use super::{Scoreboard};

pub struct GameOverSystem;
pub enum GameOverUi {
    Static,
    Title,
}

impl GameOverSystem {
    pub fn setup(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>,
    ) {
        commands
        .spawn_bundle(
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                visible: Visible { 
                    is_visible: false,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            }
        )
        .with_children(|parent| {
            parent
            .spawn_bundle(
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(70.0), Val::Percent(50.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: Rect::all(Val::Px(32.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::hex("087E8B").unwrap().into()),
                    ..Default::default()
                }
            )
            .insert(GameOverUi::Static)
            .with_children(|parent| {
                parent
                .spawn_bundle(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection {
                                    value: "Game Over".to_string(),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/square.ttf"),
                                        font_size: 50.0,
                                        color: Color::hex("F5F5F5").unwrap(),
                                    },
                                },
                            ],
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                )
                .insert(GameOverUi::Title);

                parent
                .spawn_bundle(
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        material: materials.add(Color::NONE.into()),
                        ..Default::default()
                    }
                );

                parent
                .spawn_bundle(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection {
                                    value: "Press R to restart level".to_string(),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/square.ttf"),
                                        font_size: 20.0,
                                        color: Color::hex("F5F5F5").unwrap(),
                                    },
                                },
                            ],
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                )
                .insert(GameOverUi::Static);

                parent
                .spawn_bundle(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection {
                                    value: "Press Escape to return to level select".to_string(),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/square.ttf"),
                                        font_size: 20.0,
                                        color: Color::hex("F5F5F5").unwrap(),
                                    },
                                },
                            ],
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                )
                .insert(GameOverUi::Static);
            });
        });
    }

    pub fn run(
        scoreboard: Res<Scoreboard>,
        mut query: Query<(&GameOverUi, &mut Visible, Option<&mut Text>)>,
    ) {
        for (game_over_ui, mut visible, text) in query.iter_mut() {
            visible.is_visible = scoreboard.lives <= 0 || scoreboard.score >= scoreboard.maxScores;

            if let Some(mut t) = text {
                if let GameOverUi::Title = game_over_ui {
                    if scoreboard.score >= scoreboard.maxScores {
                        t.sections[0].value = "Level Complete".to_string();
                    }
                    if scoreboard.lives <= 0 {
                        t.sections[0].value = "Game Over".to_string();
                    }
                }
            }
        }
    }
}