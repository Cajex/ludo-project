use std::any::Any;
use bevy::prelude::*;
use bevy::utils::info;
use bevy_renet::netcode::NetcodeServerTransport;
use bevy_renet::renet::{ClientId, DefaultChannel, RenetServer};
use ludo_commons::{LudoPacket, Pair};
use ludo_commons::packets::LudoGameOutcomeHandshakeCallbackPacket;
use crate::server::LudoClientPool;

#[derive(Component)]
pub struct HandshakeTimer(pub Timer, pub ClientId);

pub fn update_handshake_timer(time: Res<Time>, mut timer: Query<&mut HandshakeTimer>, mut client_pool: ResMut<LudoClientPool>, mut server: ResMut<RenetServer>, server_transport: Res<NetcodeServerTransport>) {
    let mut clients_to_remove = Vec::new();
    let mut successfully_removed: Option<Pair<i32, ClientId>> = None;
    timer.iter_mut().for_each(|mut timer| {
        if timer.0.tick(time.delta()).finished() {
            let client_id = timer.1.clone();
            let client_handshake = client_pool.ludo_clients_pool.get(&client_id);
            if let Some(client_handshake) = client_handshake {
                let mut i: i32 = -1;
                for items in client_handshake {
                    if items.0.eq("client.handshake") {
                        if let Some(value) = items.1.downcast_ref::<bool>() {
                            if *value {
                                successfully_removed = Some(Pair::new(i+1, client_id));
                                server.send_message(
                                    client_id.clone(),
                                    DefaultChannel::ReliableOrdered,
                                    LudoGameOutcomeHandshakeCallbackPacket::new().into_string().expect("unable to serialize #(LudoGameOutcomeHandshakeCallbackPacket) packet!")
                                );
                                if let Some(address) = server_transport.client_addr(client_id) {
                                    info!("Server handshake successfully! {}", address);
                                }
                            } else {
                                server.disconnect(client_id.clone());
                                clients_to_remove.push(client_id);
                                info!("no answer from client. id={}", client_id);
                                info("client disconnected because of no successfully handshake.");
                            }
                        } else {
                            server.disconnect(client_id.clone());
                            clients_to_remove.push(client_id);
                            info!("no answer from client. id={}", client_id);
                            info("client disconnected.");
                            error!("Expected boolean for this handshake query!");
                            panic!("unexpected behavior!");
                        }
                    } else {
                        for x in client_pool.ludo_clients_pool.get(&client_id).unwrap().iter() {
                            warn!("{}", x.0)
                        }
                        let result = client_pool.ludo_clients_pool.get(&client_id).unwrap().iter().filter(|item| -> bool {
                            items.0.eq("client.handshaked") == true
                        }).collect::<Vec<&Pair<String, Box<dyn Any + Sync + Send>>>>();
                        if result.is_empty() {
                            let result = client_pool.ludo_clients_pool.get(&client_id).unwrap().iter().filter(|item| -> bool {
                                items.0.eq("client.handshake") == true
                            }).collect::<Vec<&Pair<String, Box<dyn Any + Sync + Send>>>>();
                            if result.is_empty() {
                                server.disconnect(client_id.clone());
                                clients_to_remove.push(client_id);
                                info!("no answer from client. id={}", client_id);
                                info("client disconnected because of no entry.");
                            }
                        }
                    }
                    i+=1;
                }
            }
        }
    });
    if let Some(pair) = successfully_removed {
        if pair.0 != -1 {
            let mut client_pool = client_pool.ludo_clients_pool.get_mut(&pair.1).unwrap();
            client_pool.remove(pair.0.try_into().unwrap());
            client_pool.push(Pair::new("client.handshaked".to_string(), Box::new(true)));
            info!("client is marked as handshaked: {}", pair.1);
            info!("current connected clients: {0} and registered: {1}", server.connected_clients(), client_pool.len());
        }
    } else {
        if !clients_to_remove.is_empty() {
            for client_id in clients_to_remove {
                client_pool.ludo_clients_pool.remove(&client_id);
            }
            info!("current connected clients: {0} and registered: {1}", server.connected_clients(), client_pool.ludo_clients_pool.len());
        }
    }
}
