use bevy::prelude::*;
use heron::prelude::*;

use crate::harvest;
use crate::harvest::CORN_SIZE;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    GameOver,
}

#[derive(PhysicsLayer)]
pub enum GameLayer {
    Combine,
    Truck,
    Vehicle,
    Crop,
    Obstacle,
    World,
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

const GROUND_HALF_SIZE: i32 = 250;
const FIELD_HALF_SIZE: i32 = 50;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
) {
    game.size = (2 * FIELD_HALF_SIZE, 2 * FIELD_HALF_SIZE);

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 150., -75.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });

    game.score = 0;
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: (2 * GROUND_HALF_SIZE) as f32,
            })),
            material: materials.add(Color::SEA_GREEN.into()),
            transform: Transform {
                translation: Vec3::ZERO.into(),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3 {
                x: GROUND_HALF_SIZE as f32,
                y: 0.1,
                z: GROUND_HALF_SIZE as f32,
            },
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(GameLayer::World)
                .with_mask(GameLayer::Vehicle),
        );

    let mut x = -FIELD_HALF_SIZE as f32;
    loop {
        let mut z = -FIELD_HALF_SIZE as f32;
        loop {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube {
                        size: harvest::CORN_SIZE * harvest::CORN_SIZE_FILL_FRACTION,
                    })),
                    material: materials.add(Color::YELLOW.into()),
                    transform: Transform {
                        translation: Vec3::new(x, 0.1, z).into(),
                        ..default()
                    },
                    ..default()
                })
                .insert(harvest::Crop { amount: 1 })
                .insert(RigidBody::Sensor)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3 {
                        x: harvest::CORN_SIZE / 2.,
                        y: harvest::CORN_SIZE / 2.,
                        z: harvest::CORN_SIZE / 2.,
                    },
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::none()
                        .with_group(GameLayer::Crop)
                        .with_masks(&[GameLayer::Combine, GameLayer::Truck]),
                );
            z += harvest::CORN_SIZE;
            if z >= FIELD_HALF_SIZE as f32 {
                break;
            }
        }
        x += CORN_SIZE;
        if x >= FIELD_HALF_SIZE as f32 {
            break;
        }
    }

    game.light = Some(
        commands
            .spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(
                    FIELD_HALF_SIZE as f32 / 2.,
                    100.,
                    -FIELD_HALF_SIZE as f32 / 2.,
                ),
                point_light: PointLight {
                    color: Color::rgb(0.9, 0.9, 0.9).into(),
                    intensity: 120000.0,
                    shadows_enabled: true,
                    range: 500.,
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
        println!("score is now {:?} ", game.score);
    }
}
