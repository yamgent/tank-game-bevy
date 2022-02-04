use bevy::prelude::*;
use heron::prelude::*;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_towers);
    }
}

const CUBE_SIZE: f32 = 4.0;

fn spawn_tower(
    commands: &mut Commands,
    position: Vec3,
    cube: Handle<Mesh>,
    cube_material: Handle<StandardMaterial>,
) {
    commands
        .spawn_bundle((
            Transform {
                translation: position,
                ..Default::default()
            },
            GlobalTransform::default(),
        ))
        .with_children(|parent| {
            (0..6)
                .map(|i| (i, cube.clone(), cube_material.clone()))
                .for_each({
                    move |(i, cube, cube_material)| {
                        let y = (i as f32) * CUBE_SIZE;
                        parent
                            .spawn_bundle(PbrBundle {
                                mesh: cube,
                                material: cube_material,
                                ..Default::default()
                            })
                            .insert_bundle((
                                Transform {
                                    translation: Vec3::new(0.0, y, 0.0),
                                    ..Default::default()
                                },
                                GlobalTransform::default(),
                            ))
                            .insert(RigidBody::Dynamic)
                            .insert(CollisionShape::Cuboid {
                                half_extends: Vec3::new(
                                    CUBE_SIZE / 2.0,
                                    CUBE_SIZE / 2.0,
                                    CUBE_SIZE / 2.0,
                                ),
                                border_radius: None,
                            });
                    }
                });
        });
}

fn setup_towers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = meshes.add(shape::Cube { size: CUBE_SIZE }.into());
    let cube_material = materials.add(Color::WHITE.into());

    spawn_tower(
        &mut commands,
        Vec3::new(10.0, 0.0, 10.0),
        cube.clone(),
        cube_material.clone(),
    );
}
