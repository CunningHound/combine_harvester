use bevy::prelude::*;
use heron::prelude::*;

use crate::harvest;
use crate::harvest::CropHarvestedEvent;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    GameOver,
}

#[derive(PhysicsLayer)]
pub enum GameLayer {
    Combine,
    Truck,
    Crop,
    Obstacle,
}

#[derive(Default)]
pub struct Game {
    size: (i32, i32),
    pub combine: Option<Entity>,
    pub truck: Option<Entity>,
    score: i32,
    map: PbrBundle,
    camera: Camera3dBundle,
    light: Option<Entity>,
}

pub struct ScoreChangeEvent {
    pub amount: i32,
}

const MAP_HALF_SIZE: i32 = 50;
const CORN_SIZE: f32 = 1.0;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
) {
    game.size = (2 * MAP_HALF_SIZE, 2 * MAP_HALF_SIZE);

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 150., -20.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });

    game.score = 0;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: (2 * MAP_HALF_SIZE) as f32,
        })),
        material: materials.add(Color::SEA_GREEN.into()),
        transform: Transform {
            translation: Vec3::ZERO.into(),
            ..default()
        },
        ..default()
    });

    let mut x = -MAP_HALF_SIZE as f32;
    loop {
        let mut z = -MAP_HALF_SIZE as f32;
        loop {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube {
                        size: CORN_SIZE * 0.98,
                    })),
                    material: materials.add(Color::YELLOW.into()),
                    transform: Transform {
                        translation: Vec3::new(x, 0.1, z).into(),
                        ..default()
                    },
                    ..default()
                })
                .insert(harvest::Crop {
                    amount: 1,
                    value: 1,
                })
                .insert(RigidBody::Sensor)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3 {
                        x: CORN_SIZE / 2.,
                        y: CORN_SIZE / 2.,
                        z: CORN_SIZE / 2.,
                    },
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::none()
                        .with_group(GameLayer::Crop)
                        .with_masks(&[GameLayer::Combine, GameLayer::Truck]),
                );
            z += CORN_SIZE;
            if z >= MAP_HALF_SIZE as f32 {
                break;
            }
        }
        x += CORN_SIZE;
        if x >= MAP_HALF_SIZE as f32 {
            break;
        }
    }

    game.light = Some(
        commands
            .spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(
                    MAP_HALF_SIZE as f32 / 2.,
                    100.,
                    MAP_HALF_SIZE as f32 / 2.,
                ),
                point_light: PointLight {
                    color: Color::rgb(0.9, 0.9, 0.9).into(),
                    intensity: 70000.0,
                    shadows_enabled: true,
                    range: 300.,
                    ..default()
                },
                ..default()
            })
            .id(),
    );
}

pub fn update_score(
    mut game: ResMut<Game>,
    mut score_change_events: EventReader<ScoreChangeEvent>,
) {
    for score_change in score_change_events.iter() {
        game.score += score_change.amount;
    }
    println!("score is now {:?}", game.score);
}
