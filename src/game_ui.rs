use bevy::prelude::*;

use crate::{player::Player, terrain::TERRAIN_SIZE, tower::TowerHead};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerHealthUpdated>()
            .add_startup_system(setup_ui)
            .add_system(handle_health_updated)
            .add_system(update_player_dot)
            .add_system(ensure_enough_tower_dots)
            .add_system(update_tower_dots)
            .add_system(update_cannon_status);
    }
}

const MAP_COORD: (f32, f32) = (15.0, 50.0);
const MAP_SIZE: (f32, f32) = (160.0, 160.0);

const POS_DOT_SIZE: (f32, f32) = (8.0, 8.0);

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct MapPlayerDot;

#[derive(Component)]
struct MapTowerDot;

#[derive(Component)]
struct CannonText;

pub struct PlayerHealthUpdated(pub i32);

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let font = asset_server.load("FiraSans-Bold.ttf");

    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Health: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HealthText);

    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "Cannon READY".to_string(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                }],
                alignment: TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(45.0),
                    left: Val::Percent(50.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CannonText);

    let map_enclosure = asset_server.load("map_enclosure.png");

    commands.spawn_bundle(ImageBundle {
        image: UiImage(map_enclosure),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(MAP_COORD.1),
                left: Val::Px(MAP_COORD.0),
                ..Default::default()
            },
            size: Size {
                width: Val::Px(MAP_SIZE.0),
                height: Val::Px(MAP_SIZE.1),
            },
            ..Default::default()
        },
        ..Default::default()
    });

    let map_dot = asset_server.load("map_dot.png");
    commands
        .spawn_bundle(ImageBundle {
            image: UiImage(map_dot),
            color: Color::BLUE.into(),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(MAP_COORD.1),
                    left: Val::Px(MAP_COORD.0),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(POS_DOT_SIZE.0),
                    height: Val::Px(POS_DOT_SIZE.1),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MapPlayerDot);
}

fn handle_health_updated(
    mut events: EventReader<PlayerHealthUpdated>,
    mut query: Query<&mut Text, With<HealthText>>,
) {
    let mut text = query.single_mut();

    events.iter().for_each(|event| {
        text.sections[1].value = event.0.to_string();
    });
}

fn update_cannon_status(
    player_query: Query<&Player>,
    mut query: Query<&mut Text, With<CannonText>>,
) {
    let player = player_query.single();
    let mut text = query.single_mut();

    let (value, color) = if player.shoot_cooldown >= 0.1 {
        (
            format!("{}", player.shoot_cooldown.floor().to_string()),
            Color::RED,
        )
    } else {
        ("Cannon READY".to_string(), Color::BLACK)
    };

    text.sections[0].value = value;
    text.sections[0].style.color = color;
}

fn update_player_dot(
    mut query: Query<&mut Style, With<MapPlayerDot>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    let mut ui = query.single_mut();

    let player = (
        (player_transform.translation.x + (TERRAIN_SIZE / 2.0)) / TERRAIN_SIZE,
        (player_transform.translation.z + (TERRAIN_SIZE / 2.0)) / TERRAIN_SIZE,
    );

    ui.position.top = Val::Px(MAP_COORD.1 + MAP_SIZE.1 * player.1);
    ui.position.left = Val::Px(MAP_COORD.0 + MAP_SIZE.0 * player.0);
}

fn ensure_enough_tower_dots(
    query: Query<&MapTowerDot>,
    tower_query: Query<&TowerHead>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut dots_count = query.iter().count();
    let towers_count = tower_query.iter().count();

    while dots_count < towers_count {
        let map_dot = asset_server.load("map_dot.png");
        dots_count += 1;

        commands
            .spawn_bundle(ImageBundle {
                image: UiImage(map_dot),
                color: Color::RED.into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(MAP_COORD.1),
                        left: Val::Px(MAP_COORD.0),
                        ..Default::default()
                    },
                    size: Size {
                        width: Val::Px(POS_DOT_SIZE.0),
                        height: Val::Px(POS_DOT_SIZE.1),
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(MapTowerDot);
    }
}

fn update_tower_dots(
    mut query: Query<(&mut Style, &mut UiColor), With<MapTowerDot>>,
    tower_query: Query<(&GlobalTransform, &TowerHead)>,
) {
    tower_query.iter().zip(query.iter_mut()).for_each(
        |((tower_transform, tower_head), (mut ui, mut color))| {
            let tower = (
                (tower_transform.translation.x + (TERRAIN_SIZE / 2.0)) / TERRAIN_SIZE,
                (tower_transform.translation.z + (TERRAIN_SIZE / 2.0)) / TERRAIN_SIZE,
            );

            ui.position.top = Val::Px(MAP_COORD.1 + MAP_SIZE.1 * tower.1);
            ui.position.left = Val::Px(MAP_COORD.0 + MAP_SIZE.0 * tower.0);
            *color = if tower_head.alive {
                Color::RED.into()
            } else {
                Color::GRAY.into()
            };
        },
    );
}
