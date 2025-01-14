use std::fs::File;
use bevy::prelude::*;
use std::net::UdpSocket;
use std::thread;
use std::time::{Duration, SystemTime};
use bevy::asset::AssetContainer;
use bevy::image;
use bevy::render::render_resource::Texture;
use bevy::utils::info;
use bevy::window::PrimaryWindow;
use bevy::winit::{WinitSettings, WinitWindows};
use bevy_renet::netcode::{ClientAuthentication, NetcodeClientTransport};
use bevy_renet::renet::{ConnectionConfig, RenetClient};
use imageun::ImageReader;
use winit::window::Icon;
use ludo_commons::game::LudoGameProfile;
use crate::{handler, handshake, interface};
use crate::handler::handle_server_outcome_system;
use crate::interface::LudoClientGameState;

#[derive(Default)]
pub struct LudoClientPlugin {
}

#[derive(Event)]
pub struct LudoClientChangeStateEvent;

#[derive(Resource)]
pub struct LudoClientConnectionStable(pub Option<bool>);

#[derive(Component)]
pub struct LudoClientConnectionInfo(pub String);

#[derive(Resource)]
pub struct LudoClientCachedOnlinePlayersProfiles(pub Vec<LudoGameProfile>);

impl Plugin for LudoClientPlugin {
    fn build(&self, application: &mut App) {
        application
            .add_event::<LudoClientChangeStateEvent>()
            .insert_resource(LudoClientConnectionStable(None))
            .add_systems(PreStartup, Self::enable_system)
            .add_systems(Update, handle_server_outcome_system.run_if(in_state(LudoClientGameState::WaitingMenu)))
            .insert_resource(LudoClientCachedOnlinePlayersProfiles(vec![]));
    }
}

impl LudoClientPlugin {
    pub fn enable_system(mut commands: Commands, window: NonSend<WinitWindows>, main_window: Query<Entity, With<PrimaryWindow>>) {
        let result = LudoGameProfile::load_from_file("game-profile.json").expect("unable to load ludo game profile");
        info!("load game profile...");
        commands.spawn(result);

        let Some(primary) = window.get_window(main_window.single()) else {return};
        let image = ImageReader::open("ludo-client/assets/client.image.icon.png").unwrap().decode().unwrap().into_rgba8();
        let rgba = image.as_raw();
        primary.set_window_icon(Some(Icon::from_rgba(rgba.clone(), image.width(), image.height()).unwrap()));
    }

    pub fn connect_client_system(
        mut commands: Commands,
        connection_information: Query<&LudoClientConnectionInfo>,
        mut connection_stable: ResMut<LudoClientConnectionStable>,
    ) {
        connection_information.iter().for_each(|connection_info| {
            let client = RenetClient::new(ConnectionConfig::default());
            if let Ok(address) = connection_info.0.parse() {
                let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                let authentication = ClientAuthentication::Unsecure {
                    server_addr: address,
                    client_id: current_time.as_millis() as u64,
                    user_data: None,
                    protocol_id: 0,
                };
                let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
                let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
                commands.insert_resource(client);
                commands.insert_resource(transport);
                info!("Starting Ludo client tasks on: {}", address);
                commands.insert_resource(State::new(LudoClientGameState::WaitingMenu));
                let system_id_handshake = commands.register_system(handshake::commit_handshake_system);
                commands.run_system(system_id_handshake);
                let _ = connection_stable.0.insert(true);
                let system_id_interface = commands.register_system(interface::LudoClientUserInterfacePlugin::client_change_state_listener);
                commands.run_system(system_id_interface);
            } else {
                error!("connection not stable!");
                let _ = connection_stable.0.insert(false);
                commands.insert_resource(State::new(LudoClientGameState::ServerPingMenu));
            }
        });
    }

}