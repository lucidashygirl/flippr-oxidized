use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, PartialEq)]
pub struct Blue;

#[derive(Component, PartialEq)]
pub struct Red;

#[derive(Component, PartialEq)]
pub struct Yellow;

#[derive(Component, PartialEq)]
pub struct Purple;

#[derive(Component, PartialEq)]
pub struct Green;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct BlueWall;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct RedWall;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct YellowWall;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct PurpleWall;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct GreenWall;
