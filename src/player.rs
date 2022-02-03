use bevy::prelude::*;
use heron::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player);
    }
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle((
            Transform {
                translation: Vec3::new(0.0, 3.0, 0.0),
                ..Default::default()
            },
            GlobalTransform::identity(),
        ))
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(4.0, 1.5, 2.0),
            border_radius: None,
        })
        .with_children(|parent| {
            parent
                .spawn_bundle((Transform::default(), GlobalTransform::identity()))
                .with_children(|gparent| {
                    gparent.spawn_scene(asset_server.load("tank_bottom.glb#Scene0"));
                })
                .with_children(|gparent| {
                    gparent.spawn_scene(asset_server.load("tank_turret.glb#Scene0"));
                })
                .with_children(|gparent| {
                    gparent.spawn_scene(asset_server.load("tank_barrel.glb#Scene0"));
                });
        });
}
