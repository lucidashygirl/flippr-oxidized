use crate::data::components::{Controller, Disabled, FlipPlayer, Player};
use avian2d::prelude::*;
use bevy::prelude::*;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (get_player_inputs, input_flip))
            .add_systems(Update, (move_player, flip_player))
            .add_systems(PostUpdate, remove_flip_component);
    }
}

fn get_player_inputs(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Controller, (With<Player>, Without<Disabled>)>,
) {
    let left = input.pressed(KeyCode::ArrowLeft) as i32 as f32;
    let right = input.pressed(KeyCode::ArrowRight) as i32 as f32;

    query
        .iter_mut()
        .for_each(|mut controller| controller.direction = right - left);
}

fn move_player(
    mut query: Query<(&mut LinearVelocity, &Controller), (With<Player>, Without<Disabled>)>,
) {
    query
        .iter_mut()
        .for_each(|(mut linear_velocity, controller)| {
            linear_velocity.x = 100.0 * controller.direction;
        });
}

fn input_flip(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut entity: Query<Entity, (With<Player>, Without<Disabled>)>,
) {
    if !input.just_pressed(KeyCode::KeyZ) {
        return;
    }
    entity.iter_mut().for_each(|e| {
        commands.entity(e).insert(FlipPlayer);
    });
}

fn flip_player(mut query: Query<(&mut Sprite, &mut GravityScale), With<FlipPlayer>>) {
    query
        .iter_mut()
        .for_each(|(mut sprite, mut gravity_scale)| {
            sprite.flip_y = !sprite.flip_y;
            gravity_scale.0 *= -1.0;
        });
}

fn remove_flip_component(mut commands: Commands, mut entity: Query<Entity, With<FlipPlayer>>) {
    entity.iter_mut().for_each(|e| {
        commands.entity(e).remove::<FlipPlayer>();
    })
}
