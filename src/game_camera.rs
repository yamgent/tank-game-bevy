use bevy::prelude::*;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_system(look_at_player);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle::new_3d());

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

fn look_at_player(mut query: Query<&mut Transform, With<Camera>>) {
    // TODO: Actually look at player
    let mut transform = query.single_mut();
    *transform = Transform::from_translation(Vec3::new(0.0, 20.0, 30.0))
        .looking_at(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0));
}
