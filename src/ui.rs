use bevy::prelude::*;

use crate::vehicles;
use crate::{game, RigidBody};

#[derive(Component)]
pub struct ScoreText {}

#[derive(Component)]
pub struct StorageText {}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    combine_storage: Res<vehicles::CombineStorage>,
    truck_storage: Res<vehicles::TruckStorage>,
) {
    let font_handle = asset_server.load("fonts/abel-regular.ttf");

    commands
        .spawn_bundle(TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 40.0,
                    // Alpha channel of the color controls transparency.
                    color: Color::rgba(1.0, 1.0, 1.0, 0.2),
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 40.0,
                    color: Color::rgba(1., 1., 1., 0.2),
                },
            ),
        ]))
        .insert(ScoreText {});

    commands
        .spawn_bundle(TextBundle::from_sections([
            TextSection::new(
                "Combine: ",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 30.0,
                    color: Color::rgba(1.0, 1.0, 1.0, 0.2),
                },
            ),
            TextSection::new(
                format!("0/{}", combine_storage.capacity),
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 40.0,
                    color: Color::rgba(1., 1., 1., 0.2),
                },
            ),
            TextSection::new(
                "\nTruck: ",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 30.0,
                    color: Color::rgba(1.0, 1.0, 1.0, 0.2),
                },
            ),
            TextSection::new(
                format!("0/{}", truck_storage.capacity),
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 40.0,
                    color: Color::rgba(1., 1., 1., 0.2),
                },
            ),
        ]))
        .insert(StorageText {});
}

pub fn update_ui_score(game: Res<game::Game>, mut query: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[1].value = format!("{}", game.score);
    } else {
        println!("getting the thing failed :(")
    }
}

pub fn update_contents(
    game: Res<game::Game>,
    combine_storage: Res<vehicles::CombineStorage>,
    truck_storage: Res<vehicles::TruckStorage>,
    mut query: Query<&mut Text, With<StorageText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[1].value =
            format!("{}/{}", combine_storage.contents, combine_storage.capacity);
        text.sections[3].value = format!("{}/{}", truck_storage.contents, truck_storage.capacity);
    }
}
