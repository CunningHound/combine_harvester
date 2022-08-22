mod vehicles;
mod game;

use bevy::prelude::*;
use heron::prelude::*;


fn main() {
    App::new()
        .init_resource::<game::Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_state(game::GameState::Playing)
        .add_startup_system(setup_camera)
        .add_system_set(SystemSet::on_enter(game::GameState::Playing).with_system(game::setup))
        .add_system_set(SystemSet::on_enter(game::GameState::Playing).with_system(vehicles::setup))
        .add_system_set(
            SystemSet::on_update(game::GameState::Playing)
                .with_system(vehicles::move_combine)
                .with_system(vehicles::move_truck)
        )
        .run();
}



fn setup_camera( mut commands: Commands, )
{
        commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 150., -20.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}