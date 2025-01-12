use bevy::log::{info, warn};
use bevy::prelude::{error, Query, ResMut};
use bevy_renet::renet::{DefaultChannel, RenetClient};
use ludo_commons::game::LudoGameProfile;
use ludo_commons::LudoPacket;
use ludo_commons::packets::{LudoGameIncomeProfilePacket, LudoGameOutcomeDisconnectPacket, LudoGameOutcomeGameStartPacket, LudoGameOutcomeHandshakeCallbackPacket, LudoGameOutcomeProfilePacket};

pub fn handle_server_outcome_system(mut client: ResMut<RenetClient>, profile: Query<&LudoGameProfile>) {
    let outcome_message = client.receive_message(DefaultChannel::ReliableOrdered);
    if let Some(outcome_message) = outcome_message {
        if let Ok(_handshake_packet_outcome) = LudoGameOutcomeHandshakeCallbackPacket::make_packet::<LudoGameOutcomeHandshakeCallbackPacket>(String::from_utf8_lossy(&outcome_message).to_string()) {
            info!("handshake successfully received!");
            info!("requested profile data...");
            let packet = LudoGameIncomeProfilePacket::new(profile.get_single().expect("no profile provided by the system!").clone());
            client.send_message(DefaultChannel::ReliableOrdered, packet.into_string::<LudoGameIncomeProfilePacket>().expect("unable to parse #(LudoGameIncomeProfilePacket) to raw!"));
        } else {
            if let Ok(profile_packet_outcome) = LudoGameOutcomeProfilePacket::make_packet::<LudoGameOutcomeProfilePacket>(String::from_utf8_lossy(&outcome_message).to_string()) {
                info!("response successfully received!");
                info!("data: {:?}", profile_packet_outcome);
            } else {
                if let Ok(disconnect_packet) = LudoGameOutcomeDisconnectPacket::make_packet::<LudoGameOutcomeDisconnectPacket>(String::from_utf8_lossy(&outcome_message).to_string()) {
                    error!("disconnection received: {}!", disconnect_packet.reason);
                } else {
                    if let Ok(game_start_packet) = LudoGameOutcomeGameStartPacket::make_packet::<LudoGameOutcomeGameStartPacket>(String::from_utf8_lossy(&outcome_message).to_string()) {
                        info!("game is starting...!");

                    }
                }
            }
        }
    }
}