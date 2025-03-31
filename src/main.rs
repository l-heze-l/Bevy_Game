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

use crate::systems::menu::menu;

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

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d);
}

mod splash {
  use bevy::prelude::*;

  use super::{despawn_screen, GameState};

  // This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
  pub fn splash_plugin(app: &mut App) {
    // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
    app
      // When entering the state, spawn everything needed for this screen
      .add_systems(OnEnter(GameState::Splash), splash_setup)
      // While in this state, run the `countdown` system
      .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
      // When exiting the state, despawn everything that was spawned for this screen
      .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
  }

  // Tag component used to tag entities added on the splash screen
  #[derive(Component)]
  struct OnSplashScreen;

  // Newtype to use a `Timer` for this screen as a resource
  #[derive(Resource, Deref, DerefMut)]
  struct SplashTimer(Timer);

  fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon: Handle<Image> = asset_server.load("textures/icon.png");
    // Display the logo
    commands
      .spawn((
        Node {
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          ..default()
        },
        OnSplashScreen,
      ))
      .with_children(|parent| {
        parent.spawn((
          ImageNode::new(icon),
          Node {
            // This will set the logo to be 200px wide, and auto adjust its height
            width: Val::Px(200.0),
            ..default()
          },
        ));
      });
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
  }

  // Tick the timer, and change state when finished
  fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
  ) {
    if timer.tick(time.delta()).finished() {
      game_state.set(GameState::Menu);
    }
  }
}

mod game {
  use bevy::{
    color::palettes::basic::{BLUE, LIME},
    prelude::*,
  };

  use super::{despawn_screen, DisplayQuality, GameState, Volume, TEXT_COLOR};

  // This plugin will contain the game. In this case, it's just be a screen that will
  // display the current settings for 5 seconds before returning to the menu
  pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), game_setup)
      .add_systems(Update, game.run_if(in_state(GameState::Game)))
      .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
  }

  // Tag component used to tag entities added on the game screen
  #[derive(Component)]
  struct OnGameScreen;

  #[derive(Resource, Deref, DerefMut)]
  struct GameTimer(Timer);

  fn game_setup(
    mut commands: Commands,
    display_quality: Res<DisplayQuality>,
    volume: Res<Volume>,
  ) {
    commands
      .spawn((
        Node {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          // center children
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          ..default()
        },
        OnGameScreen,
      ))
      .with_children(|parent| {
        // First create a `Node` for centering what we want to display
        parent
          .spawn((
            Node {
              // This will display its children in a column, from top to bottom
              flex_direction: FlexDirection::Column,
              // `align_items` will align children on the cross axis. Here the main axis is
              // vertical (column), so the cross axis is horizontal. This will center the
              // children
              align_items: AlignItems::Center,
              ..default()
            },
            BackgroundColor(Color::BLACK),
          ))
          .with_children(|p| {
            p.spawn((
              Text::new("Will be back to the menu shortly..."),
              TextFont {
                font_size: 67.0,
                ..default()
              },
              TextColor(TEXT_COLOR),
              Node {
                margin: UiRect::all(Val::Px(50.0)),
                ..default()
              },
            ));
            p.spawn((
              Text::default(),
              Node {
                margin: UiRect::all(Val::Px(50.0)),
                ..default()
              },
            ))
            .with_children(|p| {
              p.spawn((
                TextSpan(format!("quality: {:?}", *display_quality)),
                TextFont {
                  font_size: 50.0,
                  ..default()
                },
                TextColor(BLUE.into()),
              ));
              p.spawn((
                TextSpan::new(" - "),
                TextFont {
                  font_size: 50.0,
                  ..default()
                },
                TextColor(TEXT_COLOR),
              ));
              p.spawn((
                TextSpan(format!("volume: {:?}", *volume)),
                TextFont {
                  font_size: 50.0,
                  ..default()
                },
                TextColor(LIME.into()),
              ));
            });
          });
      });
    // Spawn a 5 seconds timer to trigger going back to the menu
    commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
  }

  // Tick the timer, and change state when finished
  fn game(
    time: Res<Time>,
    mut game_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<GameTimer>,
  ) {
    if timer.tick(time.delta()).finished() {
      game_state.set(GameState::Menu);
    }
  }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
  for entity in &to_despawn {
    commands.entity(entity).despawn_recursive();
  }
}