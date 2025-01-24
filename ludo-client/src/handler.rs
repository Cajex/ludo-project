use bevy::log::{info, warn};
use bevy::prelude::{error, Commands, Query, Res, ResMut};
use bevy_renet::renet::{DefaultChannel, RenetClient};
use ludo_commons::game::{LudoGameConfiguration, LudoGameProfile};
use ludo_commons::LudoPacket;
use ludo_commons::packets::{LudoGameIncomeProfilePacket, LudoGameOutcomeDisconnectPacket, LudoGameOutcomeGameStartPacket, LudoGameOutcomeHandshakeCallbackPacket, LudoGameOutcomePlayerProfilesPacket, LudoGameOutcomeProfilePacket, LudoGameOutcomeServerDataPacket};
use crate::client::LudoClientCachedOnlinePlayersProfiles;
use crate::{game, interface};

pub fn handle_server_outcome_system(mut commands: Commands, mut client: ResMut<RenetClient>, profile: Query<&LudoGameProfile>, mut cached_profiles: ResMut<LudoClientCachedOnlinePlayersProfiles>) {
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
                    if let Ok(_game_start_packet) = LudoGameOutcomeGameStartPacket::make_packet::<LudoGameOutcomeGameStartPacket>(String::from_utf8_lossy(&outcome_message).to_string()) {
                        let system_id = commands.register_system(game::client_load_game);
                        commands.run_system(system_id);
                    } else {
                        if let Ok(communication_profiles_packet) = LudoGameOutcomePlayerProfilesPacket::make_packet::<LudoGameOutcomePlayerProfilesPacket>(String::from_utf8_lossy(&outcome_message).to_string()) {
                            cached_profiles.0 = communication_profiles_packet.list;
                        } else {
                            if let Ok(data_packet) = LudoGameOutcomeServerDataPacket::make_packet::<LudoGameOutcomeServerDataPacket>(String::from_utf8_lossy(&outcome_message).to_string()) {
                                commands.insert_resource(LudoGameConfiguration {
                                    min_players_to_start: data_packet.min_players as u8,
                                    max_players_to_start: 4,
                                });
                                let system_id = commands.register_system(interface::LudoClientUserInterfacePlugin::client_load_minimum_players_system);
                                commands.run_system(system_id);
                            }
                        }
                    }
                }
            }
        }
    }
}