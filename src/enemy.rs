use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use bevy_prng::ChaCha8Rng;
use bevy_rand::resource::GlobalEntropy;
use bevy_rapier3d::prelude::*;
use rand_core::RngCore;

use crate::{resource::Stats, component::Player};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GhostSpawnConfig {
            timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Repeating),
        }).add_systems(Update, (spawn_enemy, move_enemies, detect_collisions));
    }
}

#[derive(Component)]
struct Ghost;

#[derive(Resource)]
struct GhostSpawnConfig {
    timer: Timer,
}

#[derive(Deref, DerefMut, Component)]
struct FloatTimer(Stopwatch);

fn move_enemies(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Ghost>)>,
    mut query: Query<(&mut Transform, &mut FloatTimer), With<Ghost>>,
) {
    for (mut ghost, mut timer) in &mut query {
        timer.tick(time.delta());
        let height = timer.elapsed_secs().sin() + 1.0;

        let mut direction = Vec3::ZERO;
        if let Ok(player) = player_query.get_single() {
            let mut vantage = player.translation;
            vantage.y = height;
            ghost.look_at(vantage, Vec3::Y);
            let mut diff = player.translation - ghost.translation;
            diff.y = 0.0;
            direction = diff.normalize_or_zero() * time.delta_seconds() * 0.5;
        }
        ghost.translation += direction;
        ghost.translation.y = height;
    }
}

fn spawn_enemy(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    mut config: ResMut<GhostSpawnConfig>,
    query: Query<&Transform, With<Player>>,
    mut commands: Commands
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        let mut pos = Vec3::new(5.0, 1.0, 5.0);

        if let Ok(player) = query.get_single() {
            let angle = rng.next_u32() as f32 * 100.0;
            let radius = 5.0;
            pos.x = angle.sin() * radius + player.translation.x;
            pos.z = angle.cos() * radius + player.translation.z;
        }
        commands.spawn(SceneBundle {
            scene: asset_server.load("ghost.glb#Scene0"),
            transform: Transform::from_translation(pos).with_scale(Vec3 { x: 0.5, y: 0.5, z: 0.5 }),
            ..default()
        })
        .insert(Name::from("Ghost"))
        .insert(Ghost)
        .insert(Collider::capsule(Vec3::Y / -2.0, Vec3::Y / 2.0, 0.5))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(FloatTimer(Stopwatch::new()));
    }
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