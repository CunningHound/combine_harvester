use bevy::prelude::*;

use crate::vehicles;
use crate::{game, RigidBody};

#[derive(Component)]
pub struct ScoreText {}

#[derive(Component)]
pub struct StorageText {}

#[derive(Component)]
pub struct TimerText {}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Res<game::Game>,
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

    commands
        .spawn_bundle(TextBundle::from_sections([
            TextSection::new(
                "Time left: ",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 40.0,
                    // Alpha channel of the color controls transparency.
                    color: Color::rgba(1.0, 1.0, 1.0, 0.2),
                },
            ),
            TextSection::new(
                format!(
                    "{}:{}",
                    game.time_remaining.as_secs() / 60,
                    game.time_remaining.as_secs() % 60
                ),
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 40.0,
                    color: Color::rgba(1., 1., 1., 0.2),
                },
            ),
        ]))
        .insert(TimerText {});
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

pub fn update_time(game: Res<game::Game>, mut query: Query<&mut Text, With<TimerText>>) {
    if let Ok(mut text) = query.get_single_mut() {
        let remaining = game.time_remaining.as_secs();
        if remaining % 60 < 10 {
            text.sections[1].value = format!(
                "{}:0{}",
                game.time_remaining.as_secs() / 60,
                game.time_remaining.as_secs() % 60
            );
        } else {
            text.sections[1].value = format!(
                "{}:{}",
                game.time_remaining.as_secs() / 60,
                game.time_remaining.as_secs() % 60
            );
        }

        if remaining < 10 {
            text.sections[1].style.font_size = 40. + ((10 - remaining) as f32 * 2.);
        }
    }
}

pub fn display_final_score(
    mut commands: Commands,
    game: Res<game::Game>,
    mut score_text: Query<(Entity, &Text), With<ScoreText>>,
    mut storage_text: Query<(Entity, &Text), With<StorageText>>,
    mut timer_text: Query<(Entity, &TimerText), With<TimerText>>,
    mut asset_server: ResMut<AssetServer>,
) {
    if let Ok((entity, text)) = score_text.get_single_mut() {
        commands.entity(entity).despawn();
    }
    if let Ok((entity, storage)) = storage_text.get_single_mut() {
        commands.entity(entity).despawn();
    }
    if let Ok((entity, timer_text)) = timer_text.get_single_mut() {
        commands.entity(entity).despawn();
    }

    let font_handle = asset_server.load("fonts/abel-regular.ttf");

    commands.spawn_bundle(TextBundle::from_sections([
        TextSection::new(
            "Game Over\nfinal score: ",
            TextStyle {
                font: font_handle.clone(),
                font_size: 40.0,
                // Alpha channel of the color controls transparency.
                color: Color::rgba(1.0, 1.0, 1.0, 0.2),
            },
        ),
        TextSection::new(
            format!("{}", game.score),
            TextStyle {
                font: font_handle.clone(),
                font_size: 40.0,
                color: Color::rgba(1., 1., 1., 0.2),
            },
        ),
    ]));
}
