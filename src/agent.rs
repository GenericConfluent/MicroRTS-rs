use bevy::prelude::*;
use std::net::{SocketAddr, TcpStream};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Event)]
pub struct GameStateObservation {}

#[derive(Deserialize, Clone, Copy, Debug)]
pub enum UnitAction {
    Attack,
    Harvest,
    Return,
    Move,
    Produce,
}

/// Generic interface game code uses to interact with agents. You are not limited
/// to the agent types defined below. If you're fine working on Rust you can add
/// this as a dependency, implement `AgentHandle` on your own agent and add them
/// manually.
pub trait Agent {
    async fn compute_actions(&mut self, obs: &GameStateObservation) -> io::Result<Vec<UnitAction>>;
}

/// The game state will be serialized into `format` and passed on STDIN.
/// The config info will be passed as env varibles. This agent type will
/// be not be killed until the end of the game and will continuously recieve
/// information as one would reasonably expect.
///
/// # Example
/// ```c
/// int main() {
///     struct GameStateObservation obs;
///     struct Actions acts;
///     while (1) {
///         // From stdin
///         read_game_state(&obs);
///         if (obs.finished) break;
///         compute_actions(&acts);
///         // To stdout
///         write_actions(&acts);
///     }
///     return 0;
/// }
/// ```
pub struct ProgramAgent {
    child: async_std::process::Child,
    format: crate::config::AgentFormat,
}

use async_std::process::{Child, Command};
use std::io;

use crate::config::AgentFormat;

impl ProgramAgent {
    pub fn new(program: &str, args: &[String], format: AgentFormat) -> Self {
        Self {
            child: Command::new(program)
                .args(args)
                .spawn()
                .expect("Failed to run program agent"),
            format,
        }
    }
}

impl Agent for ProgramAgent {
    async fn compute_actions(
        &mut self,
        obs: &GameStateObservation,
    ) -> std::io::Result<Vec<UnitAction>> {
        if let async_std::process::Child {
            stdin: Some(stdin),
            stdout: Some(out),
            ..
        } = &self.child
        {
            Ok(vec![])
        } else {
            Err(io::ErrorKind::NotFound.into())
        }
    }
}

/// Like `ProgramAgent` the data will be serialized into `format`. The only
/// difference is that here you won't have access to config values in the env.
pub struct SocketAgent {
    conn: Option<SocketAgentConn>,
    descriptor: SocketAgentDescriptor,
}

pub struct SocketAgentConn {
    stream: TcpStream,
    addr: SocketAddr,
}

#[derive(Clone)]
pub struct SocketAgentDescriptor {
    format: ProgramAgentFormat,
}

impl From<SocketAgentDescriptor> for SocketAgent {
    fn from(descriptor: SocketAgentDescriptor) -> Self {
        Self {
            conn: None,
            descriptor,
        }
    }
}

// TODO: Maybe a dylib option.
