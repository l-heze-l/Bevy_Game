///////////////////////////////////////////////////////////////////////////////
/// Lukas Hezel                                                             ///
/// Spookemon                                                               ///
/// settings.rs                                                             ///
/// 3/27/2025                                                               ///
///////////////////////////////////////////////////////////////////////////////
//  INCLUDES  //

use bevy::prelude::*;

///////////////////////////////////////////////////////////////////////////////

pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
/*
  Enum that will be used as a global state for the game
*/
  #[default]
  Splash,
  Menu,
  Game,
}


#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
/*  
  One of the two settings that can be set through the menu. 
  It will be a resource in the app.
*/  
  Low,
  Medium,
  High,
}


#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);
/*
  One of the two settings that can be set through the menu. 
  It will be a resource in the app
*/

///////////////////////////////////////////////////////////////////////////////