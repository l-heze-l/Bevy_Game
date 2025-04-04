// Enum that will be used as a global state for the game

use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
  #[default]
  Splash,
  Menu,
  Game,
}