use bevy::prelude::*;
use heron::prelude::*;
use std::f32::consts::FRAC_PI_2;
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
const FIELD_BORDER: f32 = 2.;
const FENCE_SIZE: f32 = 2.;

fn spawn_fence(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    x: f32,
    z: f32,
    rotation: f32,
) {
    commands
        .spawn_bundle(SceneBundle {
            scene: asset_server.load("fence.gltf#Scene0"),
            transform: Transform {
                translation: Vec3::new(x, 0.7, z).into(),
                rotation: Quat::from_rotation_y(rotation),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3 {
                x: 2.0,
                y: 1.0,
                z: 0.2,
            },
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(GameLayer::World)
                .with_masks(&[GameLayer::Combine, GameLayer::Truck]),
        );
}

fn create_fences(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    field_half_size_x: f32,
    field_half_size_z: f32,
    field_position_x: f32,
    field_position_z: f32,
) {
    let edge_n = field_position_x + field_half_size_x + FIELD_BORDER;
    let edge_s = field_position_x - field_half_size_x - FIELD_BORDER;
    let edge_e = field_position_z + field_half_size_z + FIELD_BORDER;
    let edge_w = field_position_z - field_half_size_z - FIELD_BORDER;

    let mut x = edge_w;
    loop {
        spawn_fence(
            commands,
            asset_server,
            x + FENCE_SIZE / 2.,
            edge_n,
            FRAC_PI_2,
        );
        spawn_fence(
            commands,
            asset_server,
            x + FENCE_SIZE / 2.,
            edge_s,
            FRAC_PI_2,
        );
        x += FENCE_SIZE;
        if x >= edge_e {
            break;
        }
    }
    let mut z = edge_s;
    loop {
        spawn_fence(commands, asset_server, edge_e, z + FENCE_SIZE / 2., 0.);
        spawn_fence(commands, asset_server, edge_w, z + FENCE_SIZE / 2., 0.);
        z += FENCE_SIZE;
        if z >= edge_n {
            break;
        }
    }
}

fn create_field(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    field_half_size_x: f32,
    field_half_size_z: f32,
    field_position_x: f32,
    field_position_z: f32,
) {
    create_fences(
        commands,
        asset_server,
        field_half_size_x,
        field_half_size_z,
        field_position_x,
        field_position_z,
    );
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
            material: materials.add(StandardMaterial {
                base_color: Color::LIME_GREEN.into(),
                reflectance: 0.5,
                metallic: 0.1,
                ..default()
            }),
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

    create_field(&mut commands, &asset_server, 50.0, 50.0, 0.0, 0.0);
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
