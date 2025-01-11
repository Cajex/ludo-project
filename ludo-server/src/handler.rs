use bevy::log::warn;
use crate::server::LudoClientPool;
use bevy::prelude::{info, Res, ResMut};
use bevy::utils::info;
use bevy_renet::renet::{DefaultChannel, RenetServer};
use ludo_commons::{packets, security, LudoPacket};
use ludo_commons::packets::LudoGameIncomeHandshakePacket;

pub fn handle_client_income(mut server: ResMut<RenetServer>, mut ludo_client_pool: ResMut<LudoClientPool>) {
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
                    for mut items in client_data.iter_mut() {
                        if items.0.eq("client.handshake") {
                            if let Some(value) = items.1.downcast_ref::<bool>() {
                                if !*value {
                                    let raw_data = String::from_utf8_lossy(&message).to_string();
                                    let handshake_packet = <LudoGameIncomeHandshakePacket as LudoPacket>::make_packet::<LudoGameIncomeHandshakePacket>(raw_data.clone());
                                    if let Ok(handshake_packet) = handshake_packet {
                                        if handshake_packet.key.eq(&security::SECRET_KEY) {
                                            info!("client successfully handshake: {}", client_id);
                                            items.1 = Box::new(true);
                                        } else {
                                            server.disconnect(client_id);
                                            clients_to_remove.push(client_id);
                                            warn!("wrong security key from: {}", client_id);
                                        }
                                    } else {
                                        server.disconnect(client_id);
                                        clients_to_remove.push(client_id);
                                        warn!("wrong packet received from: {0} content: [{1}].", client_id, raw_data);
                                        info!("right packet content would be: [{}].", LudoGameIncomeHandshakePacket::new(security::SECRET_KEY).into_string().unwrap());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !clients_to_remove.is_empty() {
        for client_id in clients_to_remove {
            ludo_client_pool.ludo_clients_pool.remove(&client_id);
        }
        info!("current connected clients: {0} and registered: {1}", server.connected_clients(), ludo_client_pool.ludo_clients_pool.len());
    }
}