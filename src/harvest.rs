use bevy::prelude::*;

#[derive(Component)]
pub struct Crop {
    pub amount: i32,
    pub value: i32,
}
