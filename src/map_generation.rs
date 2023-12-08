use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::geometry::ColliderShape};
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

pub struct MapGeneratorPlugin;

impl Plugin for MapGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_first);
    }
}

#[derive(Component)]
struct NewArea;

fn spawn_first(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn(HookedSceneBundle {
        scene: SceneBundle {
        scene: asset_server.load("areas/default.glb#Scene0"),
        ..default()
        },
        hook: SceneHook::new(|entity, cmds| {
            match entity.get::<Name>().map(|t| t.as_str()) {
                Some(string) => {
                    if string.starts_with("FenceSection") {
                        cmds.insert(Collider::from(ColliderShape::cuboid(1.5, 1.0, 0.5)));
                    }
                    else if string.starts_with("Column") {
                        cmds.insert(Collider::from(ColliderShape::cuboid(0.25, 1.0, 0.25)));
                    }
                    cmds
                },
                _ => cmds,
            };
        }),
    })
    .insert(Name::from("Area-0-0"))
    .with_children(|parent| {
        parent.spawn(Collider::from(ColliderShape::cuboid(16.0, 1.0, 0.5)))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 1.0, 10.0)))
        .insert(Name::from("LowerBarrier"));
    });
}
