use crate::game;
use bevy::prelude::*;
use heron::prelude::*;

#[derive(Default)]
pub struct Vehicle {
    drive_speed: i32,
    turn_rate: i32,
}

#[derive(Component)]
pub struct Combine {
    vehicle: Vehicle,
    harvest_speed: i32,
    transfer_speed: i32,
    capacity: i32,
}

#[derive(Component)]
pub struct Truck {
    vehicle: Vehicle,
    dump_speed: i32,
    capacity: i32,
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
                translation: Vec3::new(25., 2.5, 0.).into(),
                ..default()
            },
            ..default()
        })
        .insert(Combine {
            vehicle: Vehicle {
                drive_speed: 10,
                turn_rate: 30,
            },
            harvest_speed: 5,
            transfer_speed: 10,
            capacity: 100,
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3 {
                x: 2.5,
                y: 2.5,
                z: 2.5,
            },
            border_radius: None,
        })
        .id();

    game.combine = Some(combine_id);

    let truck_id = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 3. })),
            material: materials.add(Color::RED.into()),
            transform: Transform {
                translation: Vec3::new(-15., 1.5, 0.).into(),
                ..default()
            },
            ..default()
        })
        .insert(Truck {
            vehicle: Vehicle {
                drive_speed: 20,
                turn_rate: 30,
            },
            dump_speed: 20,
            capacity: 200,
        })
        .id();

    game.truck = Some(truck_id);
}

pub fn move_combine(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Combine, &mut Transform)>,
    time: Res<Time>,
) {
    if keyboard_input.pressed(KeyCode::W) {
        for (combine, mut transform) in query.iter_mut() {
            transform.translation.z += combine.vehicle.drive_speed as f32 * time.delta_seconds();
        }
    }
    if keyboard_input.pressed(KeyCode::A) {
        for (combine, mut transform) in query.iter_mut() {
            transform.translation.x += combine.vehicle.drive_speed as f32 * time.delta_seconds();
        }
    }
    if keyboard_input.pressed(KeyCode::S) {
        for (combine, mut transform) in query.iter_mut() {
            transform.translation.z -= combine.vehicle.drive_speed as f32 * time.delta_seconds();
        }
    }
    if keyboard_input.pressed(KeyCode::D) {
        for (combine, mut transform) in query.iter_mut() {
            transform.translation.x -= combine.vehicle.drive_speed as f32 * time.delta_seconds();
        }
    }
}

pub fn move_truck(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Truck, &mut Transform)>,
    time: Res<Time>,
) {
    if keyboard_input.pressed(KeyCode::Up) {
        for (truck, mut transform) in query.iter_mut() {
            transform.translation.z += truck.vehicle.drive_speed as f32 * time.delta_seconds();
        }
    }
    if keyboard_input.pressed(KeyCode::Left) {
        for (truck, mut transform) in query.iter_mut() {
            transform.translation.x += truck.vehicle.drive_speed as f32 * time.delta_seconds();
        }
    }
    if keyboard_input.pressed(KeyCode::Down) {
        for (truck, mut transform) in query.iter_mut() {
            transform.translation.z -= truck.vehicle.drive_speed as f32 * time.delta_seconds();
        }
    }
    if keyboard_input.pressed(KeyCode::Right) {
        for (truck, mut transform) in query.iter_mut() {
            transform.translation.x -= truck.vehicle.drive_speed as f32 * time.delta_seconds();
        }
    }
}
