use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::component::*;

pub struct CollisionPlugin;

#[derive(Event)]
pub struct CollideWithPlayer(pub Entity);

#[derive(Event)]
pub enum SuckEvent {
    Start(Entity),
    Stop(Entity),
}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollideWithPlayer>()
            .add_event::<SuckEvent>()
            .add_systems(Update, handle_collisions);
    }
}

fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_collision: EventWriter<CollideWithPlayer>,
    mut suck_events: EventWriter<SuckEvent>,
    ghosts: Query<Entity, With<Ghost>>,
    players: Query<Entity, With<Player>>,
    nozzles: Query<Entity, With<Nozzle>>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(left, right, _) = collision_event {
            if let Some(ghost) = ghosts.iter().find(|ghost| ghost == left || ghost == right) {
                if players.iter().any(|p| p == *left || p == *right) {
                    player_collision.send(CollideWithPlayer(ghost));
                }
                if nozzles.iter().any(|n| n == *left || n == *right) {
                    suck_events.send(SuckEvent::Start(ghost));
                }
            }
        }
        if let CollisionEvent::Stopped(left, right, _) = collision_event {
            if let Some(ghost) = ghosts.iter().find(|ghost| ghost == left || ghost == right) {
                if nozzles.iter().any(|n| n == *left || n == *right) {
                    suck_events.send(SuckEvent::Stop(ghost));
                }
            }
        }
    }
}