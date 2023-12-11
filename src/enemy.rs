use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{component::*, collision_events::*, events::*, common::Remap, resource::*, enemy_spawner::GhostSpawnConfig, GameState};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(GhostConfig {
            height_offset: 1.0,
            height_map: (-0.5, 0.5),
        })
        .add_event::<DamageEvent>()
        .add_systems(Update, (
            move_enemies,
            detect_collisions,
            detect_suck_events,
            detect_suckage,
        ).run_if(in_state(GameState::Game)));
    }
}

#[derive(Resource)]
struct GhostConfig {
    height_offset: f32,
    height_map: (f32, f32),
}

fn move_enemies(
    time: Res<Time>,
    config: Res<GhostConfig>,
    player_query: Query<&Transform, (With<Player>, Without<Ghost>)>,
    mut query: Query<(&mut Transform, &mut FloatTimer, &Ghost), Without<SuckTimer>>,
) {
    for (mut transform, mut timer, ghost) in &mut query {
        timer.tick(time.delta());
        let height = timer.elapsed_secs().sin().remap((-1.0, 1.0), config.height_map) + config.height_offset;

        let mut direction = Vec3::ZERO;
        if let Ok(player) = player_query.get_single() {
            let mut vantage = player.translation;
            vantage.y = height;
            transform.look_at(vantage, Vec3::Y);
            let mut diff = player.translation - transform.translation;
            diff.y = 0.0;
            direction = diff.normalize_or_zero() * time.delta_seconds() * ghost.0;
        }
        transform.translation += direction;
        transform.translation.y = height;
    }
}

fn detect_collisions(
    mut ghost_spawn_config: ResMut<GhostSpawnConfig>,
    mut collision_events: EventReader<CollideWithPlayer>,
    mut damage_events: EventWriter<DamageEvent>,
    damages: Query<&Damage>,
    mut commands: Commands,
) {
    for collision_event in collision_events.read() {
        if let Some(entity) = commands.get_entity(collision_event.0) {
            if let Ok(damage) = damages.get(collision_event.0) {
                damage_events.send(DamageEvent(damage.0));
                ghost_spawn_config.eliminate_ghost();
            }
            entity.despawn_recursive();
        }
    }
}

const CAMERA_SHAKE: f32 = 0.1;

fn detect_suck_events(
    mut stats: ResMut<Stats>,
    mut ghost_spawn_config: ResMut<GhostSpawnConfig>,
    mut camera_settings: ResMut<CameraSettings>,
    mut events: EventReader<Sucked>,
    query: Query<Entity, With<Ghost>>,
    mut commands: Commands,
) {
    for event in events.read() {
        if let Ok(ghost) = query.get(event.0) {
            stats.sucked_ghosts += 1;
            ghost_spawn_config.eliminate_ghost();
            commands.entity(ghost).despawn_recursive();
            camera_settings.add(CAMERA_SHAKE);
        }
    }
}

fn detect_suckage(
    mut suck_events: EventReader<SuckEvent>,
    query: Query<&Ghost>,
    mut commands: Commands,
) {
    for suck_event in suck_events.read() {
        match suck_event {
            SuckEvent::Start(entity) => {
                if query.contains(*entity) {
                    if let Some(mut cmds) = commands.get_entity(*entity) {
                        cmds.insert(CollisionGroups::new(Group::GROUP_2, Group::GROUP_3));
                    }
                }
            }
            SuckEvent::Stop(entity) => {
                if query.contains(*entity) {
                    if let Some(mut cmds) = commands.get_entity(*entity) {
                        cmds.insert(CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_3));
                    }
                }
            }
        }
    }
}