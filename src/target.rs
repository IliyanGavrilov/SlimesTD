// Debugging
use bevy::prelude::*;
#[derive(Reflect, Component, Default)]
pub enum Target {
  #[default]
  FIRST,
  LAST,
  CLOSE,
  STRONGEST,
  WEAKEST
}