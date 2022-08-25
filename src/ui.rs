use bevy::prelude::*;

use crate::{game, RigidBody};

#[derive(Component)]
pub struct ScoreText {}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
}

pub fn update_ui_score(game: Res<game::Game>, mut query: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[1].value = format!("{}", game.score);
    } else {
        println!("getting the thing failed :(")
    }
}
