use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_scene_hook::{SceneHook, HookedSceneBundle};

use crate::{resource::{InputValues, Stats}, component::Player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementSettings { speed: 5.0 })
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, check_health));
    }
}

#[derive(Resource)]
struct MovementSettings {
    pub speed: f32,
}

#[derive(Component)]
struct Nozzle;

fn spawn_player(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands
        .spawn(HookedSceneBundle {
            scene: SceneBundle {
                scene: asset_server.load("character.glb#Scene0"),
                ..default()
            },
            hook: SceneHook::new(|entity, cmds| {
                match entity.get::<Name>().map(|t| t.as_str()) {
                    Some("Nozzle") => cmds.insert(Nozzle),
                    _ => cmds,
                };
            }),
        })
        .insert(Player)
        .insert(RigidBody::Dynamic)
        .insert(Collider::capsule(Vec3::Y * 0.5, 1.5 * Vec3::Y, 0.5))
        .insert(GravityScale(0.0))
        .insert(Velocity::default())
        .insert(Name::from("Player"));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz( 0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn move_player(
    movement_settings: Res<MovementSettings>,
    input_values: Res<InputValues>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    for (mut velocity, mut transform) in &mut query {
        velocity.linvel = Vec3::new(input_values.movement.x, 0.0, input_values.movement.y) * movement_settings.speed;
        transform.look_at(input_values.mouse_position, Vec3::Y);
    }
}

fn check_health(
    stats: Res<Stats>,
    query: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    if stats.health <= 0.0 {
        for player in &query {
            commands.entity(player).despawn_recursive();
        }
    }
}
