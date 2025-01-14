use crate::server::LudoOnlineClientPool;
use bevy::prelude::{Component, Query, Res, ResMut, Time};
use bevy::time::Timer;
use bevy_renet::renet::{DefaultChannel, RenetServer};
use ludo_commons::game::LudoGameProfile;
use ludo_commons::packets::LudoGameOutcomePlayerProfilesPacket;
use ludo_commons::LudoPacket;

#[derive(Component)]
pub struct LudoProfilesInfoTimer(pub Timer);

pub fn handle_client_outcome_profiles_info(time: Res<Time>, mut timer: Query<&mut LudoProfilesInfoTimer>, pool: Res<LudoOnlineClientPool>, mut server: ResMut<RenetServer>) {
    if !pool.ludo_clients_pool.is_empty() {
        if timer.get_single_mut().unwrap().0.tick(time.delta()).just_finished() {
            let mut online_players = vec![];
            pool.ludo_clients_pool.keys().for_each(|client| {
                if pool.is_handshaked(client) {
                    if let Some(profile) = pool.get_information::<LudoGameProfile>(client, "server.profile") {
                        online_players.push(profile.clone());
                    }
                }
            });
            let packet = LudoGameOutcomePlayerProfilesPacket::new(online_players).into_string::<LudoGameOutcomePlayerProfilesPacket>().unwrap();
            pool.ludo_clients_pool.keys().for_each(|client| {
                server.send_message(client.clone(), DefaultChannel::ReliableOrdered, packet.clone());
            });
        }
    }
}