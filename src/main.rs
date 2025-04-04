///////////////////////////////////////////////////////////////////////////////
/// Lukas Hezel                                                             ///
/// Spookemon                                                               ///
/// main.rs                                                                 ///
/// 3/27/2025                                                               ///
///////////////////////////////////////////////////////////////////////////////
//  INCLUDES  //

use bevy::prelude::*;

mod resources;
mod objects;
mod scenes;
mod systems;

///////////////////////////////////////////////////////////////////////////////
//  MAIN  //

fn main() {
  App::new()
    // Add the generic plugins
    .add_plugins(DefaultPlugins)

    // Insert as resource the initial value for the settings resources
    .insert_resource(resources::display_quality::DisplayQuality::Medium)
    .insert_resource(resources::volume::Volume(7))

    // Declare the game state, whose starting value is determined by the `Default` trait
    .init_state::<resources::game_state::GameState>()
    .add_systems(Startup, systems::setup::setup)
    
    // Adds the plugins for each state
    .add_plugins(scenes::splash::splash_plugin)
    .add_plugins(scenes::menu::menu_plugin)
    .add_plugins(scenes::game::game_plugin)

    // Start the program
    .run();
}

///////////////////////////////////////////////////////////////////////////////
