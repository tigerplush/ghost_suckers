use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{resource::InputValues, component::Player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementSettings { speed: 5.0 })
            .add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

#[derive(Resource)]
struct MovementSettings {
    pub speed: f32,
}

fn spawn_player(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(StandardMaterial::default()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(Player)
        .insert(RigidBody::Dynamic)
        .insert(Collider::capsule(-Vec3::Y / 2.0, Vec3::Y / 2.0, 0.5))
        .insert(GravityScale(0.0))
        .insert(Velocity::default())
        .insert(Name::from("Player"));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            ..default()
        },
        transform: Transform::from_xyz( 0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn move_player(
    movement_settings: Res<MovementSettings>,
    input_values: Res<InputValues>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in &mut query {
        velocity.linvel = Vec3::new(input_values.movement.x, 0.0, input_values.movement.y) * movement_settings.speed;
    }
}
