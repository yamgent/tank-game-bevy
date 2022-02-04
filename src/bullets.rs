use crate::{game_layer::GameLayer, player::PlayerHit};
use bevy::prelude::*;
use heron::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_bullet)
            .add_system(move_bullets)
            .add_system(auto_despawn_bullets)
            .add_system(handle_bullets_collisions);
    }
}

const BULLET_SIZE_RADIUS: f32 = 1.0;
const BULLET_SPEED: f32 = 12.0;
const BULLET_LIFE: f32 = 30.0; // in case it goes out of range

pub struct BulletAssets {
    mesh: Handle<Mesh>,
    enemy_material: Handle<StandardMaterial>,
}

#[derive(Component)]
struct Move {
    velocity: Vec3,
}

#[derive(Component)]
struct AutoDespawn {
    time_left: f32,
}

fn setup_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(
        shape::Icosphere {
            radius: BULLET_SIZE_RADIUS,
            subdivisions: 8,
        }
        .into(),
    );
    let enemy_material = materials.add(Color::RED.into());

    commands.insert_resource(BulletAssets {
        mesh,
        enemy_material,
    });
}

pub fn spawn_bullet(
    commands: &mut Commands,
    assets: &Res<BulletAssets>,
    position: Vec3,
    direction: Vec3,
) {
    let direction = direction.normalize();

    commands
        .spawn_bundle(PbrBundle {
            mesh: assets.mesh.clone(),
            material: assets.enemy_material.clone(),
            transform: Transform {
                translation: position,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Sensor)
        .insert(CollisionShape::Sphere {
            radius: BULLET_SIZE_RADIUS,
        })
        .insert(Move {
            velocity: direction,
        })
        .insert(AutoDespawn {
            time_left: BULLET_LIFE,
        })
        .insert(
            CollisionLayers::none()
                .with_group(GameLayer::Bullet)
                .with_masks(&[GameLayer::Player, GameLayer::World]),
        );
}

fn move_bullets(time: Res<Time>, mut query: Query<(&mut Transform, &Move)>) {
    query.iter_mut().for_each(|(mut transform, mover)| {
        transform.translation += mover.velocity * time.delta_seconds() * BULLET_SPEED;
    });
}

fn auto_despawn_bullets(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut AutoDespawn)>,
) {
    query.iter_mut().for_each(|(entity, mut auto_despawn)| {
        auto_despawn.time_left -= time.delta_seconds();

        if auto_despawn.time_left <= 0.0 {
            commands.entity(entity).despawn();
        }
    });
}

fn handle_bullets_collisions(
    mut events: EventReader<CollisionEvent>,
    mut player_hit: EventWriter<PlayerHit>,
    mut commands: Commands,
) {
    events.iter().for_each(|event| {
        if let CollisionEvent::Started(data1, data2) = event {
            let datas = if data1.collision_layers().contains_group(GameLayer::Bullet) {
                Some((data1, data2))
            } else if data2.collision_layers().contains_group(GameLayer::Bullet) {
                Some((data2, data1))
            } else {
                None
            };

            if let Some((bullet, other)) = datas {
                if other.collision_layers().contains_group(GameLayer::Player) {
                    player_hit.send(PlayerHit);
                }

                commands.entity(bullet.rigid_body_entity()).despawn();
            }
        }
    });
}
