use bevy::prelude::*;
use heron::prelude::*;
use std::f32::consts::PI;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player)
            .add_system(handle_player_input)
            .add_system(handle_player_movement);
    }
}

const ROTATION_SPEED: f32 = 0.2;
const MOVING_SPEED: f32 = 200.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct InputDirection(Vec3);

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
        .insert(PhysicMaterial {
            density: 2000.0,
            ..Default::default()
        })
        .insert(Velocity::default())
        .insert(Player)
        .insert(InputDirection(Vec3::ZERO))
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

fn handle_player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut InputDirection, With<Player>>,
) {
    let mut input_direction = query.single_mut();

    let forward = keyboard_input.pressed(KeyCode::W);
    let backward = keyboard_input.pressed(KeyCode::S);
    let left = keyboard_input.pressed(KeyCode::A);
    let right = keyboard_input.pressed(KeyCode::D);

    input_direction.0 = Vec3::new(
        if left && !right {
            1.0
        } else if right && !left {
            -1.0
        } else {
            0.0
        },
        0.0,
        if forward && !backward {
            1.0
        } else if backward && !forward {
            -1.0
        } else {
            0.0
        },
    );
}

fn handle_player_movement(
    time: Res<Time>,
    mut query: Query<(&InputDirection, &mut Velocity, &Transform), With<Player>>,
) {
    let (dir, mut velocity, transform) = query.single_mut();

    if dir.0.x != 0.0 || dir.0.z != 0.0 {
        // TODO: See whether we can improve this system
        // Problem 1: When in the target direction, the turning can overshoot and cause jittering
        // Problem 2: If the tank is rotated wildly (e.g when falling off the map), the tank speed
        // suddenly becomes very fast, which can crash the physics engine
        let facing_direction = transform.local_x();

        velocity.linear = Vec3::new(facing_direction.x, velocity.linear.y, facing_direction.z)
            * MOVING_SPEED
            * time.delta_seconds();

        let facing_direction = facing_direction.z.atan2(facing_direction.x);
        let target_direction = dir.0.z.atan2(dir.0.x);

        let delta_angle = if facing_direction > target_direction {
            let left = (target_direction + 2.0 * PI) - facing_direction;
            let right = facing_direction - target_direction;

            if left < right {
                1.0
            } else {
                -1.0
            }
        } else {
            let left = target_direction - facing_direction;
            let right = facing_direction - (target_direction - 2.0 * PI);

            if left < right {
                1.0
            } else {
                -1.0
            }
        };

        velocity.angular = AxisAngle::new(
            Vec3::new(0.0, 1.0, 0.0),
            delta_angle * 360.0 * ROTATION_SPEED * time.delta_seconds(),
        );
    }
}
