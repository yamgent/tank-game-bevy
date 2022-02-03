use bevy::prelude::*;
use heron::prelude::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_terrain);
    }
}

fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane = shape::Plane { size: 32. };
    let color = Color::GREEN;

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(plane.into()),
            material: materials.add(color.into()),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(16.0, 1.0, 16.0),
            border_radius: None,
        });
}
