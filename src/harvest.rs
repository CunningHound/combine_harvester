use crate::game::ScoreChangeEvent;
use crate::{game, vehicles};
use bevy::prelude::*;

#[derive(Component)]
pub struct Crop {
    pub amount: i32,
}

pub const CORN_SIZE: f32 = 2.0;
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
    mut query: Query<(&Crop, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut combine_store: ResMut<vehicles::CombineStorage>,
    asset_server: Res<AssetServer>,
) {
    for event in crop_squashed_events.iter() {
        let entity = event.entity;
        if let Ok((crop, transform)) = query.get_mut(entity) {
            let position = transform.translation;
            commands.entity(event.entity).despawn_recursive();
            commands.spawn_bundle(SceneBundle {
                scene: asset_server.load("squashed_wheat.gltf#Scene0"),
                transform: Transform {
                    translation: Vec3::new(transform.translation.x, 0.1, transform.translation.z)
                        .into(),
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
            commands.entity(entity).despawn_recursive();
            commands.spawn_bundle(SceneBundle {
                scene: asset_server.load("harvested_wheat.gltf#Scene0"),
                transform: Transform {
                    translation: Vec3::new(transform.translation.x, 0.1, transform.translation.z)
                        .into(),
                    ..default()
                },
                ..default()
            });
        };
    }
}
