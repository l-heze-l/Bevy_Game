// One of the two settings that can be set through the menu. 
// It will be a resource in the app.

use bevy::prelude::*;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);
