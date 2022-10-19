mod animals;
mod drop_zone;
mod game;
mod harvest;
mod ui;
mod vehicles;

use bevy::prelude::*;
use heron::prelude::*;

fn main() {
    App::new()
        .init_resource::<game::Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))
        .add_state(game::GameState::Playing)
        .add_event::<harvest::CropHarvestedEvent>()
        .add_event::<harvest::CropSquashedEvent>()
        .add_event::<game::ScoreChangeEvent>()
        .insert_resource(vehicles::CombineStorage {
            capacity: 500,
            contents: 0,
        })
        .insert_resource(vehicles::TruckStorage {
            capacity: 1500,
            contents: 0,
        })
        .add_system_set(SystemSet::on_enter(game::GameState::Playing).with_system(game::setup))
        .add_system_set(SystemSet::on_enter(game::GameState::Playing).with_system(vehicles::setup))
        .add_system_set(SystemSet::on_enter(game::GameState::Playing).with_system(drop_zone::setup))
        .add_system_set(SystemSet::on_enter(game::GameState::Playing).with_system(ui::setup))
        .add_system_set(SystemSet::on_enter(game::GameState::Playing).with_system(animals::setup))
        .add_system_set(
            SystemSet::on_update(game::GameState::Playing)
                .with_system(vehicles::move_combine)
                .with_system(vehicles::move_truck)
                .with_system(vehicles::combine_collision_check)
                .with_system(vehicles::truck_collision_check)
                .with_system(vehicles::transfer_harvest),
        )
        .add_system_set(
            SystemSet::on_update(game::GameState::Playing)
                .with_system(harvest::crop_events_handler),
        )
        .add_system_set(
            SystemSet::on_update(game::GameState::Playing)
                .with_system(drop_zone::drop_zone_update)
                .with_system(drop_zone::drop_zone_accept),
        )
        .add_system_set(
            SystemSet::on_update(game::GameState::Playing).with_system(animals::move_animals),
        )
        .add_system_set(
            SystemSet::on_update(game::GameState::Playing)
                .with_system(ui::update_ui_score)
                .with_system(ui::update_contents)
                .with_system(ui::update_time),
        )
        .add_system_set(
            SystemSet::on_update(game::GameState::Playing)
                .with_system(game::update_score)
                .with_system(game::countdown_timer),
        )
        .add_system_set(
            SystemSet::on_enter(game::GameState::GameOver).with_system(ui::display_final_score),
        )
        .run();
}
