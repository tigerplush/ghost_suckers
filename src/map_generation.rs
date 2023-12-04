use std::f32::consts::PI;

use bevy::prelude::*;

pub struct MapGeneratorPlugin;

impl Plugin for MapGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_first);
    }
}

#[derive(Component)]
struct NewArea;

const WIDTH: f32 = 32.0;
const HEIGHT: f32 = 18.0;

fn spawn_first(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(WIDTH, HEIGHT)))),
        material: materials.add(StandardMaterial::default()),
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, PI / -2.0, 0.0, 0.0)),
        ..default()
    });
}