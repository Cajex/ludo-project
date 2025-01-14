use std::fs::File;
use bevy::prelude::*;
use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::image;
use bevy::render::render_resource::Texture;
use bevy::window::PrimaryWindow;
use bevy::winit::{WinitSettings, WinitWindows};
use bevy_renet::netcode::{ClientAuthentication, NetcodeClientTransport};
use bevy_renet::renet::{ConnectionConfig, RenetClient};
use imageun::ImageReader;
use winit::window::Icon;
use ludo_commons::game::LudoGameProfile;
use crate::{handler, handshake};

#[derive(Default)]
pub struct LudoClientPlugin {
}

#[derive(Event)]
pub struct LudoClientChangeStateEvent;

impl Plugin for LudoClientPlugin {
    fn build(&self, application: &mut App) {
        application
            .add_event::<LudoClientChangeStateEvent>()
            .add_systems(PreStartup, Self::enable_system)
            .add_systems(Startup, Self::connect_client_system)
            .add_systems(PostStartup, handshake::commit_handshake_system)
            .add_systems(Update, handler::handle_server_outcome_system);
    }
}

impl LudoClientPlugin {
    pub fn enable_system(mut commands: Commands, window: NonSend<WinitWindows>, main_window: Query<Entity, With<PrimaryWindow>>, asset_server: Res<AssetServer>, mut texture_assets: ResMut<Assets<Image>>) {
        let result = LudoGameProfile::load_from_file("game-profile.json").expect("unable to load ludo game profile");
        info!("load game profile...");
        commands.spawn(result);

        let Some(primary) = window.get_window(main_window.single()) else {return};
        let image = ImageReader::open("ludo-client/assets/client.image.icon.png").unwrap().decode().unwrap().into_rgba8();
        let rgba = image.as_raw();
        primary.set_window_icon(Some(Icon::from_rgba(rgba.clone(), image.width(), image.height()).unwrap()));
    }

    pub fn connect_client_system(mut commands: Commands) {
        let client = RenetClient::new(ConnectionConfig::default());
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
        commands.insert_resource(client);
        commands.insert_resource(transport);
        info!("Starting Ludo client tasks.");
    }

}