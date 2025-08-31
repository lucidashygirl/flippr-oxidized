use crate::data::components::{BlueWall, GreenWall, Player, PurpleWall, RedWall, YellowWall};
use bevy::prelude::{Bundle, Sprite};
use bevy_ecs_ldtk::prelude::{LdtkEntity, LdtkIntCell};
#[derive(Bundle, LdtkEntity, Default)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet_bundle: Sprite,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct BlueWallBundle {
    wall: BlueWall,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct RedWallBundle {
    wall: RedWall,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct YellowWallBundle {
    wall: YellowWall,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct PurpleWallBundle {
    wall: PurpleWall,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct GreenWallBundle {
    wall: GreenWall,
}
