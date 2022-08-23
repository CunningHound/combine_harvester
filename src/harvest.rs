use crate::game;
use crate::game::ScoreChangeEvent;
use bevy::prelude::*;

#[derive(Component)]
pub struct Crop {
    pub amount: i32,
    pub value: i32,
}

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
    mut query: Query<&Crop>,
) {
    for event in crop_squashed_events.iter() {
        commands.entity(event.entity).despawn();
    }

    for event in crop_harvested_events.iter() {
        let entity = event.entity;
        if let Ok(crop) = query.get_mut(entity) {
            score_change_events.send(game::ScoreChangeEvent { amount: crop.value })
        }
        commands.entity(event.entity).despawn();
    }
}
