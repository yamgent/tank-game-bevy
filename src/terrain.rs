use bevy::prelude::*;
use heron::prelude::*;

use crate::game_layer::GameLayer;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_terrain);
    }
}

const TERRAIN_SIZE: f32 = 512.0;

fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let plane = shape::Plane { size: TERRAIN_SIZE };

    let texture = StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(asset_server.load("grass.png")),
        ..Default::default()
    };

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(plane.into()),
            material: materials.add(texture),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(TERRAIN_SIZE / 2.0, 1.0, TERRAIN_SIZE / 2.0),
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(GameLayer::World)
                .with_masks(&[GameLayer::Player, GameLayer::Tower, GameLayer::Bullet]),
        );
}
