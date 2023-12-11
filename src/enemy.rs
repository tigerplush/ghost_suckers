use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{component::*, collision_events::*, events::*, resource::*, enemy_spawner::{GhostSpawnConfig, Spawning}, GameState, common::Remap};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<DamageEvent>()
        .add_systems(Update, (
            move_enemies,
            detect_collisions,
            detect_suck_events,
            detect_suckage,
            rise_ghost,
        ).run_if(in_state(GameState::Game)))
        .add_systems(Update, float_entites);
    }
}

fn move_enemies(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Ghost>)>,
    mut query: Query<(&mut Transform, &Ghost), (Without<SuckTimer>, Without<Spawning>)>,
) {
    for (mut transform, ghost) in &mut query {

        let mut direction = Vec3::ZERO;
        if let Ok(player) = player_query.get_single() {
            let mut vantage = player.translation;
            vantage.y = transform.translation.y;
            transform.look_at(vantage, Vec3::Y);
            let mut diff = player.translation - transform.translation;
            diff.y = 0.0;
            direction = diff.normalize_or_zero() * time.delta_seconds() * ghost.0;
        }
        direction.y = 0.0;
        transform.translation += direction;
        transform.scale = Vec3::ONE;
    }
}

fn rise_ghost(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Spawning>)>,
    mut ghosts: Query<(&mut Transform, &mut Spawning, Entity)>,
    mut commands: Commands,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };
    for (mut transform, mut spawning, entity) in &mut ghosts {
        spawning.0.tick(time.delta());
        let height = spawning.0.percent().remap((0.0, 1.0), (-1.0, 1.0));
        transform.translation.y = height;
        transform.look_at(Vec3::new(player.translation.x, height, player.translation.z), Vec3::Y);
        if spawning.0.just_finished() {
            commands.entity(entity).remove::<Spawning>();
        }
    }
}

fn float_entites(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut FloatTimer), (Without<SuckTimer>, Without<Spawning>)>
) {
    for (mut transform, mut timer) in &mut query {
        timer.tick(time.delta());
        let height = timer.height();
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
        info!("Handling colision with player for {:?}", collision_event.0);
        if let Some(entity) = commands.get_entity(collision_event.0) {
            if let Ok(damage) = damages.get(collision_event.0) {
                damage_events.send(DamageEvent(damage.0));
                ghost_spawn_config.eliminate_ghost();
                entity.despawn_recursive();
            }
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
        info!("Handling vacuuming of {:?}", event.0);
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
                info!("Started vacuuming {:?}", entity);
                if query.contains(*entity) {
                    if let Some(mut cmds) = commands.get_entity(*entity) {
                        cmds.insert(CollisionGroups::new(Group::GROUP_2, Group::GROUP_3));
                    }
                }
            }
            SuckEvent::Stop(entity) => {
                info!("Stopped vacuuming {:?}", entity);
                if query.contains(*entity) {
                    if let Some(mut cmds) = commands.get_entity(*entity) {
                        cmds.insert(CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_3));
                    }
                }
            }
        }
    }
}