use crate::game;
use crate::harvest;

use bevy::prelude::*;
use heron::prelude::*;

#[derive(Default)]
pub struct Vehicle {
    drive_speed: f32,
    turn_rate: f32,
    acceleration: f32,
}

#[derive(Component)]
pub struct Combine {
    vehicle: Vehicle,
    harvest_speed: f32,
    transfer_speed: f32,
    capacity: f32,
}

#[derive(Component)]
pub struct Truck {
    vehicle: Vehicle,
    dump_speed: f32,
    capacity: f32,
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<game::Game>,
) {
    let combine_id = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 5. })),
            material: materials.add(Color::BLUE.into()),
            transform: Transform {
                translation: Vec3::new(25., 2.6, 0.).into(),
                ..default()
            },
            ..default()
        })
        .insert(Combine {
            vehicle: Vehicle {
                drive_speed: 10.,
                turn_rate: 4.,
                acceleration: 5.,
            },
            harvest_speed: 5.,
            transfer_speed: 10.,
            capacity: 100.,
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::from_linear(Vec3::ZERO).with_angular(AxisAngle::new(Vec3::Y, 0.)))
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3 {
                x: 2.5,
                y: 2.5,
                z: 2.5,
            },
            border_radius: None,
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
        .id();

    game.combine = Some(combine_id);

    let truck_id = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 3. })),
            material: materials.add(Color::RED.into()),
            transform: Transform {
                translation: Vec3::new(-15., 1.6, 0.).into(),
                ..default()
            },
            ..default()
        })
        .insert(Truck {
            vehicle: Vehicle {
                drive_speed: 15.,
                turn_rate: 6.,
                acceleration: 10.,
            },
            dump_speed: 20.,
            capacity: 200.,
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::from_linear(Vec3::ZERO).with_angular(AxisAngle::new(Vec3::Y, 0.)))
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3 {
                x: 1.5,
                y: 1.5,
                z: 1.5,
            },
            border_radius: None,
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
        .id();

    game.truck = Some(truck_id);
}

pub fn harvest_event_handler(
    mut commands: Commands,
    mut harvest_events: EventReader<harvest::CropHarvestedEvent>,
    mut game: ResMut<game::Game>,
    mut query: Query<&harvest::Crop>,
) {
    for harvest in harvest_events.iter() {
        let entity = harvest.entity;
        if let Ok(crop) = query.get_mut(entity) {
            println!("found a crop with value: {:?} ", crop.value);
        }
    }
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
    for (combine, mut transform, mut velocity) in query.iter_mut() {
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
}

pub fn move_truck(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Truck, &mut Transform, &mut Velocity)>,
    time: Res<Time>,
) {
    // there's always exactly one but I didn't understand resources when I wrote this
    for (truck, mut transform, mut velocity) in query.iter_mut() {
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
}

fn update_vehicle(
    vehicle: &Vehicle,
    requested_direction: Vec2,
    transform: &mut Transform,
    mut velocity: &mut Velocity,
    time: &Res<Time>,
) {
    let mut speed = velocity.linear.length();
    if requested_direction.length() > 0. {
        let current_direction = transform.forward();

        if speed < vehicle.drive_speed {
            speed += vehicle.acceleration;
        }

        velocity.linear = current_direction * speed;

        let current_velocity_2d = Vec2::new(velocity.linear.x, velocity.linear.z);
        if (requested_direction.normalize() - current_velocity_2d.normalize()).length() > 0.1 {
            if current_velocity_2d.angle_between(requested_direction) > 0.05 {
                velocity.angular = AxisAngle::new(Vec3::NEG_Y, vehicle.turn_rate);
            } else {
                velocity.angular = AxisAngle::new(Vec3::Y, vehicle.turn_rate);
            }
        } else {
            velocity.angular = AxisAngle::new(Vec3::Y, 0.);
            transform.rotate_y(-current_velocity_2d.angle_between(requested_direction));
        }
    } else {
        if velocity.linear.length() > 0. {
            let current_direction = transform.forward();
            speed = f32::max(speed - vehicle.acceleration, 0.);
            velocity.linear.x = current_direction.x * speed;
            velocity.linear.z = current_direction.z * speed;
        }

        velocity.angular = AxisAngle::new(Vec3::Y, 0.);
    }
}
