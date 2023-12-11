use bevy::prelude::*;

use crate::{resource::Stats, GameState, enemy_spawner::GhostSpawnConfig};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), spawn_main_menu)
            .add_systems(Update, handle_main_menu.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_main_menu)
            .add_systems(OnEnter(GameState::Game), setup)
            .add_systems(Update, (
                update_stats,
                update_entities,
            ).run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), (update_stats, update_entities))
            .add_systems(OnEnter(GameState::GameOver), spawn_restart_button)
            .add_systems(Update, button_system.run_if(in_state(GameState::GameOver)))
            .add_systems(OnExit(GameState::GameOver), cleanup_restart_button);
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

#[derive(Component)]
struct WaveCounter;

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

    commands.spawn(
        TextBundle::from_section(
            "fartbag",
            TextStyle {
                font: asset_server.load("graveyrd.ttf"),
                font_size: 50.0,
                ..default()
            })
        .with_style(
            Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
        ),
    )
    .insert(WaveCounter);
}

fn update_stats(
    stats: Res<Stats>,
    ghost_config: Res<GhostSpawnConfig>,
    mut health: Query<&mut Text, (With<HealthText>, Without<GhostText>, Without<WaveCounter>)>,
    mut ghosts: Query<&mut Text, (With<GhostText>, Without<HealthText>, Without<WaveCounter>)>,
    mut waves: Query<&mut Text, (With<WaveCounter>, Without<HealthText>, Without<GhostText>)>,
    mut overlays: Query<&mut BackgroundColor, With<FrostOverlay>>,
) {
    for mut text in &mut health {
        text.sections[1].value = format!("{0:.0}", stats.health);
    }

    for mut text in &mut ghosts {
        text.sections[0].value = format!("- {} -", stats.sucked_ghosts);
    }

    for mut text in &mut waves {
        text.sections[0].value = format!("Wave: {}", ghost_config.current_wave());
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

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct ButtonNode;

fn spawn_restart_button(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(ButtonNode)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Restart",
                        TextStyle {
                            font: asset_server.load("graveyrd.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn button_system(
    mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Restart".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                game_state.set(GameState::Game);
            }
            Interaction::Hovered => {
                text.sections[0].value = "Restart".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Restart".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn cleanup_restart_button(
    query: Query<Entity, With<ButtonNode>>,
    mut commands: Commands,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
struct MainMenu;

fn spawn_main_menu(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(MainMenu)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: asset_server.load("graveyrd.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn handle_main_menu(
    mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Start".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                game_state.set(GameState::Game);
            }
            Interaction::Hovered => {
                text.sections[0].value = "Start".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Start".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn cleanup_main_menu(
    query: Query<Entity, With<MainMenu>>,
    mut commands: Commands,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}