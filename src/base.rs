use bevy::prelude::*;

// !!! Debugging
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Base {
  pub health: i32
}