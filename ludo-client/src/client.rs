use bevy::prelude::*;
use std::net::UdpSocket;
use std::time::SystemTime;
use bevy_renet::netcode::{ClientAuthentication, NetcodeClientTransport};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetClient};
use ludo_commons::game::LudoGameProfile;
use crate::{handler, handshake};

#[derive(Default)]
pub struct LudoClientPlugin {
}

impl Plugin for LudoClientPlugin {
    fn build(&self, application: &mut App) {
        application.add_systems(PreStartup, Self::enable_system).add_systems(Startup, Self::connect_client_system).add_systems(PostStartup, handshake::commit_handshake_system).add_systems(Update, handler::handle_server_outcome_system);
    }
}

impl LudoClientPlugin {
    pub fn enable_system(mut commands: Commands) {
        let result = LudoGameProfile::load_from_file("game-profile.json").expect("unable to load ludo game profile");
        info!("load game profile...");
        commands.spawn(result);
    }

    pub fn connect_client_system(mut commands: Commands) {
        let client = RenetClient::new(ConnectionConfig::default());
        commands.insert_resource(client);
        let address = "127.0.0.1:2000".parse().unwrap();

        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let authentication = ClientAuthentication::Unsecure {
            server_addr: address,
            client_id: current_time.as_millis() as u64,
            user_data: None,
            protocol_id: 0,
        };
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

        commands.insert_resource(transport);

        info!("Starting Ludo client tasks.");
    }

}