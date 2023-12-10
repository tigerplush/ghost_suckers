use bevy::prelude::*;

use crate::{collision_events::SuckEvent, component::*, resource::*, common::Random, events::Sucked};

pub struct VacuumPlugin;

impl Plugin for VacuumPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Sucked>()
            .add_systems(Update, (detect_suckage, update_suckage));
    }
}

fn detect_suckage(
    config: Res<Stats>,
    mut suck_events: EventReader<SuckEvent>,
    mut query: Query<&mut Transform, With<Suckable>>,
    mut commands: Commands,
) {
    for suck_event in suck_events.read() {
        match suck_event {
            SuckEvent::Start(entity) => {
                if let Some(mut cmds) = commands.get_entity(*entity) {
                    cmds.insert(SuckTimer(Timer::from_seconds(config.suck_time, TimerMode::Once)));
                }
            }
            SuckEvent::Stop(entity) => {
                if let Some(mut cmds) = commands.get_entity(*entity) {
                    cmds.remove::<SuckTimer>();
                    if let Ok(mut suckable) = query.get_mut(*entity) {
                        suckable.scale = Vec3::ONE;
                    }
                }
            }
        }
    }
}

const SUCKING_SPEED: f32 = 2.0;

fn update_suckage(
    time: Res<Time>,
    mut suck_events: EventWriter<Sucked>,
    mut query: Query<(&mut SuckTimer, &mut Transform, Entity), Without<Nozzle>>,
    nozzles: Query<&GlobalTransform, With<Nozzle>>,
) {
    let Ok(nozzle) = nozzles.get_single() else {
        return;
    };
    for (mut timer, mut transform, entity) in &mut query {
        timer.tick(time.delta());
        transform.scale = Vec3::ONE * timer.percent_left();
        transform.rotation = Quat::random();
        let diff = nozzle.translation() - transform.translation;
        let direction = diff.normalize_or_zero() * time.delta_seconds() * SUCKING_SPEED;
        transform.translation += direction;
        if timer.finished() {
            suck_events.send(Sucked(entity));
        }
    }
}