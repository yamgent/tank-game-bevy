use bevy::prelude::*;
use heron::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_bullet)
            .add_system(move_bullets)
            .add_system(auto_despawn_bullets);
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
        });
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

// TODO: Collision detection with player
