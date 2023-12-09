use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::geometry::ColliderShape};
use bevy_scene_hook::{SceneHook, HookedSceneBundle};

use crate::{resource::{InputValues, Stats}, component::{Player, Nozzle}, events::{DamageEvent, VacuumEvent}, common::{Random, point_in_circle}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementSettings { speed: 5.0 })
            .add_systems(Startup, (spawn_player, ))
            .add_systems(Update, (
                move_player,
                check_health,
                handle_vacuum,
                read_damage,
                spawn_vacuum_effect,
                move_vacuum_effect,
            ));
    }
}

#[derive(Resource)]
struct MovementSettings {
    pub speed: f32,
}

/// Creates the vertices and the indices of a prism that spans the whole up/down axis
/// and widens moving out from the origin
fn create_vacuum_range() -> (Vec<Vect>, Vec<[u32; 3]>) {
    let vertices = vec![
        // right hand corners
        Vect::new(0.5, 0.5, 1.0),
        Vect::new(1.5, -1.0, 1.0),
        Vect::new(1.5, -1.0, -1.0),
        Vect::new(0.5, 0.5, -1.0),
        // left hand corners
        Vect::new(-0.5, 0.5, 1.0),
        Vect::new(-1.5, -1.0, 1.0),
        Vect::new(-1.5, -1.0, -1.0),
        Vect::new(-0.5, 0.5, -1.0),
        ];

    let indices: Vec<[u32; 3]> = vec![
        // right hand wall
        [0, 1, 2],
        [0, 2, 3],
        // left hand wall
        [4, 5, 6],
        [4, 6, 7],
        //front wall
        [1, 5, 6],
        [1, 6, 2],
        // back wall
        [0, 4, 7],
        [0, 7, 3],
        // top wall,
        [3, 2, 6],
        [3, 6, 7],
        // bottom wall,
        [0, 1, 5],
        [0, 5, 4],
        ];

    (vertices, indices)
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
                            .insert(TransformBundle::from(Transform::from_xyz(0.0, -0.25, 0.0)))
                            .insert(CollisionGroups::new(Group::GROUP_3, Group::GROUP_2));
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

fn move_player(
    stats: Res<Stats>,
    movement_settings: Res<MovementSettings>,
    input_values: Res<InputValues>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    for (mut velocity, mut transform) in &mut query {
        velocity.linvel = Vec3::new(input_values.movement.x, 0.0, input_values.movement.y) * movement_settings.speed * stats.normalized_health();
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
    query: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    stats.regenerate(time.delta_seconds());
    if stats.health <= 0.0 {
        for player in &query {
            commands.entity(player).despawn_recursive();
        }
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
