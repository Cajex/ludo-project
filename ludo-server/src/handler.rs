use bevy::log::warn;
use crate::server::LudoOnlineClientPool;
use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetServer};
use ludo_commons::{security, LudoPacket, Pair};
use ludo_commons::game::LudoGameProfileData;
use ludo_commons::packets::{LudoGameIncomeHandshakePacket, LudoGameIncomeProfilePacket, LudoGameOutcomeProfilePacket};

pub fn handle_client_income(
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
    mut ludo_client_pool: ResMut<LudoOnlineClientPool>,
    mut profile_data: Query<&mut LudoGameProfileData>
) {
    let mut clients_to_remove = Vec::new();
    let client_ids: Vec<_> = server.clients_id_iter().collect();
    for client_id in client_ids {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            match ludo_client_pool.ludo_clients_pool.get_mut(&client_id) {
                None => {
                    server.disconnect(client_id);
                    warn!("Ludo client not registered: {}", client_id);
                    info!("Connection closed: {}", client_id);
                }
                Some(client_data) => {
                    let raw_data = String::from_utf8_lossy(&message).to_string();
                    if let Ok(handshake_packet) = <LudoGameIncomeHandshakePacket as LudoPacket>::make_packet::<LudoGameIncomeHandshakePacket>(raw_data.clone()) {
                    for items in client_data.iter_mut() {
                        if items.0.eq("client.handshake") {
                            if let Some(value) = items.1.downcast_ref::<bool>() {
                                if !*value {
                                    if handshake_packet.key.eq(&security::SECRET_KEY) {
                                        info!("client successfully handshake: {}", client_id);
                                        items.1 = Box::new(true);
                                    } else {
                                        server.disconnect(client_id);
                                        clients_to_remove.push(client_id);
                                        warn!("wrong security key from: {}", client_id);
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if let Ok(profile_income_packet) = <LudoGameIncomeProfilePacket as LudoPacket>::make_packet::<LudoGameIncomeProfilePacket>(raw_data.clone()) {
                        ludo_client_pool.ludo_clients_pool.get_mut(&client_id).unwrap().push(Pair::new("server.profile".to_string(), Box::new(profile_income_packet.profile.clone())));
                        let mut found = false;
                        profile_data.iter_mut().for_each(|profile_data| {
                            if profile_data.unique_id.eq(&profile_income_packet.profile.unique_id.clone()) {
                                let packet = LudoGameOutcomeProfilePacket::new(profile_data.clone());
                                server.send_message(client_id, DefaultChannel::ReliableOrdered, packet.into_string::<LudoGameOutcomeProfilePacket>().unwrap());
                                found = true;
                                info!("Client successfully sent profile data: {:?}", profile_income_packet.profile);
                            }
                        });
                        if !found {
                            info!("Client profile not found in database: {:?}", profile_income_packet.profile.unique_id);
                            let profile_data = LudoGameProfileData {
                                unique_id: profile_income_packet.profile.unique_id.clone(),
                                points: 0,
                            };
                            commands.spawn(profile_data.clone());
                            let packet = LudoGameOutcomeProfilePacket::new(profile_data);
                            server.send_message(client_id, DefaultChannel::ReliableOrdered, packet.into_string::<LudoGameOutcomeProfilePacket>().unwrap());
                            info!("Client successfully sent profile data: {:?}", profile_income_packet.profile);
                            info!("a new profile data were created!")
                        }
                    }
                }
            }
        }
    }
}
if ! clients_to_remove.is_empty() {
        for client_id in clients_to_remove {
            ludo_client_pool.ludo_clients_pool.remove(&client_id);
        }
        info!("current connected clients: {0} and registered: {1}", server.connected_clients(), ludo_client_pool.ludo_clients_pool.len());
    }
}