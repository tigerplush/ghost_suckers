use bevy::prelude::*;

use crate::{resource::CameraSettings, component::{Player, FollowCamera}, common::Random};

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(PostUpdate, update_camera);
    }
}

fn spawn_camera(
    camera_settings: Res<CameraSettings>,
    mut commands: Commands,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(camera_settings.offset)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(FollowCamera);
}

fn update_camera(
    time: Res<Time>,
    mut camera_settings: ResMut<CameraSettings>,
    player_query: Query<&Transform, (With<Player>, Without<FollowCamera>)>,
    mut camera_query: Query<&mut Transform, (With<FollowCamera>, Without<Player>)>,
) {
    if let Ok(player) = player_query.get_single() {
        for mut camera in &mut camera_query {
            camera.translation = player.translation + camera_settings.offset + Vec3::random() * camera_settings.translational_shake * camera_settings.translational_strength;
            camera_settings.tick(time.delta());
        }
    }
}