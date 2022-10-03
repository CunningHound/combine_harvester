use bevy::prelude::*;
use heron::prelude::*;
use std::time;

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
    pub combine: Option<Entity>,
    pub truck: Option<Entity>,
    pub score: i32,
    map: PbrBundle,
    camera: Camera3dBundle,
    light: Option<Entity>,
    pub time_remaining: time::Duration,
}

pub struct ScoreChangeEvent {
    pub amount: i32,
}

const GROUND_HALF_SIZE: i32 = 250;

pub fn create_field(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    field_half_size_x: f32,
    field_half_size_z: f32,
    field_position_x: f32,
    field_position_z: f32,
) {
    let mut x = field_position_x - field_half_size_x as f32;
    loop {
        let mut z = -field_half_size_z as f32;
        loop {
            commands
                .spawn_bundle(SceneBundle {
                    scene: asset_server.load("wheat.gltf#Scene0"),
                    transform: Transform {
                        translation: Vec3::new(x, 0.7, z).into(),
                        ..default()
                    },
                    ..default()
                })
                .insert(harvest::Crop { amount: 1 })
                .insert(RigidBody::Sensor)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3 {
                        x: harvest::CORN_SIZE / 2.5,
                        y: harvest::CORN_SIZE / 2.5,
                        z: harvest::CORN_SIZE / 2.5,
                    },
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::none()
                        .with_group(GameLayer::Crop)
                        .with_masks(&[GameLayer::Combine, GameLayer::Truck]),
                );
            z += harvest::CORN_SIZE;
            if z >= field_half_size_z as f32 {
                break;
            }
        }
        x += CORN_SIZE;
        if x >= field_half_size_x as f32 {
            break;
        }
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
) {
    game.time_remaining = time::Duration::new(150, 0);

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 100., -100.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });

    game.score = 0;
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: (2 * GROUND_HALF_SIZE) as f32,
            })),
            material: materials.add(Color::LIME_GREEN.into()),
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

    const ORTH_PROJECTION_SIZE: f32 = 250.0;
    game.light = Some(
        commands
            .spawn_bundle(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    illuminance: 10000.0,
                    shadow_projection: OrthographicProjection {
                        left: -ORTH_PROJECTION_SIZE,
                        right: ORTH_PROJECTION_SIZE,
                        bottom: -ORTH_PROJECTION_SIZE,
                        top: ORTH_PROJECTION_SIZE,
                        near: -ORTH_PROJECTION_SIZE,
                        far: ORTH_PROJECTION_SIZE,
                        ..default()
                    },
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 10.0, 0.0),
                    rotation: Quat::from_euler(
                        EulerRot::XYZ,
                        std::f32::consts::FRAC_PI_3 * 4.0,
                        -std::f32::consts::FRAC_PI_4,
                        0.,
                    ),
                    ..default()
                },
                ..default()
            })
            .id(),
    );

    create_field(commands, asset_server, 50.0, 50.0, 0.0, 0.0);
}

pub fn update_score(
    mut game: ResMut<Game>,
    mut score_change_events: EventReader<ScoreChangeEvent>,
) {
    for score_change in score_change_events.iter() {
        game.score += score_change.amount;
    }
}

pub fn countdown_timer(
    mut game: ResMut<Game>,
    mut time: ResMut<Time>,
    mut app_state: ResMut<State<GameState>>,
) {
    if game.time_remaining > std::time::Duration::ZERO {
        game.time_remaining = game
            .time_remaining
            .saturating_sub(time::Duration::from_secs_f32(time.delta_seconds()));
        return;
    }

    if *app_state.current() != GameState::GameOver {
        app_state.set(GameState::GameOver).unwrap();
    }
}
