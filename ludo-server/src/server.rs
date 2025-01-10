use bevy_renet::netcode::NetcodeServerTransport;
use bevy_renet::netcode::ServerAuthentication;
use bevy_renet::netcode::ServerConfig;
use std::any::Any;
use std::net::UdpSocket;
pub use std::time::{Duration, SystemTime};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_renet::renet::{ClientId, ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent};
use ludo_commons::Pair;
use crate::handshake;
use crate::handshake::HandshakeTimer;

#[derive(Default)]
pub struct LudoServerPlugin {
}

#[derive(Resource, Default)]
pub struct LudoClientPool {
    pub ludo_clients_pool: HashMap<ClientId, Vec<Pair<String, Box<dyn Any>>>>,
}

impl Plugin for LudoServerPlugin {
    fn build(&self, application: &mut App) {
        application.add_systems(Startup, Self::enable_listener_system).add_systems(Update, (handshake::update_handshake_timer));
    }
}

impl LudoServerPlugin {
    pub fn enable_listener_system(mut commands: Commands) {
        let address = "127.0.0.1:2000".parse().unwrap();
        let renet_server_interface = RenetServer::new(ConnectionConfig::default());
        commands.insert_resource(renet_server_interface);

        let udp_server_interface = UdpSocket::bind(address).unwrap();
        let udp_server_config = ServerConfig {
            current_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
            max_clients: 6,
            protocol_id: 0,
            public_addresses: vec![address],
            authentication: ServerAuthentication::Unsecure,
        };

        let transport = NetcodeServerTransport::new(udp_server_config, udp_server_interface).unwrap();
        commands.insert_resource(transport);
        commands.insert_resource(LudoClientPool::default());
        info!("listening server socket on {}", address);

    }

    pub fn connect_listener(mut commands: Commands, mut server_event: EventReader<ServerEvent>, mut client_pool: ResMut<LudoClientPool>) {
        for server_event in server_event.read() {
            match server_event {
                ServerEvent::ClientConnected { client_id } => {
                    client_pool.ludo_clients_pool.insert(*client_id, Vec::new());
                    info!("new client connected: {}", client_id);
                    client_pool.ludo_clients_pool.get_mut(&*client_id).unwrap().push(Pair::new("client.handshake".to_string(), Box::new(false)));
                    commands.spawn(HandshakeTimer(Timer::new(Duration::from_millis(500), TimerMode::Once), client_id.clone()));
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    client_pool.ludo_clients_pool.remove(*&client_id);
                    info!("client disconnected: {0}. Because of: {1}", client_id, reason);
                }
            }
        }
    }

}