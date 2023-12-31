use bevy::{prelude::*, audio::{VolumeLevel, PlaybackMode}};

use crate::{resource::Stats, component::Ghost, enemy_spawner::GhostSpawnConfig, events::{VacuumEvent, WaveEnd, PickedUpgrade, Sucked, DamageEvent}, GameState};

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DangerLevel::new())
            .add_systems(OnEnter(GameState::Menu), play_base_track)
            .add_systems(OnExit(GameState::Menu), stop_base_track)
            .add_systems(OnEnter(GameState::Game), setup)
            .add_systems(Update, (
                update_danger_level,
                start_stop_vacuum,
                apply_danger_level,
                check_vacuum_sound,
                check_wave_end,
                play_upgrade_sound,
                suck_ghosts,
                hurt,
            ).run_if(in_state(GameState::Game)))
            .add_systems(OnEnter(GameState::GameOver), kill_all_sound)
            .add_systems(OnExit(GameState::GameOver), kill_all_sound);
    }
}

fn play_base_track(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/Basetrack.wav"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..default()
        },
        ..default()
    })
    .insert(BaseTrack);
}

fn stop_base_track(
    query: Query<Entity, With<BaseTrack>>,
    mut commands: Commands,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
struct BaseTrack;
#[derive(Component)]
struct MediumTrack;

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/Basetrack.wav"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..default()
        },
        ..default()
    })
    .insert(BaseTrack);

    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/Medium Track.wav"),
        settings: PlaybackSettings {
            volume: bevy::audio::Volume::Relative(VolumeLevel::new(0.0)),
            mode: PlaybackMode::Loop,
            ..default()
        },
        ..default()
    })
    .insert(MediumTrack);
}

#[derive(Resource)]
struct DangerLevel {
    remaining_health: f32,
    ghosts_on_screen: f32,
    health_weight: f32,
    ghost_weight: f32,
    base_to_medium_threshold: (f32, f32),
}

impl DangerLevel {
    pub fn new() -> Self {
        Self {
            remaining_health: 1.0,
            ghosts_on_screen: 0.0,
            health_weight: 2.0,
            ghost_weight: 1.0,
            base_to_medium_threshold: (0.32, 0.35),
        }
    }

    pub fn update(&mut self, remaining_health: f32, ghosts_on_screen: f32) {
        self.remaining_health = remaining_health;
        self.ghosts_on_screen = ghosts_on_screen;
    }

    /// Returns the normalized danger level with weights applied
    pub fn danger_level(&self) -> f32 {
        let missing_health = 1.0 - self.remaining_health;
        (missing_health * self.health_weight + self.ghosts_on_screen * self.ghost_weight) / (self.health_weight + self.ghost_weight)
    }
}

fn update_danger_level(
    stats: Res<Stats>,
    wave_config: Res<GhostSpawnConfig>,
    mut danger_level: ResMut<DangerLevel>,
    ghosts: Query<&Ghost>,
) {
    let ghosts_on_screen = ghosts.iter().collect::<Vec<&Ghost>>().len();
    danger_level.update(stats.normalized_health(), ghosts_on_screen as f32 / wave_config.wave_size() as f32);
}

fn apply_danger_level(
    time: Res<Time>,
    danger_level: Res<DangerLevel>,
    base_tracks: Query<&AudioSink, (With<BaseTrack>, Without<MediumTrack>)>,
    medium_tracks: Query<&AudioSink, (With<MediumTrack>, Without<BaseTrack>)>,
) {
    let Ok(base_track) = base_tracks.get_single() else {
        return;
    };
    let Ok(medium_track) = medium_tracks.get_single() else {
        return;
    };

    if danger_level.danger_level() > danger_level.base_to_medium_threshold.0 {
        medium_track.set_volume((medium_track.volume() + time.delta_seconds()).clamp(0.0, 1.0));
        base_track.set_volume((base_track.volume() - time.delta_seconds()).clamp(0.0, 1.0));
    }

    if danger_level.danger_level() < danger_level.base_to_medium_threshold.1 {
        medium_track.set_volume((medium_track.volume() - time.delta_seconds()).clamp(0.0, 1.0));
        base_track.set_volume((base_track.volume() + time.delta_seconds()).clamp(0.0, 1.0));
    }
}

#[derive(Component)]
struct VacuumSound;

#[derive(Component)]
struct VacuumStart;

fn vacuum_volume() -> bevy::audio::Volume {
    bevy::audio::Volume::Relative(VolumeLevel::new(0.5))
}

fn start_stop_vacuum(
    asset_server: Res<AssetServer>,
    mut vacuum_events: EventReader<VacuumEvent>,
    vacuum_sounds: Query<Entity, With<VacuumSound>>,
    mut commands: Commands,
) {
    for vacuum_event in vacuum_events.read() {
        if let Ok(vacuum_sound) = vacuum_sounds.get_single() {
            commands.entity(vacuum_sound).despawn_recursive();
        }
        match vacuum_event {
            VacuumEvent::Start => {
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/vacuum_start.wav"),
                    settings: PlaybackSettings {
                        volume: vacuum_volume(),
                        ..default()
                    },
                    ..default()
                })
                .insert(VacuumSound)
                .insert(VacuumStart);
            }
            VacuumEvent::Stop => {
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/vacuum_stop.wav"),
                    settings: PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        volume: vacuum_volume(),
                        ..default()
                    },
                    ..default()
                })
                .insert(VacuumSound);
            }
        }
    }
}

fn check_vacuum_sound(
    asset_server: Res<AssetServer>,
    vacuum_starts: Query<(&AudioSink, Entity), With<VacuumStart>>,
    mut commands: Commands,
) {
    let Ok((vacuum_start, entity)) = vacuum_starts.get_single() else {
        return;
    };

    if vacuum_start.empty() {
        commands.entity(entity).despawn_recursive();
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/vacuum_running.wav"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: vacuum_volume(),
                ..default()
            },
            ..default()
        })
        .insert(VacuumSound);
    }
}

fn check_wave_end(
    asset_server: Res<AssetServer>,
    mut wave_end_event: EventReader<WaveEnd>,
    mut commands: Commands,
) {
    for _ in wave_end_event.read() {

        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/upgrade_sound.wav"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: vacuum_volume(),
                ..default()
            },
            ..default()
        });
    }
}

fn play_upgrade_sound(
    asset_server: Res<AssetServer>,
    mut events: EventReader<PickedUpgrade>,
    mut commands: Commands,
) {
    for _ in events.read() {
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/blop.wav"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: vacuum_volume(),
                ..default()
            },
            ..default()
        });
    }
}

fn suck_ghosts(
    asset_server: Res<AssetServer>,
    mut events: EventReader<Sucked>,
    mut commands: Commands,
) {
    for _ in events.read() {
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/suck_pop.wav"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: vacuum_volume(),
                ..default()
            },
            ..default()
        });
    }
}

fn hurt(
    asset_server: Res<AssetServer>,
    mut events: EventReader<DamageEvent>,
    mut commands: Commands,
) {
    for _ in events.read() {
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/oof.wav"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: vacuum_volume(),
                ..default()
            },
            ..default()
        });
    }
}

fn kill_all_sound(
    query: Query<Entity, With<AudioSink>>,
    mut commands: Commands,
) {
    for sink in &query {
        commands.entity(sink).despawn_recursive();
    }
}