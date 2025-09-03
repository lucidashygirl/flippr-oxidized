use avian2d::prelude::*;
use bevy::prelude::*;
#[macro_use]
mod macros;
mod data;
mod plugins;
use plugins::camera::CameraPlugin;
use plugins::level::LevelPlugin;
use plugins::player::PlayerControllerPlugin;
use plugins::tiles::TileLogicPlugin;
fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LevelPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerControllerPlugin)
        .add_plugins(TileLogicPlugin)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .run()
}
