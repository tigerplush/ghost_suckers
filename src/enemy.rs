use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy)
            .add_systems(Update, move_enemies);
    }
}

#[derive(Component)]
struct Ghost;

fn move_enemies(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Ghost>>,
) {
    for mut ghost in &mut query {
        let height = time.elapsed_seconds().sin();
        ghost.translation = Vec3::new(ghost.translation.x, 1.0 + height, ghost.translation.z);
    }
}

fn spawn_enemy(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
        material: materials.add(StandardMaterial::default()),
        transform: Transform::from_xyz(0.0, 1.0, 0.0).with_scale(Vec3 { x: 0.25, y: 0.25, z: 0.25 }),
        ..default()
    })
    .insert(Name::from("Ghost"))
    .insert(Ghost);
}