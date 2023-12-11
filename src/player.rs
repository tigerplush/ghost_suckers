use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::geometry::ColliderShape};
use bevy_scene_hook::{SceneHook, HookedSceneBundle};

use crate::{resource::{InputValues, Stats}, component::{Player, Nozzle}, events::*, common::{Random, point_in_circle}, GameState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDied>()
            .add_systems(OnEnter(GameState::Game), (spawn_player, reset_stats))
            .add_systems(Update, (
                move_player,
                check_health,
                handle_vacuum,
                read_damage,
                spawn_vacuum_effect,
                move_vacuum_effect,
                handle_between_waves,
            ).run_if(in_state(GameState::Game)));
    }
}

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
                    Some("Nozzle") => {
                        cmds.with_children( |parent| {
                            // let (vertices, indices) = create_vacuum_range();
                            // let col = Collider::trimesh(vertices, indices);
                            parent.spawn(Collider::from(ColliderShape::cone(1.0, 1.0)))
                            //parent.spawn(col)
                            //.insert(RigidBody::KinematicPositionBased)
                            .insert(Nozzle)
                            .insert(ColliderDisabled)
                            .insert(TransformBundle::from(Transform::from_xyz(-0.25, -0.25, 0.0)))
                            .insert(CollisionGroups::new(Group::GROUP_3, Group::GROUP_2 | Group::GROUP_5));
                        });

                        cmds
                    },
                    _ => cmds,
                };
            }),
        })
        .insert(Player)
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::TRANSLATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED_Z)
        .insert(Collider::capsule(Vec3::ZERO, Vec3::Y, 0.25))
        .insert(GravityScale(0.0))
        .insert(Velocity::default())
        .insert(CollisionGroups::new(Group::GROUP_1, Group::GROUP_2 | Group::GROUP_4))
        .insert(Name::from("Player"));


    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz( 0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn reset_stats(
    mut commands: Commands,
) {
    commands.insert_resource(Stats::new());
}

fn move_player(
    stats: Res<Stats>,
    input_values: Res<InputValues>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    for (mut velocity, mut transform) in &mut query {
        velocity.linvel = Vec3::new(input_values.movement.x, 0.0, input_values.movement.y) * stats.movement_speed * stats.normalized_health();
        transform.look_at(input_values.mouse_position, Vec3::Y);
    }
}

fn handle_vacuum(
    mut mouse_events: EventReader<VacuumEvent>,
    query: Query<Entity, With<Nozzle>>,
    mut commands: Commands,
) {
    for e in &query {
        for events in mouse_events.read() {
            match events {
                VacuumEvent::Start => {
                    commands.entity(e).remove::<ColliderDisabled>();
                }
                VacuumEvent::Stop => {
                    commands.entity(e).insert(ColliderDisabled);
                }
            }
        }
    }
}

#[derive(Component)]
struct VacuumParticle;

fn spawn_vacuum_effect(
    input_values: Res<InputValues>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&GlobalTransform, With<Nozzle>>,
    mut commands: Commands,
) {
    let Ok(global) = query.get_single() else {
        return;
    };

    let (_, _, translation) = global.to_scale_rotation_translation();

    // we have to use down here as forward because the nozzle is rotated by 90°
    let centerpoint = translation + global.down();
    let (sin, cos) = point_in_circle(1.0);
    let point_in_circle = centerpoint + global.forward() * sin + global.left() * cos;

    if input_values.mouse_pressed {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(0.1))),
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY.with_a(0.5),
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            // we have to use down here as forward because the nozzle is rotated by 90°
            transform: Transform::from_translation(point_in_circle).with_rotation(Quat::random()),
            ..default()
        })
        .insert(Name::from("VacuumParticle"))
        .insert(VacuumParticle);
    }
}

fn move_vacuum_effect(
    time: Res<Time>,
    query: Query<&GlobalTransform, (With<Nozzle>, Without<VacuumParticle>)>,
    mut particles: Query<(&mut Transform, Entity), With<VacuumParticle>>,
    mut commands: Commands,
) {
    let Ok(global) = query.get_single() else {
        return;
    };

    for (mut particle, entity) in &mut particles {
        let distance = global.translation() - particle.translation + global.up() * 0.25;

        particle.translation += distance.normalize_or_zero() * time.delta_seconds() * distance.length_squared().max(1.0) * 2.5;
        particle.rotation = Quat::random();
        if distance.length_squared() < 0.01 {
            commands.entity(entity).despawn();
        }
    }
}

fn check_health(
    time: Res<Time>,
    mut stats: ResMut<Stats>,
    mut game_state: ResMut<NextState<GameState>>,
    query: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    if !stats.reg_paused {
        stats.regenerate(time.delta_seconds());
    }
    if stats.health <= 0.0 {
        stats.reg_paused = true;
        for player in &query {
            commands.entity(player).despawn_recursive();
        }
        game_state.set(GameState::GameOver);
    }
}

fn read_damage(
    mut stats: ResMut<Stats>,
    mut damage_events: EventReader<DamageEvent>
) {
    for damage_event in damage_events.read() {
        stats.health -= damage_event.0;
    }
}

fn handle_between_waves(
    mut stats: ResMut<Stats>,
    wave_end_events: EventReader<WaveEnd>,
    picked_upgrade_events: EventReader<PickedUpgrade>,
) {
    if !stats.reg_paused && !wave_end_events.is_empty() {
        stats.reg_paused = true;
    }

    if stats.reg_paused && !picked_upgrade_events.is_empty() {
        stats.reg_paused = false;
    }
}