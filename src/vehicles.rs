use crate::game;
use crate::harvest;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, PI};

use bevy::gltf::GltfMesh;
use bevy::prelude::*;
use heron::prelude::*;

#[derive(Default)]
pub struct Vehicle {
    pub drive_speed: f32,
    pub turn_rate: f32,
    pub acceleration: f32,
    pub max_reverse: f32,
}

#[derive(Component)]
pub struct Combine {
    pub vehicle: Vehicle,
    pub transfer_speed: f32,
}

#[derive(Component)]
pub struct CombineStorage {
    pub capacity: i32,
    pub contents: i32,
}

#[derive(Component)]
pub struct Truck {
    pub vehicle: Vehicle,
    pub dump_speed: f32,
}

#[derive(Component)]
pub struct TruckStorage {
    pub capacity: i32,
    pub contents: i32,
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<game::Game>,
    asset_server: Res<AssetServer>,
) {
    let combine_id = commands
        .spawn_bundle(SceneBundle {
            scene: asset_server.load("combine.gltf#Scene0"),
            transform: Transform {
                translation: Vec3::new(60., 1.6, 0.).into(),
                ..default()
            },
            ..default()
        })
        .insert(Combine {
            vehicle: Vehicle {
                drive_speed: 10.,
                turn_rate: 4.,
                acceleration: 20.,
                max_reverse: 5.,
            },
            transfer_speed: 10.,
        })
        .insert(
            CollisionLayers::none()
                .with_groups(&[game::GameLayer::Combine, game::GameLayer::Vehicle])
                .with_masks(&[
                    game::GameLayer::Crop,
                    game::GameLayer::Obstacle,
                    game::GameLayer::Vehicle,
                    game::GameLayer::World,
                ]),
        )
        .insert(RigidBody::Dynamic)
        .insert(Velocity::from_linear(Vec3::ZERO).with_angular(AxisAngle::new(Vec3::Y, 0.)))
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3 {
                x: 2.,
                y: 0.3,
                z: 2.,
            },
            border_radius: Some(0.2),
        })
        .id();

    game.combine = Some(combine_id);

    let truck_id = commands
        .spawn_bundle(SceneBundle {
            scene: asset_server.load("truck.gltf#Scene0"),
            transform: Transform {
                translation: Vec3::new(70., 1.6, 0.).into(),
                ..default()
            },
            ..default()
        })
        .insert(Truck {
            vehicle: Vehicle {
                drive_speed: 15.,
                turn_rate: 3.,
                acceleration: 40.,
                max_reverse: 5.,
            },
            dump_speed: 20.,
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::from_linear(Vec3::ZERO).with_angular(AxisAngle::new(Vec3::Y, 0.)))
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3 {
                x: 1.5,
                y: 0.3,
                z: 1.,
            },
            border_radius: Some(0.2),
        })
        .insert(
            CollisionLayers::none()
                .with_groups(&[game::GameLayer::Truck, game::GameLayer::Vehicle])
                .with_masks(&[
                    game::GameLayer::Crop,
                    game::GameLayer::Obstacle,
                    game::GameLayer::Vehicle,
                    game::GameLayer::World,
                ]),
        )
        .insert(Collisions::default())
        .id();

    game.truck = Some(truck_id);
}

pub fn combine_collision_check(
    mut collisions: EventReader<CollisionEvent>,
    mut crop_harvested_events: EventWriter<harvest::CropHarvestedEvent>,
) {
    for collision in collisions.iter() {
        match collision {
            CollisionEvent::Started(data1, data2) => {
                if data1
                    .collision_layers()
                    .contains_group(game::GameLayer::Combine)
                {
                    crop_harvested_events.send(harvest::CropHarvestedEvent {
                        entity: data2.rigid_body_entity(),
                    });
                } else if data2
                    .collision_layers()
                    .contains_group(game::GameLayer::Combine)
                {
                    crop_harvested_events.send(harvest::CropHarvestedEvent {
                        entity: data1.rigid_body_entity(),
                    });
                }
            }
            _ => {}
        }
    }
}

pub fn truck_collision_check(
    mut collisions: EventReader<CollisionEvent>,
    mut crop_squashed_events: EventWriter<harvest::CropSquashedEvent>,
) {
    for collision in collisions.iter() {
        match collision {
            CollisionEvent::Started(data1, data2) => {
                if data1
                    .collision_layers()
                    .contains_group(game::GameLayer::Truck)
                {
                    crop_squashed_events.send(harvest::CropSquashedEvent {
                        entity: data2.rigid_body_entity(),
                    })
                } else if data2
                    .collision_layers()
                    .contains_group(game::GameLayer::Truck)
                {
                    crop_squashed_events.send(harvest::CropSquashedEvent {
                        entity: data1.rigid_body_entity(),
                    })
                }
            }
            _ => {}
        }
    }
}

pub fn move_combine(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Combine, &mut Transform, &mut Velocity)>,
    time: Res<Time>,
) {
    // there's always exactly one but I didn't understand resources when I wrote this
    let mut requested_direction = Vec2::new(0., 0.);
    let (combine, mut transform, mut velocity) = query.single_mut();
    if keyboard_input.pressed(KeyCode::W) {
        requested_direction.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::A) {
        requested_direction.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::S) {
        requested_direction.y -= 1.;
    }
    if keyboard_input.pressed(KeyCode::D) {
        requested_direction.x -= 1.;
    }

    update_vehicle(
        &combine.vehicle,
        requested_direction,
        &mut transform,
        &mut velocity,
        &time,
    );
}

pub fn move_truck(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Truck, &mut Transform, &mut Velocity)>,
    time: Res<Time>,
) {
    let (truck, mut transform, mut velocity) = query.single_mut();
    let mut requested_direction = Vec2::new(0., 0.);
    if keyboard_input.pressed(KeyCode::Up) {
        requested_direction.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        requested_direction.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        requested_direction.y -= 1.;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        requested_direction.x -= 1.;
    }
    update_vehicle(
        &truck.vehicle,
        requested_direction,
        &mut transform,
        &mut velocity,
        &time,
    );
}

fn update_vehicle(
    vehicle: &Vehicle,
    requested_direction: Vec2,
    transform: &mut Transform,
    mut velocity: &mut Velocity,
    time: &Res<Time>,
) {
    let mut velocity_on_plane = Vec2::new(velocity.linear.x, velocity.linear.z);
    let current_speed = velocity_on_plane.length();
    let mut speed = current_speed;
    let max_change_in_speed = vehicle.acceleration * time.delta_seconds();

    if (requested_direction.length() == 0.) {
        if current_speed >= 0. {
            speed = f32::max((current_speed - max_change_in_speed), 0.);
        } else {
            speed = f32::min(current_speed + max_change_in_speed, 0.);
        }
    } else {
        let mut xz_velocity = velocity.linear;
        xz_velocity.y = 0.;

        let forward_on_plane = Vec2::new(transform.forward().x, transform.forward().z);
        let mut requested_turn_angle = requested_direction.angle_between(forward_on_plane);
        let reverse = requested_turn_angle.abs() > (4. * FRAC_PI_6);

        speed += vehicle.acceleration * time.delta_seconds();

        if reverse {
            speed = -1. * speed;
            if requested_turn_angle > 0. {
                requested_turn_angle -= PI;
            } else {
                requested_turn_angle += PI;
            }
        }

        let max_angle_to_turn = vehicle.turn_rate * time.delta_seconds();
        let mut angle_to_turn = requested_turn_angle.clamp(-max_angle_to_turn, max_angle_to_turn);

        speed = speed.clamp(-vehicle.max_reverse, vehicle.drive_speed);

        transform.rotate_y(angle_to_turn);
    }
    velocity.linear.x = speed * transform.forward().x;
    velocity.linear.z = speed * transform.forward().z;
}

pub fn transfer_harvest(
    mut combines: Query<(&Combine, &Transform)>,
    mut trucks: Query<(&Truck, &Transform)>,
    mut combine_storage: ResMut<CombineStorage>,
    mut truck_storage: ResMut<TruckStorage>,
) {
    if let Ok((combine, combine_transform)) = combines.get_single_mut() {
        if let Ok((truck, truck_transform)) = trucks.get_single_mut() {
            if combine_transform
                .translation
                .distance(truck_transform.translation)
                < 15.
            {
                let to_transfer = i32::min(
                    (truck_storage.capacity - truck_storage.contents),
                    combine_storage.contents,
                );
                truck_storage.contents += to_transfer;
                combine_storage.contents -= to_transfer;
            }
        }
    }
}
