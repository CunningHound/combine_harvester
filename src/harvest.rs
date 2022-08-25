use crate::game::ScoreChangeEvent;
use crate::{game, vehicles};
use bevy::prelude::*;

#[derive(Component)]
pub struct Crop {
    pub amount: i32,
}

pub const CORN_SIZE: f32 = 1.0;
pub const CORN_SIZE_FILL_FRACTION: f32 = 0.98;

pub struct CropHarvestedEvent {
    pub entity: Entity,
}

pub struct CropSquashedEvent {
    pub entity: Entity,
}

pub fn crop_events_handler(
    mut commands: Commands,
    mut crop_squashed_events: EventReader<CropSquashedEvent>,
    mut crop_harvested_events: EventReader<CropHarvestedEvent>,
    mut score_change_events: EventWriter<ScoreChangeEvent>,
    mut query: Query<(&Crop, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut combine_store: ResMut<vehicles::CombineStorage>,
) {
    for event in crop_squashed_events.iter() {
        let entity = event.entity;
        if let Ok((crop, transform)) = query.get_mut(entity) {
            let position = transform.translation;
            commands.entity(event.entity).despawn();
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube {
                    size: CORN_SIZE * CORN_SIZE_FILL_FRACTION,
                })),
                material: materials.add(Color::BEIGE.into()),
                transform: Transform {
                    translation: position,
                    scale: Vec3::new(1., 0.1, 1.).into(),
                    ..default()
                },
                ..default()
            });
        }
    }

    for event in crop_harvested_events.iter() {
        let entity = event.entity;
        if let Ok((crop, transform)) = query.get_mut(entity) {
            if combine_store.contents < combine_store.capacity {
                combine_store.contents += crop.amount;
            }
            commands.entity(event.entity).despawn();
        }
    }
}
