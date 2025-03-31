///////////////////////////////////////////////////////////////////////////////
/// Lukas Hezel                                                             ///
/// Spookemon                                                               ///
/// main.rs                                                                 ///
/// 3/27/2025                                                               ///
///////////////////////////////////////////////////////////////////////////////
//  INCLUDES  //

use bevy::prelude::*;

mod settings;
use crate::settings::*;

mod systems;
use crate::systems::menu::menu;
use crate::systems::splash::splash;
use crate::systems::game::game;

mod utility;
use crate::utility::*;

///////////////////////////////////////////////////////////////////////////////
//  MAIN  //

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)

    // Insert as resource the initial value for the settings resources
    .insert_resource(DisplayQuality::Medium)
    .insert_resource(Volume(7))

    // Declare the game state, whose starting value is determined by the `Default` trait
    .init_state::<GameState>()
    .add_systems(Startup, setup)
    
    // Adds the plugins for each state
    .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
    .run();
}

///////////////////////////////////////////////////////////////////////////////
