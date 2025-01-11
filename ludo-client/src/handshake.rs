use bevy::log::info;
use bevy::prelude::ResMut;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use ludo_commons::{packets, security, LudoPacket};

pub fn commit_handshake_system(mut client: ResMut<RenetClient>) {
    let handshake_packet = packets::LudoGameIncomeHandshakePacket::new(security::SECRET_KEY);
    if let Ok(handshake_packet) = handshake_packet.into_string() {
        client.send_message(DefaultChannel::ReliableOrdered, handshake_packet);
        info!("handshake packet sent!");
    }
}