mod game_camera;
mod player;
mod terrain;

use crate::game_camera::GameCameraPlugin;
use crate::player::PlayerPlugin;
use crate::terrain::TerrainPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GameCameraPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
