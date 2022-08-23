mod game;
mod harvest;
mod vehicles;

use bevy::prelude::*;
use heron::prelude::*;

fn main() {
    App::new()
        .init_resource::<game::Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_state(game::GameState::Playing)
        .add_event::<harvest::CropHarvestedEvent>()
        .add_event::<harvest::CropSquashedEvent>()
        .add_event::<game::ScoreChangeEvent>()
        .add_system_set(SystemSet::on_enter(game::GameState::Playing).with_system(game::setup))
        .add_system_set(SystemSet::on_enter(game::GameState::Playing).with_system(vehicles::setup))
        .add_system_set(
            SystemSet::on_update(game::GameState::Playing)
                .with_system(vehicles::move_combine)
                .with_system(vehicles::move_truck)
                .with_system(vehicles::combine_collision_check)
                .with_system(vehicles::truck_collision_check)
                .with_system(vehicles::harvest_event_handler),
        )
        .add_system_set(
            SystemSet::on_update(game::GameState::Playing)
                .with_system(harvest::crop_events_handler)
                .after(vehicles::harvest_event_handler),
        )
        .add_system(game::update_score)
        .run();
}
