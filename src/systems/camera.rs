///////////////////////////////////////////////////////////////////////////////
/// Lukas Hezel                                                             ///
/// Spookemon                                                               ///
/// camera.rs                                                               ///
/// 3/27/2025                                                               ///
///////////////////////////////////////////////////////////////////////////////
//  IMPORTS  //

use bevy::prelude::*;

// use crate::components::player::PlayerComponent;
// use crate::scenes::GameState;


///////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct UserInterfaceCamera;

// #[derive(Component)]
// pub struct Orthographic2DCamera;

pub struct CameraPlugin;


///////////////////////////////////////////////////////////////////////////////

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_user_interface_camera);
    // app.add_systems(Startup, spawn_2d_camera);

    // app.add_systems(Update, camera_follow.run_if(in_state(SceneState::InGameSurvivalMode)));

    // app.add_systems(OnExit(GameState::Game), reset_camera);
  }
}

fn spawn_user_interface_camera(mut commands: Commands) {
  commands
    .spawn(Camera2d::default())
    .insert(Name::new("UserInterfaceCamera"))
    .insert(UserInterfaceCamera);
}

// fn spawn_2d_camera(mut commands: Commands) {
//   let mut camera: Camera2d = Camera2d::default();

//   commands
//     .spawn(camera)
//     .insert(Orthographic2DCamera)
//     .insert(Name::new("Orthographic2DCamera"));
// }


// fn camera_follow(
//   player_query: Query<&Transform, With<PlayerComponent>>,
//   mut camera_query: Query<&mut Transform, (Without<PlayerComponent>, With<Orthographic2DCamera>)>,
// ) {
//   let player_transform = player_query.single();
//   let mut camera_transform = camera_query.single_mut();

//   camera_transform.translation.x = player_transform.translation.x;
//   camera_transform.translation.y = player_transform.translation.y;
// }

// fn reset_camera(mut camera_query: Query<&mut Transform, With<Orthographic2DCamera>>) {
//   let mut camera_transform = camera_query.single_mut();
//   camera_transform.translation.x = 0.0;
//   camera_transform.translation.y = 0.0;
// }

///////////////////////////////////////////////////////////////////////////////