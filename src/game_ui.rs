use bevy::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerHealthUpdated>()
            .add_startup_system(setup_ui)
            .add_system(handle_health_updated);
    }
}

#[derive(Component)]
struct HealthText;

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
                            font,
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
