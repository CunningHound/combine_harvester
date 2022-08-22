use bevy::prelude::*;

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


pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
)
{
    game.size = (100, 100);

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