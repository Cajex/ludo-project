use bevy::prelude::*;
use bevy::utils::info;
use bevy_renet::renet::{ClientId, RenetServer};
use crate::server::LudoClientPool;

#[derive(Component)]
pub struct HandshakeTimer(pub Timer, pub ClientId);

pub fn update_handshake_timer(time: Res<Time>, mut timer: Query<&mut HandshakeTimer>, mut client_pool: ResMut<LudoClientPool>, mut server: ResMut<RenetServer>) {
    timer.iter_mut().for_each(|mut timer| {
        if timer.0.tick(time.delta()).finished() {
            let client_id = timer.1.clone();
            let client_handshake = client_pool.ludo_clients_pool.get(&client_id).expect("no data for this client!");
            for items in client_handshake {
                if items.0.eq("client.handshake") {
                    if let Some(value) = items.1.downcast_ref::<bool>() {
                        if *value {
                            info!("Server handshake successfully! {}", client_id);
                        } else {
                            server.disconnect(client_id.clone());
                            info!("no answer from client. id={}", client_id);
                            info("client disconnected.");
                        }
                    } else {
                        server.disconnect(client_id.clone());
                        info!("no answer from client. id={}", client_id);
                        info("client disconnected.");
                        error!("Expected boolean for this handshake query!");
                        panic!("unexpected behavior!");
                    }
                } else {
                    server.disconnect(client_id.clone());
                    info!("no answer from client. id={}", client_id);
                    info("client disconnected.");
                }
            }
        }
    });
}
