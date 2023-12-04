use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

use camera::FollowCameraPlugin;
use input::InputPlugin;
use map_generation::MapGeneratorPlugin;
use player::PlayerPlugin;
use resource::*;

mod camera;
mod component;
mod input;
mod map_generation;
mod player;
mod resource;

fn main() {
    App::new()
        .insert_resource(InputValues::new())
        .insert_resource(CameraSettings {
            offset: Vec3 { x: 0.0, y: 10.0, z: 10.0 },
            ..default()
        })
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            FollowCameraPlugin,
            InputPlugin,
            PlayerPlugin,
            MapGeneratorPlugin,
        ))
        .run();
}
