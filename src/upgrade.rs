use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::{events::{WaveEnd, Sucked, PickedUpgrade}, component::{FloatTimer, Suckable}, resource::{CameraSettings, Stats}, common::Random};

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickedUpgrade>()
            .add_systems(Update, (spawn_update, detect_suck_events, remove_all_upgrades));
    }
}

#[derive(Component)]
struct Upgrade {
    max_health: f32,
    health: f32,
    regeneration: f32,
    suck_time: f32,
    movement_speed: f32,
}

impl Default for Upgrade {
    fn default() -> Self {
        Self {
            max_health: 1.0,
            health: 0.0,
            regeneration: 1.0,
            suck_time: 1.0,
            movement_speed: 1.0,
        }
    }
}

impl Upgrade {
    pub fn apply(&self, stats: &mut Stats) {
        let health_normalized = stats.normalized_health();
        stats.max_health *= self.max_health;
        if self.max_health > 1.0 {
            stats.health = stats.max_health * health_normalized;
        }
        stats.add_health_percent(self.health);
        stats.regeneration *= self.regeneration;
        stats.suck_time *= self.suck_time;
        stats.movement_speed *= self.movement_speed;
    }
}

impl Random for Upgrade {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut values = vec![0.0; 5];
        let random_index = rng.gen_range(0..values.len());
        values[random_index] = 0.1;
        Self {
            max_health: 1.0 + values[0],
            health: values[1],
            regeneration: 1.0 + values[2],
            suck_time: 1.0 - values[3],
            movement_speed: 1.0 + values[4],
        }
    }
}

fn spawn_update(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut wave_end_event: EventReader<WaveEnd>,
    mut commmands: Commands,
) {
    for _ in wave_end_event.read() {
        commmands.spawn(PbrBundle {
            mesh: meshes.add(shape::UVSphere::default().into()),
            material: materials.add(StandardMaterial::default()),
            transform: Transform::from_xyz(-5.0, 0.0, 0.0),
            ..default()
        })
        .insert(Upgrade::random())
        .insert(FloatTimer::new())
        .insert(Collider::ball(1.0))
        .insert(Sensor)
        .insert(CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_3))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Suckable);

        commmands.spawn(PbrBundle {
            mesh: meshes.add(shape::UVSphere::default().into()),
            material: materials.add(StandardMaterial::default()),
            transform: Transform::from_xyz(5.0, 0.0, 0.0),
            ..default()
        })
        .insert(Upgrade::random())
        .insert(FloatTimer::new())
        .insert(Collider::ball(1.0))
        .insert(Sensor)
        .insert(CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_3))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Suckable);
    }
}

const CAMERA_SHAKE: f32 = 0.1;

fn detect_suck_events(
    mut stats: ResMut<Stats>,
    mut camera_settings: ResMut<CameraSettings>,
    mut events: EventReader<Sucked>,
    mut picked_upgrade_event: EventWriter<PickedUpgrade>,
    query: Query<(Entity, &Upgrade)>,
    mut commands: Commands,
) {
    for event in events.read() {
        if let Ok((entity, upgrade)) = query.get(event.0) {
            upgrade.apply(&mut stats);
            println!("{:?}", stats);
            commands.entity(entity).despawn_recursive();
            camera_settings.add(CAMERA_SHAKE);
            picked_upgrade_event.send(PickedUpgrade);
            // if we have picked an upgrade, we want to return early or else we apply a second upgrade
            return;
        }
    }
}

fn remove_all_upgrades(
    mut events: EventReader<PickedUpgrade>,
    query: Query<Entity, With<Upgrade>>,
    mut commands: Commands,
) {
    for _ in events.read() {
        for entity in &query {
            if let Some(upgrade) = commands.get_entity(entity) {
                upgrade.despawn_recursive();
            }
        }
    }
}