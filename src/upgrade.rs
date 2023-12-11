use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::{events::{WaveEnd, Sucked, PickedUpgrade}, component::{FloatTimer, Suckable}, resource::{CameraSettings, Stats}, GameState};

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickedUpgrade>()
            .add_systems(Update, (
                spawn_update,
                detect_suck_events,
                remove_all_upgrades,
                show_labels,
            ).run_if(in_state(GameState::Game))
            );
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

impl Upgrade {
    fn random() -> (Self, String) {
        let mut rng = rand::thread_rng();
        let mut values = vec![0.0; 5];
        let random_index = rng.gen_range(0..values.len());
        values[random_index] = 0.1;
        let strings = vec![
            "Increases maximum health by 10%".to_string(),
            "Heals 20% of your maximum health".to_string(),
            "Increases health regeneration by 10%".to_string(),
            "Decreases time to vacuum ghosts by 10%".to_string(),
            "Increases movement speed by 10%".to_string()
        ];
        (Self {
            max_health: 1.0 + values[0],
            health: 2.0 * values[1],
            regeneration: 1.0 + values[2],
            suck_time: 1.0 - values[3],
            movement_speed: 1.0 + values[4],
        },
        strings[random_index].clone()
        )
    }
}

#[derive(Component)]
struct UpgradeLabel(Entity);

fn spawn_update(
    asset_server: Res<AssetServer>,
    mut wave_end_event: EventReader<WaveEnd>,
    mut commands: Commands,
) {
    for _ in wave_end_event.read() {
        let label_text_style = TextStyle {
            font: asset_server.load("graveyrd.ttf"),
            font_size: 25.0,
            color: Color::ORANGE,
        };

        let (upgrade_left, label_left) = Upgrade::random();
        let entity_left = commands.spawn(SceneBundle {
            scene: asset_server.load("dirtbag.glb#Scene0"),
            transform: Transform::from_xyz(-5.0, 0.0, 0.0),
            ..default()
        })
        .insert(upgrade_left)
        .insert(FloatTimer::new((0.0, 0.5)))
        .insert(Collider::ball(0.5))
        .insert(Sensor)
        .insert(CollisionGroups::new(Group::GROUP_5, Group::GROUP_3))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Suckable)
        .id();

        commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            UpgradeLabel(entity_left),
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(label_left, label_text_style.clone())
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::ZERO,
                        ..default()
                    })
                    .with_no_wrap(),
            );
        });

        let (upgrade_right, label_right) = Upgrade::random();
        let entity_right = commands.spawn(SceneBundle {
            scene: asset_server.load("dirtbag.glb#Scene0"),
            transform: Transform::from_xyz(5.0, 0.0, 0.0),
            ..default()
        })
        .insert(upgrade_right)
        .insert(FloatTimer::new((0.0, 0.5)))
        .insert(Collider::ball(0.5))
        .insert(Sensor)
        .insert(CollisionGroups::new(Group::GROUP_5, Group::GROUP_3))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Suckable)
        .id();

        commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            UpgradeLabel(entity_right),
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(label_right, label_text_style.clone())
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::ZERO,
                        ..default()
                    })
                    .with_no_wrap(),
            );
        });
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
    dirtbags: Query<Entity, With<Upgrade>>,
    labels: Query<Entity, With<UpgradeLabel>>,
    mut commands: Commands,
) {
    for _ in events.read() {
        for entity in &dirtbags {
            if let Some(upgrade) = commands.get_entity(entity) {
                upgrade.despawn_recursive();
            }
        }
        for entity in &labels {
            if let Some(upgrade) = commands.get_entity(entity) {
                upgrade.despawn_recursive();
            }
        }
    }
}

fn show_labels (
    mut camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut labels: Query<(&mut Style, &UpgradeLabel)>,
    labelled: Query<&GlobalTransform>,
) {

    let (camera, camera_global_transform) = camera.single_mut();

    for (mut style, label) in &mut labels {
        let Ok(t) = labelled.get(label.0) else {
            continue;
        };

        let world_position = t.translation() + Vec3::Y;

        let viewport_position = camera
            .world_to_viewport(camera_global_transform, world_position)
            .unwrap();

        style.top = Val::Px(viewport_position.y);
        style.left = Val::Px(viewport_position.x);
    }
}