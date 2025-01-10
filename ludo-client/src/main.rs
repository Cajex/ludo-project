mod client;

use crate::client::LudoClientPlugin;
use bevy::log::tracing_subscriber;
use bevy::prelude::*;
use bevy_renet::netcode::NetcodeClientPlugin;
use bevy_renet::RenetClientPlugin;

fn main() {
    tracing_subscriber::fmt().compact().with_ansi(true).init();
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(RenetClientPlugin)
        .add_plugins(NetcodeClientPlugin)
        .add_plugins(LudoClientPlugin::default())
        .run();
}
