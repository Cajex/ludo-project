use bevy::log::info;
use bevy::prelude::ResMut;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use ludo_commons::LudoPacket;
use ludo_commons::packets::LudoGameOutcomeHandshakeCallbackPacket;

pub fn handle_server_outcome_system(mut client: ResMut<RenetClient>) {
    let outcome_message = client.receive_message(DefaultChannel::ReliableOrdered);
    if let Some(outcome_message) = outcome_message {
        if let Ok(_handshake_packet_outcome) = LudoGameOutcomeHandshakeCallbackPacket::make_packet::<LudoGameOutcomeHandshakeCallbackPacket>(String::from_utf8_lossy(&outcome_message).to_string()) {
            info!("handshake successfully received!");
        }
    }
}