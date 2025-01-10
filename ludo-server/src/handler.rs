use bevy::log::warn;
use crate::server::LudoClientPool;
use bevy::prelude::{info, Res, ResMut};
use bevy_renet::renet::{DefaultChannel, RenetServer};
use ludo_commons::{packets, security};
use ludo_commons::packets::LudoGameIncomeHandshakePacket;

pub fn handle_client_income(mut server: ResMut<RenetServer>, mut ludo_client_pool: ResMut<LudoClientPool>) {
    server.clients_id_iter().for_each(|client_id| {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            match ludo_client_pool.ludo_clients_pool.get_mut(&client_id) {
                None => {
                    server.disconnect(client_id);
                    warn!("Ludo client not registered: {}", client_id);
                    info!("Connection closed: {}", client_id);
                }
                Some(client_data) => {
                    for items in client_data.iter_mut() {
                        if items.0.eq("client.handshake") {
                            if let Ok(value) = <Box<dyn std::any::Any + Send + Sync> as Clone>::clone(&items.1).downcast::<bool>() {
                                if !*value {
                                    let raw_data = String::from_utf8_lossy(&message.clone());
                                    let handshake_packet = <LudoGameIncomeHandshakePacket as packets::LudoPacket>::make_packet::<LudoGameIncomeHandshakePacket>(raw_data.to_string());
                                    if let Ok(handshake_packet) = handshake_packet {
                                        if handshake_packet.key.eq(&security::SECRET_KEY) {
                                            info!("client successfully handshake: {}", client_id);
                                            items.1 = Box::new(true);
                                        } else {
                                            server.disconnect(client_id);
                                            ludo_client_pool.ludo_clients_pool.remove(&client_id);
                                            warn!("wrong security key from: {}", client_id);
                                        }
                                    } else {
                                        server.disconnect(client_id);
                                        ludo_client_pool.ludo_clients_pool.remove(&client_id);
                                        warn!("wrong packet received from: {}", client_id);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}