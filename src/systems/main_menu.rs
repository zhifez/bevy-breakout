use bevy::prelude::*;

pub struct MainMenuSystem;

impl MainMenuSystem {
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
                                value: "Breakout".to_string(),
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
                                value: "Made with Bevy".to_string(),
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

            parent
            .spawn_bundle(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "Press SPACE to play the game".to_string(),
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
        });
    }
}