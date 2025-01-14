use bevy_renet::netcode::NetcodeServerTransport;
use bevy_renet::netcode::ServerAuthentication;
use bevy_renet::netcode::ServerConfig;
use std::any::Any;
use std::net::UdpSocket;
pub use std::time::{Duration, SystemTime};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_renet::renet::{ClientId, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use derive_new::new;
use ludo_commons::{LudoPacket, Pair};
use ludo_commons::game::{LudoGameObject, LudoGameProfile, LudoGameProfileData, LudoGameState};
use ludo_commons::packets::LudoGameOutcomeGameStartPacket;
use crate::{backup, communication, handler, handshake};
use crate::backup::LudoBackupProfileTimer;
use crate::communication::LudoProfilesInfoTimer;
use crate::handshake::HandshakeTimer;

#[derive(Default)]
pub struct LudoServerPlugin {
}

#[derive(Resource, new)]
pub struct LudoGameConfiguration {
    pub min_players_to_start: u8,
    pub max_players_to_start: u8,
}

#[derive(Resource, Default)]
pub struct LudoOnlineClientPool {
    pub ludo_clients_pool: HashMap<ClientId, Vec<Pair<String, Box<dyn Any + Sync + Send>>>>,
}

impl Plugin for LudoServerPlugin {
    fn build(&self, application: &mut App) {
        application
            .insert_resource(LudoGameConfiguration::new(4, 4))
            .add_systems(PreStartup, Self::enable_system)
            .add_systems(Startup, Self::enable_listener_system)
            .add_systems(
                Update,
                (
                    Self::connect_listener,
                    handshake::update_handshake_timer,
                    handler::handle_client_income,
                    Self::disable_application_system,
                    backup::handle_backup_profile_timer,
                    communication::handle_client_outcome_profiles_info,
                )
            );
    }
}

impl LudoServerPlugin {
    pub fn enable_system(mut commands: Commands) {
        let result = LudoGameProfileData::load_from_file("profiles.json").expect("unable to load ludo game profiles");
        info!("load game profiles...");
        result.iter().for_each(|profile| {
            let info = commands.spawn(profile.clone()).id();
            info!("loaded profile to the cache: [{}].", info);
        });
        commands.insert_resource(LudoGameObject { state: LudoGameState::Waiting });
        commands.spawn(LudoBackupProfileTimer(Timer::new(Duration::from_secs(9), TimerMode::Repeating)));
        commands.spawn(LudoProfilesInfoTimer(Timer::new(Duration::from_secs(2), TimerMode::Repeating)));
    }

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
        commands.insert_resource(LudoOnlineClientPool::default());
        info!("listening server socket on {}", address);

    }

    pub fn connect_listener(
        mut commands: Commands,
        mut server_event: EventReader<ServerEvent>,
        mut client_pool: ResMut<LudoOnlineClientPool>,
        server_transport: Res<NetcodeServerTransport>,
        game_object: Res<LudoGameObject>,
        configuration: Res<LudoGameConfiguration>,
        mut server: ResMut<RenetServer>
    ) {
        for server_event in server_event.read() {
            match server_event {
                ServerEvent::ClientConnected { client_id } => {
                    if game_object.state == LudoGameState::Waiting {
                        if client_pool.ludo_clients_pool.len() < configuration.max_players_to_start as usize {
                            client_pool.ludo_clients_pool.insert(*client_id, Vec::new());
                            if let Some(address) = server_transport.client_addr(client_id.clone()) {
                                info!("new client connected id: {0}, address: {1}", client_id, address);
                            }
                            client_pool.ludo_clients_pool.get_mut(&*client_id).unwrap().push(Pair::new("client.handshake".to_string(), Box::new(false)));
                            commands.spawn(HandshakeTimer(Timer::new(Duration::from_millis(500), TimerMode::Once), client_id.clone()));
                        } else {
                            server.disconnect(client_id.clone());
                        }
                    } else {
                        server.disconnect(client_id.clone());
                    }
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    client_pool.ludo_clients_pool.remove(*&client_id);
                    info!("client disconnected: {0}. Because of: {1}", client_id, reason);
                }
            }
        }
    }

    pub fn disable_application_system(mut event_reader: EventReader<AppExit>, profiles: Query<&LudoGameProfileData>) {
        event_reader.read().for_each(move |_exit_event| {
            info!("exit event.");
            let mut list = vec![];
            profiles.iter().for_each(|profile| {
                list.push(profile.clone());
            });
            LudoGameProfileData::load_to_file("profiles.json", list).expect("unable to save ludo game profiles!");
            info!("disabled ludo game profiles");
        })
    }

    pub fn ludo_game_update(
        configuration: Res<LudoGameConfiguration>,
        mut game_object: ResMut<LudoGameObject>,
        online_profile_pool: Res<LudoOnlineClientPool>,
        mut data_pool: Query<&mut LudoGameProfileData>,
        mut server: ResMut<RenetServer>
    ) {
        if game_object.state == LudoGameState::Waiting {
            if online_profile_pool.ludo_clients_pool.len() >= configuration.min_players_to_start as usize {
                for client in online_profile_pool.ludo_clients_pool.iter() {
                    let start_packet = LudoGameOutcomeGameStartPacket::new().into_string::<LudoGameOutcomeGameStartPacket>().expect("unable to parse packet into string");
                    server.send_message(client.0.clone(), DefaultChannel::ReliableOrdered, start_packet);
                }
                info!("ludo game is starting...");
            }
        }
    }

}

impl LudoOnlineClientPool {
    pub fn get_information<T: 'static>(&self, client: &ClientId, info: &str) -> Option<&T> {
        if let Some(profile_cache) = self.ludo_clients_pool.get(client) {
            for item in profile_cache {
                if item.0 == info {
                    let cast = item.1.as_ref().downcast_ref::<T>().expect("unable to cast target information");
                    return Some(cast);
                }
            }
            None
        } else { None }
    }

    pub fn set_information<T>(&mut self, client: &ClientId, info: &str, some: T) where T: Any + Sync + Send {
        if let Some(profile_cache) = self.ludo_clients_pool.get_mut(client) {
            let mut i = 0;
            let mut bool = false;
            for item in &mut *profile_cache {
                if item.0 == info {
                    bool = true;
                    break;
                }
                i+=1;
            }
            if bool {
                profile_cache.insert(i, Pair::new(info.to_string(), Box::new(some)));
            } else {
                profile_cache.push(Pair::new(info.to_string(), Box::new(some)));
            }
        }
    }

    pub fn is_handshaked(&self, client: &ClientId) -> bool {
        self.get_information::<bool>(client, "client.handshaked").is_some()
    }
}
