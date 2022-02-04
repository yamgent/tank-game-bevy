mod bullets;
mod game_camera;
mod game_layer;
mod game_ui;
mod player;
mod terrain;
mod tower;

use bevy::prelude::*;
use heron::prelude::*;

use crate::bullets::BulletPlugin;
use crate::game_camera::GameCameraPlugin;
use crate::game_ui::GameUiPlugin;
use crate::player::PlayerPlugin;
use crate::terrain::TerrainPlugin;
use crate::tower::TowerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))
        .add_plugin(GameCameraPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(GameUiPlugin)
        .run();
}
