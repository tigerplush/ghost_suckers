use bevy::{prelude::*, window::PrimaryWindow, input::{mouse::MouseButtonInput, ButtonState}};

use crate::{resource::InputValues, component::FollowCamera, events::VacuumEvent};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VacuumEvent>()
            .add_systems(Update, update_values);
    }
}

fn update_values(
    keys: Res<Input<KeyCode>>,
    mut input_values: ResMut<InputValues>,
    mut mouse_events: EventReader<MouseButtonInput>,
    mut vacuum_events: EventWriter<VacuumEvent>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<FollowCamera>>,
) {
    let mut movement = Vec2::ZERO;
    if keys.pressed(KeyCode::A) {
        movement.x += -1.0;
    }
    if keys.pressed(KeyCode::D) {
        movement.x += 1.0;
    }
    if keys.pressed(KeyCode::W) {
        movement.y += -1.0;
    }
    if keys.pressed(KeyCode::S) {
        movement.y += 1.0;
    }

    input_values.movement = movement.normalize_or_zero();

    for event in mouse_events.read() {
        match event.state {
            ButtonState::Pressed => {
                if event.button == MouseButton::Left {
                    input_values.mouse_pressed = true;
                    vacuum_events.send(VacuumEvent::Start);
                }
            }
            ButtonState::Released => {
                if event.button == MouseButton::Left {
                    input_values.mouse_pressed = false;
                    vacuum_events.send(VacuumEvent::Stop);
                }
            }
        }
    }

    let (camera, camera_transform) = cameras.single();
    let ground_transform = GlobalTransform::default();
    let window = windows.single();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let plane_origin = ground_transform.translation();
    let plane_normal = ground_transform.up();
    let Some(distance) = ray.intersect_plane(plane_origin, plane_normal) else {
        return;
    };

    input_values.mouse_position = ray.get_point(distance);
}
