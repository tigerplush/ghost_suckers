use bevy::prelude::*;

use crate::resource::Stats;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (
                update_stats,
                update_entities,
            ));
    }
}

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct GhostText;

#[derive(Component)]
struct FrostOverlay;

#[derive(Component)]
struct EntityCounter;

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

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "fartbag",
            TextStyle {
                font: asset_server.load("graveyrd.ttf"),
                font_size: 100.0,
                ..default()
            })
            .with_text_alignment(TextAlignment::Center)
        )
        .insert(GhostText);
    });

    commands.spawn(
        ImageBundle {
            style: Style {
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::rgba(1.0, 1.0, 1.0, 0.0).into(),
            image: UiImage::from(asset_server.load("frost-overlay.png")),
            ..default()
        }
    )
    .insert(FrostOverlay);

    commands.spawn(
        TextBundle::from("From an &str into a TextBundle with the default font!")
        .with_style(
            Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
        ),
    )
    .insert(EntityCounter);
}

fn update_stats(
    stats: Res<Stats>,
    mut health: Query<&mut Text, (With<HealthText>, Without<GhostText>)>,
    mut ghosts: Query<&mut Text, (With<GhostText>, Without<HealthText>)>,
    mut overlays: Query<&mut BackgroundColor, With<FrostOverlay>>,
) {
    for mut text in &mut health {
        text.sections[1].value = format!("{0:.0}", stats.health);
    }

    for mut text in &mut ghosts {
        text.sections[0].value = format!("- {} -", stats.sucked_ghosts);
    }

    for mut overlay in &mut overlays {
        overlay.0 = Color::rgba(1.0, 1.0, 1.0, 1.0 - stats.normalized_health()).into();
    }
}

fn update_entities(
    query: Query<Entity>,
    mut counters: Query<&mut Text, With<EntityCounter>>,
) {
    let entities = query.iter().collect::<Vec<Entity>>().len();
    for mut counter in &mut counters {
        counter.sections[0].value = format!("Entities: {}", entities);
    }
}