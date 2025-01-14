use bevy::log::info;
use bevy::prelude::{Component, Event, EventReader, ResMut};
use bevy_renet::renet::{DefaultChannel, RenetClient};
use ludo_commons::{security, LudoPacket};
use ludo_commons::packets::LudoGameIncomeHandshakePacket;


pub fn commit_handshake_system(mut client: ResMut<RenetClient>) {
    let handshake_packet = LudoGameIncomeHandshakePacket::new(security::SECRET_KEY);
    if let Ok(handshake_packet) = handshake_packet.into_string::<LudoGameIncomeHandshakePacket>() {
        client.send_message(DefaultChannel::ReliableOrdered, handshake_packet);
        info!("handshake packet sent!");
    }
}