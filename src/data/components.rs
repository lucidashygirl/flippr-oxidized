use avian2d::prelude::{
    Collider, CollisionMargin, Friction, GravityScale, LockedAxes, Mass, Restitution, RigidBody,
    Sensor, SleepingDisabled,
};
use bevy::prelude::Component;

#[derive(Component, Default)]
#[require(
    Controller::default(),
    RigidBody::Dynamic,
    GravityScale(87.7), // to scale with original grav
    Friction::ZERO,
    Restitution::ZERO,
    SleepingDisabled,
    Mass(5.0),
    LockedAxes::ROTATION_LOCKED,
    Collider::rectangle(14., 14.),
    CollisionMargin(0.01),
)]
pub struct PlayerAddons;

#[derive(Component, Default)]
#[require(SleepingDisabled, LockedAxes::ROTATION_LOCKED, Sensor)]
pub struct PlayerSensor;

impl PlayerSensor {
    pub fn new(x: f32, y: f32) -> (PlayerSensor, Collider) {
        (PlayerSensor, Collider::rectangle(x, y))
    }
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct Controller {
    pub direction: f32,
}

#[derive(Component)]
pub struct FlipPlayer;

#[derive(Component)]
pub struct Disabled;

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
