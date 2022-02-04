use bevy::prelude::*;

use crate::player::Player;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_system(look_at_player);
    }
}

#[derive(Component)]
struct ViewCamera;

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle::new_3d())
        .insert(ViewCamera);

    // TODO: Better place to put light?
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 32000.0,
            ..Default::default()
        },
        transform: Transform::default()
            .looking_at(Vec3::new(0.0, -1.0, -1.0), Vec3::new(0.0, 1.0, 0.0)),
        ..Default::default()
    });
}

fn look_at_player(
    mut query: Query<&mut Transform, With<ViewCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<ViewCamera>)>,
) {
    let mut transform = query.single_mut();
    let player_transform = player_query.single();

    *transform =
        Transform::from_translation(player_transform.translation + Vec3::new(0.0, 60.0, 60.0))
            .looking_at(player_transform.translation, Vec3::new(0.0, 1.0, 0.0));
}
