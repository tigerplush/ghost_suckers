use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use bevy_prng::ChaCha8Rng;
use bevy_rand::resource::GlobalEntropy;
use bevy_rapier3d::prelude::*;
use rand_core::RngCore;

use crate::{component::{Player, Ghost, Nozzle, Damage}, collision_events::{CollideWithPlayer, SuckEvent}, common::*, resource::{Stats, CameraSettings}, events::DamageEvent};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GhostSpawnConfig {
            timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Repeating),
        })
        .insert_resource(GhostConfig { 
            speed: 1.0,
            sucking_speed: 2.0,
            sucking_time: 1.0,
            height_offset: 1.0,
            height_map: (-0.5, 0.5),
            camera_shake: 0.0,
        })
        .add_event::<DamageEvent>()
        .add_systems(Update, (
            spawn_enemy,
            move_enemies,
            detect_collisions,
            detect_suckage,
            update_suckage
        ));
    }
}

#[derive(Resource)]
struct GhostSpawnConfig {
    timer: Timer,
}

#[derive(Deref, DerefMut, Component)]
struct FloatTimer(Stopwatch);

#[derive(Resource)]
struct GhostConfig {
    speed: f32,
    sucking_time: f32,
    sucking_speed: f32,
    height_offset: f32,
    height_map: (f32, f32),
    camera_shake: f32,
}

fn move_enemies(
    time: Res<Time>,
    config: Res<GhostConfig>,
    player_query: Query<&Transform, (With<Player>, Without<Ghost>)>,
    mut query: Query<(&mut Transform, &mut FloatTimer), (With<Ghost>, Without<SuckTimer>)>,
) {
    for (mut ghost, mut timer) in &mut query {
        timer.tick(time.delta());
        let height = timer.elapsed_secs().sin().remap((-1.0, 1.0), config.height_map) + config.height_offset;

        let mut direction = Vec3::ZERO;
        if let Ok(player) = player_query.get_single() {
            let mut vantage = player.translation;
            vantage.y = height;
            ghost.look_at(vantage, Vec3::Y);
            let mut diff = player.translation - ghost.translation;
            diff.y = 0.0;
            direction = diff.normalize_or_zero() * time.delta_seconds() * config.speed;
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
            transform: Transform::from_translation(pos),
            ..default()
        })
        .insert(Name::from("Ghost"))
        .insert(Ghost)
        .insert(Collider::capsule(Vec3::Y / -4.0, Vec3::Y / 4.0, 0.25))
        //.insert(RigidBody::KinematicPositionBased)
        .insert(Sensor)
        .insert(CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_3))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(FloatTimer(Stopwatch::new()))
        .insert(Damage(10.0));
    }
}

fn detect_collisions(
    mut collision_events: EventReader<CollideWithPlayer>,
    mut damage_events: EventWriter<DamageEvent>,
    damages: Query<&Damage>,
    mut commands: Commands,
) {
    for collision_event in collision_events.read() {
        if let Some(entity) = commands.get_entity(collision_event.0) {
            if let Ok(damage) = damages.get(collision_event.0) {
                damage_events.send(DamageEvent(damage.0));
            }
            entity.despawn_recursive();
        }
    }
}

#[derive(Deref, DerefMut, Component)]
struct SuckTimer(Timer);

fn detect_suckage(
    config: Res<GhostConfig>,
    mut suck_events: EventReader<SuckEvent>,
    mut query: Query<&mut Transform, With<Ghost>>,
    mut commands: Commands,
) {
    for suck_event in suck_events.read() {
        match suck_event {
            SuckEvent::Start(entity) => {
                if let Some(mut cmds) = commands.get_entity(*entity) {
                    cmds.try_insert(SuckTimer(Timer::from_seconds(config.sucking_time, TimerMode::Once)));
                }
            }
            SuckEvent::Stop(entity) => {
                if let Some(mut cmds) = commands.get_entity(*entity) {
                    cmds.remove::<SuckTimer>();
                    let mut ghost = query.get_mut(*entity).unwrap();
                    ghost.scale = Vec3::ONE;
                }
            }
        }
    }
}

fn update_suckage(
    time: Res<Time>,
    config: Res<GhostConfig>,
    mut stats: ResMut<Stats>,
    mut camera_settings: ResMut<CameraSettings>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    mut query: Query<(&mut SuckTimer, &mut Transform, Entity), Without<Nozzle>>,
    nozzles: Query<&GlobalTransform, With<Nozzle>>,
    mut commands: Commands,
) {
    for (mut timer, mut transform, entity) in &mut query {
        timer.tick(time.delta());
        transform.scale = Vec3::ONE * timer.percent_left();
        transform.rotation = Quat::from_euler(EulerRot::XYZ, rng.next_u32() as f32,rng.next_u32() as f32, rng.next_u32() as f32);
        let nozzle = nozzles.single();
        let diff = nozzle.translation() - transform.translation;
        let direction = diff.normalize_or_zero() * time.delta_seconds() * config.sucking_speed;
        transform.translation += direction;
        if timer.finished() {
            commands.entity(entity).despawn_recursive();
            stats.sucked_ghosts += 1;
            camera_settings.add(config.camera_shake);
        }
    }
}