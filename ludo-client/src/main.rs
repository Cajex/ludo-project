mod client;
mod handshake;
mod handler;
mod interface;

use crate::client::LudoClientPlugin;
use crate::interface::LudoClientUserInterfacePlugin;
use bevy::log::tracing_subscriber;
use bevy::prelude::*;
use bevy::window::{ExitCondition, WindowLevel};
use bevy_renet::netcode::NetcodeClientPlugin;
use bevy_renet::RenetClientPlugin;

fn main() {
    tracing_subscriber::fmt().compact().with_ansi(true).init();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                position: WindowPosition::Centered(MonitorSelection::Current),
                title: "ludo game".to_string(),
                name: Some("ludo game".to_owned()),
                resizable: false,
                decorations: false,
                window_level: WindowLevel::AlwaysOnTop,
                transparent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RenetClientPlugin)
        .add_plugins(NetcodeClientPlugin)
        .add_plugins(LudoClientPlugin::default())
        .add_plugins(LudoClientUserInterfacePlugin::default())
        .run();
}
