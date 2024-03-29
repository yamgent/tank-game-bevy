use bevy::prelude::*;
use heron::prelude::*;

use crate::{
    bullets::{BulletAssets, BulletType},
    game_layer::GameLayer,
    player::Player,
};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_towers)
            .add_system(update_alive_status)
            .add_system(shoot_bullets);
    }
}

const CUBE_SIZE: f32 = 4.0;
const SHOOT_INTERVAL: f32 = 2.0;
const TOWER_PLAYER_MIN_DISTANCE: f32 = 40.0;

#[derive(Component)]
pub struct TowerHead {
    pub alive: bool,
    initial_y: f32,
    shoot_time: f32,
}

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
            let total_height = 6;
            (0..total_height)
                .map(|i| (i, cube.clone(), cube_material.clone()))
                .for_each({
                    move |(i, cube, cube_material)| {
                        let y = (i as f32) * CUBE_SIZE + 0.5;

                        let mut section = parent.spawn();

                        section
                            .insert_bundle(PbrBundle {
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
                            })
                            .insert(Velocity::default())
                            .insert(PhysicMaterial {
                                restitution: 0.0,
                                density: 1500.0,
                                friction: 1.0,
                                ..Default::default()
                            })
                            .insert(
                                CollisionLayers::none()
                                    .with_group(GameLayer::Tower)
                                    .with_masks(&[
                                        GameLayer::Player,
                                        GameLayer::World,
                                        GameLayer::Bullet,
                                        GameLayer::Tower,
                                    ]),
                            );

                        if i == total_height - 1 {
                            section.insert(TowerHead {
                                alive: true,
                                initial_y: y,
                                shoot_time: SHOOT_INTERVAL,
                            });
                        }
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

    [
        Vec3::new(10.0, 0.0, 10.0),
        Vec3::new(40.0, 0.0, 10.0),
        Vec3::new(70.0, 0.0, 40.0),
        Vec3::new(-20.0, 0.0, -80.0),
        Vec3::new(-30.0, 0.0, -60.0),
        Vec3::new(-20.0, 0.0, -60.0),
        Vec3::new(-30.0, 0.0, -80.0),
        Vec3::new(-30.0, 0.0, 110.0),
        Vec3::new(-40.0, 0.0, 80.0),
        Vec3::new(-45.0, 0.0, 70.0),
        Vec3::new(80.0, 0.0, -30.0),
        Vec3::new(-60.0, 0.0, -90.0),
        Vec3::new(100.0, 0.0, 95.0),
        Vec3::new(-95.0, 0.0, -10.0),
    ]
    .into_iter()
    .for_each(|pos| {
        spawn_tower(&mut commands, pos, cube.clone(), cube_material.clone());
    });
}

fn update_alive_status(
    mut query: Query<(&mut TowerHead, &Transform, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    query.iter_mut().filter(|(head, _, _)| head.alive).for_each(
        |(mut head, transform, mut material)| {
            if transform.translation.y < head.initial_y - CUBE_SIZE * 2.0 {
                head.alive = false;
                *material = materials.add(Color::GRAY.into());
            }
        },
    );
}

fn shoot_bullets(
    time: Res<Time>,
    mut commands: Commands,
    bullet_assets: Res<BulletAssets>,
    mut query: Query<(&mut TowerHead, &GlobalTransform)>,
    player_query: Query<&GlobalTransform, (With<Player>, Without<TowerHead>)>,
) {
    let player_transform = player_query.single();

    query
        .iter_mut()
        .filter(|(head, _)| head.alive)
        .for_each(|(mut head, transform)| {
            head.shoot_time -= time.delta_seconds();

            if head.shoot_time <= 0.0 {
                head.shoot_time = SHOOT_INTERVAL;

                let direction = player_transform.translation - transform.translation;

                if direction.length() < TOWER_PLAYER_MIN_DISTANCE {
                    let offset = Vec3::new(direction.normalize().x, 0.0, direction.normalize().z);

                    crate::bullets::spawn_bullet(
                        &mut commands,
                        &bullet_assets,
                        transform.translation + (offset * CUBE_SIZE * 1.25),
                        direction,
                        BulletType::Tower,
                    );
                }
            }
        });
}
