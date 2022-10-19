use crate::game;

use crate::game::GameLayer;
use bevy::prelude::*;
use heron::prelude::*;
use rand::distributions::{Distribution, Uniform};

#[derive(Default, Component)]
pub struct Animal {
    pub move_speed: f32,
    pub move_frequency: f32,
    pub jump_height: f32,
    pub direction_change_chance: f32,
    pub time_since_move: f32,
}

const sheep: Animal = Animal {
    move_speed: 3.0,
    move_frequency: 10.0,
    jump_height: 0.5,
    direction_change_chance: 0.5,
    time_since_move: 0.,
};

const pig: Animal = Animal {
    move_speed: 2.0,
    move_frequency: 5.0,
    jump_height: 0.25,
    direction_change_chance: 0.2,
    time_since_move: 0.,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let sheep_id = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
            material: materials.add(Color::rgba(0.8, 0.8, 0.8, 1.).into()),
            transform: Transform {
                translation: Vec3::new(5., 0.1, 0.).into(),
                ..default()
            },
            ..default()
        })
        .insert(sheep)
        .insert(
            CollisionLayers::none()
                .with_groups(&[GameLayer::Animal, GameLayer::Obstacle])
                .with_masks(&[GameLayer::World, GameLayer::Obstacle, GameLayer::Vehicle]),
        )
        .insert(RigidBody::Dynamic)
        .insert(Velocity::from_linear(Vec3::ZERO).with_angular(AxisAngle::new(Vec3::Y, 0.)))
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            border_radius: Some(0.2),
        });

    let sheep_id = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
            material: materials.add(Color::rgba(0.8, 0.5, 0.5, 1.).into()),
            transform: Transform {
                translation: Vec3::new(-5., 0.1, 0.).into(),
                ..default()
            },
            ..default()
        })
        .insert(pig)
        .insert(
            CollisionLayers::none()
                .with_groups(&[GameLayer::Animal, GameLayer::Obstacle])
                .with_masks(&[GameLayer::World, GameLayer::Obstacle, GameLayer::Vehicle]),
        )
        .insert(RigidBody::Dynamic)
        .insert(Velocity::from_linear(Vec3::ZERO).with_angular(AxisAngle::new(Vec3::Y, 0.)))
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            border_radius: Some(0.2),
        });
}

pub fn move_animals(
    mut query: Query<(&mut Animal, &mut Transform, &mut Velocity)>,
    time: Res<Time>,
) {
    for (mut animal, mut animal_transform, mut animal_velocity) in &mut query {
        if animal.time_since_move > 10. / animal.move_frequency {
            let random_generator = Uniform::new(0, 359);
            let mut rng = rand::thread_rng();
            if random_generator.sample(&mut rng) as f32 > animal.direction_change_chance {
                let choice = random_generator.sample(&mut rng) as f32;
                animal_transform.rotate_y(choice);
            }
            animal.time_since_move = 0.;
            animal_velocity.linear.x = animal.move_speed * animal_transform.forward().x;
            animal_velocity.linear.z = animal.move_speed * animal_transform.forward().z;
        }
        animal.time_since_move += time.delta_seconds();
    }
}
