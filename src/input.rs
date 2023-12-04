use bevy::prelude::*;

use crate::resource::InputValues;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_values);
    }
}

fn update_values(
    keys: Res<Input<KeyCode>>, mut input_values: ResMut<InputValues>,
    buttons: Res<Input<MouseButton>>,
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
    input_values.mouse_pressed = buttons.pressed(MouseButton::Left);
}
