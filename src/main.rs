mod game_camera;
mod player;
mod terrain;

use crate::game_camera::GameCameraPlugin;
use crate::player::PlayerPlugin;
use crate::terrain::TerrainPlugin;
use bevy::prelude::*;
use heron::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))
        .add_plugin(GameCameraPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
