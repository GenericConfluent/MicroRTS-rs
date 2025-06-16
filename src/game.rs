//! Core gameplay functions:
//! - Send each agent the state and get back their desired action
//! - Apply the action and update the game state.

use crate::agent::{
    Agent, GameStateObservation, ProgramAgent, ProgramAgentDescriptor, SocketAgent,
    SocketAgentDescriptor, UnitAction,
};
use crate::bridge::BridgeExtension;
use crate::config::AgentDescriptor;
use crate::unit::NextAction;
use std::sync::mpsc::{channel, Sender};
use std::{
    io,
    net::{SocketAddr, TcpStream},
    sync::Arc,
};

use bevy::{prelude::*, tasks::IoTaskPool};

pub struct AgentPlugin {
    config: crate::config::Config,
}

#[derive(Resource)]
pub struct AgentSenders(Vec<Sender<GameStateObservation>>);

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        let task = IoTaskPool::get();
        // We can just clone the sender for AgentResponses.
        let (tx, rx) = channel::<AgentResponse>();
        app.add_event_channel(rx);

        let mut senders = Vec::new();
        // Attach each agent type.
        for (player_id, agent) in self.config.players.iter().enumerate() {
            let event_writer = tx.clone();
            // Each agent needs its own reciever.
            let (tx, event_reader) = channel::<GameStateObservation>();
            senders.push(tx);
            match &agent.agent_descriptor {
                AgentDescriptor::Program {
                    program,
                    args,
                    format,
                } => {
                    let mut program_agent = ProgramAgent::new(program, args, *format);
                    task.spawn(async move {
                        loop {
                            let Ok(obs) = event_reader.recv() else {
                                error_once!(
                                    "Sender for agent {} (`Program`) has hung up",
                                    player_id
                                );
                                break;
                            };
                            let response = AgentResponse {
                                id: player_id,
                                result: program_agent.compute_actions(&obs).await,
                            };
                            event_writer.send(response);
                        }
                    });
                }
                _ => panic!("Unsupported agent type {:?}", agent.agent_descriptor),
            }
        }

        app.insert_resource(AgentSenders(senders));
    }
}

#[derive(Event)]
pub struct Update;

#[derive(Event)]
pub struct AgentResponse {
    id: usize,
    result: io::Result<Vec<UnitAction>>,
}

#[derive(Event)]
pub struct StateUpdate;

fn collect_actions(
    mut event_reader: EventReader<AgentResponse>,
    mut units: Query<(Entity, &mut NextAction)>,
) {
    for response in event_reader.read() {
        if let Ok(actions) = &response.result {
            let current_action = next_agent_action.0.get(response.id);
            if current_action.is_none() {
                next_agent_action.0[response.id] = Some(actions.to_vec());
            } else {
                warn!("Agent {} sent a second action set, dropping.", response.id);
            }
        } else {
            warn!("Agent {} responded with error.", response.id);
        }
    }
}

fn step(query: Query<NextAgentAction>) {
    if next_agent_action.0.iter().any(Option::is_none) {
        return;
    }

    for action_set in next_agent_action {
        // Consider the actions currently being executed by the units.
    }
}
