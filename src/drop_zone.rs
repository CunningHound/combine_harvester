use crate::game;
use crate::vehicles;

use crate::game::ScoreChangeEvent;
use crate::vehicles::{CombineStorage, TruckStorage};
use bevy::prelude::*;
use heron::prelude::*;

#[derive(Component)]
pub struct DropZone {
    pub combine_in_zone: bool,
    pub truck_in_zone: bool,
}

const DROP_ZONE_SIZE: f32 = 40.;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: DROP_ZONE_SIZE,
            })),
            material: materials.add(Color::rgba(0.8, 0.2, 0.2, 0.1).into()),
            transform: Transform {
                translation: Vec3::new(75., 0.1, 0.).into(),
                ..default()
            },
            ..default()
        })
        .insert(DropZone {
            combine_in_zone: false,
            truck_in_zone: false,
        })
        .insert(RigidBody::Sensor)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3 {
                x: DROP_ZONE_SIZE / 2.,
                y: DROP_ZONE_SIZE / 2.,
                z: DROP_ZONE_SIZE / 2.,
            },
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(game::GameLayer::World)
                .with_masks(&[game::GameLayer::Combine, game::GameLayer::Truck]),
        )
        .insert(Collisions::default());
}

pub fn drop_zone_update(
    mut query: Query<(&mut DropZone, &Collisions)>,
    combines: Query<(&vehicles::Combine)>,
    trucks: Query<(&vehicles::Truck)>,
) {
    for (mut drop_zone, collisions) in query.iter_mut() {
        let mut combine_found: bool = false;
        let mut truck_found: bool = false;
        for entity in collisions.iter() {
            if let Ok(combine) = combines.get_component::<vehicles::Combine>(entity) {
                combine_found = true;
            } else if let Ok(truck) = trucks.get_component::<vehicles::Truck>(entity) {
                truck_found = true;
            }
        }

        drop_zone.combine_in_zone = combine_found;
        drop_zone.truck_in_zone = truck_found;
    }
}

pub fn drop_zone_accept(
    mut query: Query<&mut DropZone>,
    mut combine_store: ResMut<CombineStorage>,
    mut truck_store: ResMut<TruckStorage>,
    mut score_event: EventWriter<ScoreChangeEvent>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(drop_zone) = query.get_single() {
        if keyboard_input.pressed(KeyCode::Space) {
            if drop_zone.combine_in_zone {
                score_event.send(ScoreChangeEvent {
                    amount: combine_store.contents,
                });
                combine_store.contents = 0;
            }
            if drop_zone.truck_in_zone {
                score_event.send(ScoreChangeEvent {
                    amount: truck_store.contents,
                });
                truck_store.contents = 0;
            }
        }
    }
}
