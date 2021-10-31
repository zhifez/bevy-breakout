use bevy::prelude::*;

use crate::{GameLevel, GameLevelAsset, GameState};

pub struct LevelSelectSystem;
pub struct LevelName(String);

impl LevelSelectSystem {
    pub fn setup(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>,
        game_state: Res<GameState>,
        game_level_assets: Res<Assets<GameLevelAsset>>,
        game_levels: Query<&GameLevel>,
    ) {
        commands
        .spawn_bundle(
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: materials.add(Color::hex("087E8B").unwrap().into()),
                ..Default::default()
            }
        )
        .with_children(|parent| {
            parent
            .spawn_bundle(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "Level Select".to_string(),
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
            );

            parent
            .spawn_bundle(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "Press space to play level".to_string(),
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
            );
            
            parent
            .spawn_bundle(
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                }
            );

            let mut sorted_game_levels = Vec::<&GameLevelAsset>::new();
            for gl_data in game_levels.iter() {
                if let Some(gl) = game_level_assets.get(&gl_data.handle) {
                    sorted_game_levels.push(gl);
                }
            }
            sorted_game_levels.sort_by_key(|a| a.name.clone());

            for (index, gl) in sorted_game_levels.iter().enumerate() {
                let mut level_name_display = gl.name.to_string();
                if index == game_state.selected_level as usize {
                    level_name_display = format!("{} {} {}", "> ", level_name_display.to_string(), " <");
                }

                parent
                .spawn_bundle(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection {
                                    value: level_name_display,
                                    style: TextStyle {
                                        font: asset_server.load("fonts/square.ttf"),
                                        font_size: 30.0,
                                        color: Color::hex("F5F5F5").unwrap(),
                                    },
                                },
                            ],
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                )
                .insert(LevelName(gl.name.clone()));
            }
        });
    }

    pub fn run(
        keyboard_input: Res<Input<KeyCode>>,
        mut game_state: ResMut<GameState>,
        game_level_assets: Res<Assets<GameLevelAsset>>,
        mut level_texts: Query<(&mut Text, &LevelName)>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Up) 
            || keyboard_input.just_pressed(KeyCode::W)  {
            if game_state.selected_level > 0 {
                game_state.selected_level -= 1;
            }
        }
        if keyboard_input.just_pressed(KeyCode::Down) 
            || keyboard_input.just_pressed(KeyCode::S)  {
            if game_state.selected_level < game_level_assets.len() - 1 {
                game_state.selected_level += 1;
            }
        }

        for (index, (mut text, level_name)) in level_texts.iter_mut().enumerate() {
            text.sections[0].value = level_name.0.to_string();
            if index == game_state.selected_level {
                text.sections[0].value = format!("{} {} {}", "> ", level_name.0.to_string(), " <");
            }
        }
    }
}