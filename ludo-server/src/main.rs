mod server;
mod handshake;
mod handler;

use bevy::log::tracing_subscriber;
use bevy::prelude::*;
use bevy_renet::netcode::NetcodeServerPlugin;
use bevy_renet::RenetServerPlugin;
use crate::server::LudoServerPlugin;

fn main() {
    tracing_subscriber::fmt().compact().with_ansi(true).init();
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(RenetServerPlugin)
        .add_plugins(NetcodeServerPlugin)
        .add_plugins(LudoServerPlugin::default())
        .run();
}
