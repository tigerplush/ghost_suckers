use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{resource::Stats, component::Player};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy)
            .add_systems(Update, (move_enemies, detect_collisions));
    }
}

#[derive(Component)]
struct Ghost;

fn move_enemies(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Ghost>)>,
    mut query: Query<&mut Transform, With<Ghost>>,
) {
    for mut ghost in &mut query {
        let height = time.elapsed_seconds().sin();
        ghost.translation = Vec3::new(ghost.translation.x, 1.0 + height, ghost.translation.z);
        if let Ok(player) = player_query.get_single() {
            let mut vantage = player.translation;
            vantage.y = ghost.translation.y;
            ghost.look_at(vantage, Vec3::Y);
        }
    }
}

fn spawn_enemy(
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("ghost.glb#Scene0"),
        transform: Transform::from_xyz(5.0, 1.0, 5.0).with_scale(Vec3 { x: 0.5, y: 0.5, z: 0.5 }),
        ..default()
    })
    .insert(Name::from("Ghost"))
    .insert(Ghost)
    .insert(Collider::capsule(Vec3::Y / -2.0, Vec3::Y / 2.0, 0.5))
    .insert(RigidBody::KinematicPositionBased)
    .insert(Sensor)
    .insert(ActiveEvents::COLLISION_EVENTS);
}

fn detect_collisions(
    mut stats: ResMut<Stats>,
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<(&Ghost, Entity)>,
    player_query: Query<(&Player, Entity)>,
    mut commands: Commands,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(a, b, _) = collision_event {
            query.iter()
                .filter(|(_, e)| e == a || e == b)
                .filter(|(_, _)| player_query.contains(*a) || player_query.contains(*b))
                .for_each(|(_, e)| {
                    stats.health -= 10.0;
                    commands.entity(e).despawn_recursive();
            });
        }
    }
}