use bevy::prelude::*;

use crate::harvest;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    GameOver,
}

#[derive(Default)]
pub struct Game {
    size: (i32, i32),
    pub combine: Option<Entity>,
    pub truck: Option<Entity>,
    score: i32,
    map: PbrBundle,
    camera: Camera3dBundle,
    light: Option<Entity>,
}

const MAP_HALF_SIZE: i32 = 100;
const CORN_SIZE: f32 = 2.0;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
)
{
    game.size = (2*MAP_HALF_SIZE, 2*MAP_HALF_SIZE);

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 150., -20.)
            .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });

    game.score = 0;
    commands.spawn_bundle( PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: (2 * MAP_HALF_SIZE) as f32 })),
        material: materials.add(Color::SEA_GREEN.into()),
        transform: Transform {
            translation: Vec3::ZERO.into(),
            ..default()

        },
        ..default()
    });

    let mut x = -MAP_HALF_SIZE as f32;
    loop
    {
        let mut z = -MAP_HALF_SIZE as f32;
        loop {
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: CORN_SIZE * 0.98 })),
                material: materials.add(Color::YELLOW.into()),
                transform: Transform {
                    translation: Vec3:: new(x, 0.1, z).into(),
                    ..default()
                },
                ..default()
            })
                .insert(harvest::Crop{amount: 1, value: 1});
            z += CORN_SIZE;
            if z >= MAP_HALF_SIZE as f32
            {
                break;
            }
        }
        x += CORN_SIZE;
        if x >= MAP_HALF_SIZE as f32
        {
            break;
        }
    }

    game.light = Some(commands.spawn_bundle(PointLightBundle{
        transform: Transform::from_xyz(MAP_HALF_SIZE /2., 100., MAP_HALF_SIZE /2.),
        point_light: PointLight{
            color: Color::rgb(0.9,0.9,0.9).into(),
            intensity: 70000.0,
            shadows_enabled: true,
            range: 300.,
            ..default()
        },
        ..default()
    }
    ).id());


}