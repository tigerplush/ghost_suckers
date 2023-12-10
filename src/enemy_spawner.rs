use std::time::Duration;

use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::resource::GlobalEntropy;
use bevy_rapier3d::prelude::*;
use rand_core::RngCore;

use crate::{component::*, events::{WaveEnd, PickedUpgrade}};

pub struct EnemySpawnerPlugin;

const INITIAL_TIME_BETWEEN_GHOSTS: f32 = 0.8;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WaveEnd>()
        .insert_resource(GhostSpawnConfig {
            timer: Timer::new(Duration::from_secs_f32(INITIAL_TIME_BETWEEN_GHOSTS), TimerMode::Repeating),
            current_time_between_ghosts: INITIAL_TIME_BETWEEN_GHOSTS,
            damage: 8.0,
            speed: 2.0,
            wave_size: 40,
            spawned_ghosts: 0,
            eliminated_ghosts: 0,
            current_wave: 1,
        })
        .add_systems(Update, (
            spawn_enemy,
            check_wave_end,
            check_picked_upgrade,
        ));
    }
}

#[derive(Resource)]
pub struct GhostSpawnConfig {
    timer: Timer,
    current_time_between_ghosts: f32,
    damage: f32,
    speed: f32,
    wave_size: u32,
    spawned_ghosts: u32,
    eliminated_ghosts: u32,
    current_wave: u32,
}

impl GhostSpawnConfig {
    pub fn wave_size(&self) -> u32 {
        self.wave_size
    }

    pub fn eliminate_ghost(&mut self) {
        self.eliminated_ghosts += 1;
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

    if config.timer.finished() && config.spawned_ghosts < config.wave_size {
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
        .insert(Ghost(config.speed))
        .insert(Collider::capsule(Vec3::Y / -4.0, Vec3::Y / 4.0, 0.25))
        //.insert(RigidBody::KinematicPositionBased)
        .insert(Sensor)
        .insert(CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_3))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(FloatTimer::new())
        .insert(Damage(config.damage))
        .insert(Suckable);

        config.spawned_ghosts += 1;
    }
}

fn check_wave_end(
    mut config: ResMut<GhostSpawnConfig>,
    mut wave_end_events: EventWriter<WaveEnd>,
) {
    if config.eliminated_ghosts == config.wave_size {
        wave_end_events.send(WaveEnd);
        config.eliminated_ghosts = 0;
    }
}

fn check_picked_upgrade(
    mut config: ResMut<GhostSpawnConfig>,
    mut picked_upgrade_events: EventReader<PickedUpgrade>,
) {
    for _ in picked_upgrade_events.read() {
        config.wave_size = (config.wave_size as f32 * 1.1) as u32;
        config.spawned_ghosts = 0;
        config.damage *= 1.1;
        config.speed *= 1.1;
        config.current_time_between_ghosts *= 0.9;
        config.timer = Timer::new(Duration::from_secs_f32(config.current_time_between_ghosts), TimerMode::Repeating);
        config.timer.reset();
        config.current_wave += 1;
    }
}