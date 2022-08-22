mod combine;

use bevy::pbr::LightEntity::Directional;
use bevy::prelude::*;
use heron::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_state(GameState::Playing)
        .add_startup_system(setup_camera)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
        .run();
}

#[derive(Default)]
struct Combine {
    entity: Option<Entity>,
    drive_speed: i32,
    harvest_speed: i32,
    transfer_speed: i32,
    capacity: i32,
}

#[derive(Default)]
struct Truck {
    entity: Option<Entity>,
    drive_speed: i32,
    dump_speed: i32,
    capacity: i32,
}

#[derive(Default)]
struct Game {
    size: (i32, i32),
    combine: Combine,
    truck: Truck,
    score: i32,
    map: PbrBundle,
    camera: Camera3dBundle,
    light: Option<Entity>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
)
{
    game.size = (100, 100);
    game.combine.entity = Some(commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 5.})),
        material: materials.add(Color::BLUE.into()),
        transform: Transform {
            translation: Vec3::new(25., 2.5, 0.).into(),
            ..default()
        },
        ..default()
        })
        .id()
    );

    game.combine.entity = Some(commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 3.})),
        material: materials.add(Color::YELLOW.into()),
        transform: Transform {
            translation: Vec3::new(-15., 1.5, 0.).into(),
            ..default()
        },
        ..default()
        })
        .id()
    );

    game.score = 0;
    commands.spawn_bundle( PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    });

    game.light = Some(commands.spawn_bundle(PointLightBundle{
        transform: Transform::from_xyz(0., 100., 0.),
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

fn setup_camera( mut commands: Commands, )
{
        commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 20., -50.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}