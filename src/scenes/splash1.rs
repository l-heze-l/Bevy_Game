///////////////////////////////////////////////////////////////////////////////
/// Lukas Hezel                                                             ///
/// Spookemon                                                               ///
/// splash1.rs                                                              ///
/// 3/27/2025                                                               ///
///////////////////////////////////////////////////////////////////////////////
//  INCLUDES  //

use crate::scenes::GameState;
use crate::systems::despawn_screen::despawn_screen;
use crate::resources::colors::*;

use bevy::{
  prelude::*,
  input::ButtonInput,
};


///////////////////////////////////////////////////////////////////////////////
//  DECLARATIONS  //

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;


///////////////////////////////////////////////////////////////////////////////
//  MAIN PLUGIN  //

pub fn splash1_plugin(app: &mut App) {
  // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
  app
    // When entering the state, spawn everything needed for this screen
    .add_systems(OnEnter(GameState::Splash1), splash_setup)
    // While in this state, run the `countdown` system
    .add_systems(Update, await_input.run_if(in_state(GameState::Splash1)))
    // When exiting the state, despawn everything that was spawned for this screen
    .add_systems(OnExit(GameState::Splash1), despawn_screen::<OnSplashScreen>);
}


///////////////////////////////////////////////////////////////////////////////
//  SUPPORTING FUNCTIONS  //

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let handfish_icon: Handle<Image> = asset_server.load("textures/Branding/handfish.png");
  // Display the logo
  commands.spawn((
    Node {
      align_items: AlignItems::Center,
      justify_content: JustifyContent::Center,
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      flex_direction: FlexDirection::Column, 
      ..default()
    },  
    OnSplashScreen,
  )).with_children(|parent| {
    parent.spawn((
      Text::new("A Handfish Production"),
      TextFont {
        font_size: 20.0,
        ..default()
      },
      TextColor(TEXT_1.into()),
      Node {
        margin: UiRect::all(Val::Px(50.0)),
        ..default()
      },
    ));
    parent.spawn((
      ImageNode::new(handfish_icon),
      Node {
        width: Val::Px(200.0),
        ..default()
      },
    ));
    parent.spawn((
      Text::new("Press SPACE to Continue..."),
      TextFont {
        font_size: 20.0,
        ..default()
      },
      TextColor(TEXT_1.into()),
      Node {
        margin: UiRect::all(Val::Px(50.0)),
        ..default()
      },
    ));
  });
}


// Tick the timer, and change state when finished
fn await_input(
  mut game_state: ResMut<NextState<GameState>>,
  keyboard_input: Res<ButtonInput<KeyCode>>,
) {
  if keyboard_input.just_pressed(KeyCode::Space) {
    game_state.set(GameState::Splash2);
  }
}


///////////////////////////////////////////////////////////////////////////////