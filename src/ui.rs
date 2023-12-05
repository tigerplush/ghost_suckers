use bevy::prelude::*;

use crate::resource::Stats;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_health);
    }
}

#[derive(Component)]
struct HealthText;

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn(
        TextBundle::from_section(
            "100",
            TextStyle {
                font: asset_server.load("graveyrd.ttf"),
                font_size: 100.0,
                color: Color::GOLD
        })
    )
    .insert(HealthText);
}

fn update_health(
    stats: Res<Stats>,
    mut query: Query<&mut Text, With<HealthText>>,
) {
    for mut text in &mut query {
        text.sections[0].value = format!("{0:.0}", stats.health);
    }
}