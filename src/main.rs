///////////////////////////////////////////////////////////////////////////////
/// Lukas Hezel                                                             ///
/// Spookemon                                                               ///
/// main.rs                                                                 ///
/// 3/27/2025                                                               ///
///////////////////////////////////////////////////////////////////////////////
//  INCLUDES  //

use bevy::{
  prelude::*, 
  window::WindowMode
};

use bevy::window::{
  WindowResolution, 
  WindowResizeConstraints
};

use config::*;
use scenes::GameState;

mod config;
mod resources;
mod objects;
mod scenes;
mod systems;

///////////////////////////////////////////////////////////////////////////////
//  MAIN  //

fn main() { App::new()

  // Add the generic plugins
  .add_plugins( DefaultPlugins
    .set( WindowPlugin{
        primary_window: Some( Window{
          resolution: WindowResolution::new(
            WINDOW_HEIGHT * RESOLUTION, 
            WINDOW_HEIGHT
          ),
          title: TITLE.to_string(),
          position: WindowPosition::At( IVec2::new(
            MONITOR_WIDTH / 4, 
            MONITOR_HEIGHT / 4
          )),
          resizable: false,
          resize_constraints: WindowResizeConstraints{
            min_width: WINDOW_HEIGHT * RESOLUTION,
            max_width: WINDOW_HEIGHT * RESOLUTION,
            min_height: WINDOW_HEIGHT,
            max_height: WINDOW_HEIGHT,
          },
          mode: WindowMode::Windowed,
          ..default()
        }),
        ..default()
    })
    .set(
      ImagePlugin::default_nearest()
    )
  )

  // Insert as resource the initial value for the settings resources
  .insert_resource(resources::display_quality::DisplayQuality::Medium)
  .insert_resource(resources::volume::Volume(7))

  // Declare the game state, whose starting value is determined by the `Default` trait
  .init_state::<GameState>()
  .add_plugins(systems::camera::CameraPlugin)
  
  // Adds the plugins for each state
  .add_plugins(scenes::splash1::splash1_plugin)
  .add_plugins(scenes::splash2::splash2_plugin) 
  .add_plugins(scenes::menu::menu_plugin)
  .add_plugins(scenes::game::game_plugin)

  // Start the program
  .run();
}

///////////////////////////////////////////////////////////////////////////////
