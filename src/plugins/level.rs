use crate::data::bundles::{
    BlueWallBundle, GreenWallBundle, PlayerBundle, PurpleWallBundle, RedWallBundle,
    YellowWallBundle,
};
use crate::data::components::{
    BlueWall, GreenWall, Player, PlayerAddons, PlayerSensor, PurpleWall, RedWall, YellowWall,
};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_int_cell::<BlueWallBundle>(1)
            .register_ldtk_int_cell::<RedWallBundle>(2)
            .register_ldtk_int_cell::<YellowWallBundle>(3)
            .register_ldtk_int_cell::<PurpleWallBundle>(4)
            .register_ldtk_int_cell::<GreenWallBundle>(5)
            .add_systems(Startup, spawn_project)
            .add_systems(
                Update,
                (
                    process_player,
                    generate_blue_tiles,
                    generate_red_tiles,
                    generate_yellow_tiles,
                    generate_purple_tiles,
                    generate_green_tiles,
                ),
            );
    }
}

fn spawn_project(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server
            .load("level_packs/flippr_classic/flippr.ldtk")
            .into(),
        ..Default::default()
    });
}

fn level_swap(
    commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    level_select: Res<LevelSelection>,
) {
}

fn process_player(
    mut commands: Commands,
    new_entity_instances: Query<Entity, (Without<PlayerAddons>, With<Player>)>,
) {
    new_entity_instances.iter().for_each(|entity| {
        commands
            .entity(entity)
            .insert((PlayerAddons, children![PlayerSensor::new(14.1, 14.1)]));
    });
}

use std::collections::{HashMap, HashSet};

collision_generator!(generate_blue_tiles, BlueWall);
collision_generator!(generate_red_tiles, RedWall);
collision_generator!(generate_green_tiles, GreenWall);
collision_generator!(generate_yellow_tiles, YellowWall);
collision_generator!(generate_purple_tiles, PurpleWall);
