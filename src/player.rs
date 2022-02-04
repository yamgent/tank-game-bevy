use bevy::prelude::*;
use heron::prelude::*;
use std::f32::consts::PI;

use crate::{
    bullets::{BulletAssets, BulletType},
    game_layer::GameLayer,
    game_ui::PlayerHealthUpdated,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerHit>()
            .add_startup_system(setup_player)
            .add_system(handle_player_movement_input)
            .add_system(handle_player_movement)
            .add_system(handle_player_aim_input)
            .add_system(handle_player_aim)
            .add_system(handle_player_hit)
            .add_system(handle_player_shoot_input);
    }
}

const ROTATION_SPEED: f32 = 0.2;
const MOVING_SPEED: f32 = 200.0;
const INITIAL_HEALTH: i32 = 10;
const PLAYER_SIZE: (f32, f32, f32) = (8.0, 3.0, 4.0);

#[derive(Component)]
pub struct Player {
    health: i32,
}

#[derive(Component)]
struct MovementInputDirection(Vec3);

pub struct PlayerHit;

#[derive(Component)]
struct AimInputDirection(Vec3);

#[derive(Component)]
struct TankTop;

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut health_updated: EventWriter<PlayerHealthUpdated>,
) {
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
            half_extends: Vec3::new(
                PLAYER_SIZE.0 / 2.0,
                PLAYER_SIZE.1 / 2.0,
                PLAYER_SIZE.2 / 2.0,
            ),
            border_radius: None,
        })
        .insert(PhysicMaterial {
            restitution: 0.0,
            density: 2000.0,
            ..Default::default()
        })
        .insert(Velocity::default())
        .insert(Player {
            health: INITIAL_HEALTH,
        })
        .insert(MovementInputDirection(Vec3::ZERO))
        .insert(AimInputDirection(Vec3::ZERO))
        .with_children(|parent| {
            parent
                .spawn_bundle((Transform::default(), GlobalTransform::identity()))
                .with_children(|gparent| {
                    gparent.spawn_scene(asset_server.load("tank_bottom.glb#Scene0"));
                });
            parent
                .spawn_bundle((Transform::default(), GlobalTransform::identity()))
                .insert(TankTop)
                .with_children(|gparent| {
                    gparent.spawn_scene(asset_server.load("tank_turret.glb#Scene0"));
                })
                .with_children(|gparent| {
                    gparent.spawn_scene(asset_server.load("tank_barrel.glb#Scene0"));
                });
        })
        .insert(
            CollisionLayers::none()
                .with_group(GameLayer::Player)
                .with_masks(&[GameLayer::Bullet, GameLayer::Tower, GameLayer::World]),
        );

    health_updated.send(PlayerHealthUpdated(INITIAL_HEALTH));
}

fn handle_player_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut MovementInputDirection, With<Player>>,
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
    mut query: Query<(&MovementInputDirection, &mut Velocity, &Transform), With<Player>>,
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

fn handle_player_hit(
    mut query: Query<&mut Player>,
    mut events: EventReader<PlayerHit>,
    mut health_updated: EventWriter<PlayerHealthUpdated>,
) {
    let mut player = query.single_mut();

    events.iter().for_each(|_| {
        player.health -= 1;

        if player.health < 0 {
            player.health = 0;
        }

        health_updated.send(PlayerHealthUpdated(player.health));
    });
}

fn handle_player_aim_input(
    windows: Res<Windows>,
    mut query: Query<&mut AimInputDirection, With<Player>>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(pos) = window.cursor_position() {
        let mut input = query.single_mut();

        let middle = Vec2::new(window.width() / 2.0, window.height() / 2.0);
        let aim_direction = (pos - middle).normalize();
        input.0 = Vec3::new(aim_direction.x, 0.0, aim_direction.y);
    }
}

fn handle_player_aim(
    mut query: Query<(&mut Transform, &Parent), With<TankTop>>,
    parent_query: Query<(&Transform, &AimInputDirection), Without<TankTop>>,
) {
    query.iter_mut().for_each(|(mut transform, parent)| {
        let (parent_transform, aim) = parent_query.get(parent.0).unwrap();

        let parent_facing_direction = parent_transform.local_x();
        let parent_angle = parent_facing_direction.z.atan2(parent_facing_direction.x);

        let angle = if aim.0.x == 0.0 {
            0.0
        } else {
            aim.0.z.atan2(aim.0.x)
        };

        transform.rotation = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), angle + parent_angle);
    });
}

fn handle_player_shoot_input(
    mouse: Res<Input<MouseButton>>,
    query: Query<(&Transform, &AimInputDirection)>,
    mut commands: Commands,
    bullet_assets: Res<BulletAssets>,
) {
    let (transform, aim) = query.single();

    if mouse.just_pressed(MouseButton::Left) {
        let aim = Vec3::new(aim.0.x, 0.0, -aim.0.z);
        let offset = aim * PLAYER_SIZE.0.max(PLAYER_SIZE.2);

        crate::bullets::spawn_bullet(
            &mut commands,
            &bullet_assets,
            transform.translation + offset,
            aim,
            BulletType::Player,
        );
    }
}
