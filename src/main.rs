use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use bevy_rand::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_scene_hook::HookPlugin;
use bevy_hanabi::prelude::*;

use camera::FollowCameraPlugin;
use collision_events::CollisionPlugin;
use enemy::EnemyPlugin;
use input::InputPlugin;
use map_generation::MapGeneratorPlugin;
use player::PlayerPlugin;
use resource::*;
use ui::UiPlugin;

mod camera;
mod collision_events;
mod component;
mod enemy;
mod events;
mod input;
mod map_generation;
mod player;
mod resource;
mod ui;
mod common;

fn main() {
    App::new()
        .insert_resource(InputValues::new())
        .insert_resource(CameraSettings {
            offset: Vec3 { x: 0.0, y: 10.0, z: 10.0 },
            translational_strength: 1.5,
            ..default()
        })
        .insert_resource(Stats::new())
        .add_plugins((
            DefaultPlugins,
            //WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            //RapierDebugRenderPlugin::default(),
            EntropyPlugin::<ChaCha8Rng>::default(),
            HanabiPlugin,
            HookPlugin,
            FollowCameraPlugin,
            InputPlugin,
            PlayerPlugin,
            MapGeneratorPlugin,
            EnemyPlugin,
            UiPlugin,
            CollisionPlugin,
        ))
        .run();
}
