use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::geometry::ColliderShape};
use bevy_scene_hook::{SceneHook, HookedSceneBundle};
use bevy_hanabi::prelude::*;

use crate::{resource::{InputValues, Stats}, component::{Player, Nozzle}, events::{DamageEvent, VacuumEvent}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementSettings { speed: 5.0 })
            .add_systems(Startup, (spawn_player, setup_particle))
            .add_systems(Update, (
                move_player,
                check_health,
                handle_vacuum,
                read_damage,
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

fn setup_particle(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut commands: Commands,
) {
    let effect = create_effect();

    // Insert into the asset system
    let effect_handle = effects.add(effect);

    commands
    .spawn(ParticleEffectBundle {
        effect: ParticleEffect::new(effect_handle),
        transform: Transform::from_translation(Vec3::Y),
        ..Default::default()
    });
}

fn create_effect() -> EffectAsset {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.5, 0.5, 0.5, 1.0));
    gradient.add_key(1.0, Vec4::new(0.5, 0.5, 0.5, 0.0));

    // Create a new expression module
    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(1.0),
        dimension: ShapeDimension::Surface,
    };

    let kill = KillSphereModifier {
        center: module.lit(Vec3::new(5.0, 5.0, 5.0)),
        sqr_radius: module.lit(0.1),
        kill_inside: true,
    };

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(10.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME, lifetime);
    // Create the effect asset
    EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        // Spawn at a rate of 5 particles per second
        Spawner::rate(5.0.into()).with_starts_active(false),
        // Move the expression module into the asset
        module
    )
    .with_name("VacuumEffect")
    .init(init_pos)
    .init(init_lifetime)
    .update(ForceFieldModifier::new(vec![
        ForceFieldSource {
            position: Vec3::new(0.0, 0.0, 0.0),
            max_radius: f32::MAX,
            min_radius: 0.1,
            mass: 5.0,
            force_exponent: 1.0,
            conform_to_sphere: true,
        },
    ]))
    .update(kill)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(ColorOverLifetimeModifier { gradient })
}