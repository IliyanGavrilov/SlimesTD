use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
  #[default]
  AssetLoading,
  MainMenu,
  Gameplay
}