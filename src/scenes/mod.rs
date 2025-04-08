use bevy::prelude::States;

pub mod game;
pub mod splash1;
pub mod splash2;
pub mod menu;


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
  #[default]
  Splash1,
  Splash2,
  Menu,
  Game,
}