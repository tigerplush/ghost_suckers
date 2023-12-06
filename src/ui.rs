use bevy::prelude::*;

use crate::resource::Stats;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_stats);
    }
}

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct GhostText;

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn(TextBundle::from_sections([
        TextSection::new(
            "HP: ",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("graveyrd.ttf"),
                font_size: 60.0,
                ..default()
            },
        ),
        TextSection::from_style(if cfg!(feature = "default_font") {
            TextStyle {
                font_size: 60.0,
                color: Color::GOLD,
                // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                ..default()
            }
        } else {
            // "default_font" feature is unavailable, load a font to use instead.
            TextStyle {
                font: asset_server.load("graveyrd.ttf"),
                font_size: 60.0,
                color: Color::GOLD,
            }
        }),
    ]),
    )
    .insert(HealthText);

    commands.spawn((
        TextBundle::from_section(
            "hello\nbevy!",
            TextStyle {
                font: asset_server.load("graveyrd.ttf"),
                font_size: 100.0,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        GhostText,
    ));
}

fn update_stats(
    stats: Res<Stats>,
    mut health: Query<&mut Text, (With<HealthText>, Without<GhostText>)>,
    mut ghosts: Query<&mut Text, (With<GhostText>, Without<HealthText>)>,
) {
    for mut text in &mut health {
        text.sections[1].value = format!("{0:.0}", stats.health);
    }

    for mut text in &mut ghosts {
        text.sections[0].value = format!("{}", stats.sucked_ghosts);
    }
}