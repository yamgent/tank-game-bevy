mod game_camera;
mod terrain;

use crate::game_camera::GameCameraPlugin;
use crate::terrain::TerrainPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GameCameraPlugin)
        .add_plugin(TerrainPlugin)
        .run();
}
