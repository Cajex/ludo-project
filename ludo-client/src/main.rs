mod client;
mod handshake;
mod handler;
mod interface;
mod game;

use crate::client::LudoClientPlugin;
use crate::interface::LudoClientUserInterfacePlugin;
use bevy::log::tracing_subscriber;
use bevy::prelude::*;
use bevy::window::{WindowLevel, WindowTheme};
use bevy_renet::netcode::NetcodeClientPlugin;
use bevy_renet::RenetClientPlugin;
use bevy_simple_text_input::TextInputPlugin;

fn main() {
    tracing_subscriber::fmt().compact().with_ansi(true).init();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ludo game".to_string(),
                name: Some("ludo game".to_owned()),
                resizable: false,
                fullsize_content_view: false,
                window_level: WindowLevel::AlwaysOnTop,
                transparent: true,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(TextInputPlugin)
        .add_plugins(RenetClientPlugin)
        .add_plugins(NetcodeClientPlugin)
        .add_plugins(LudoClientPlugin::default())
        .add_plugins(LudoClientUserInterfacePlugin::default())
        .run();
}
