use serde::{Deserialize, Serialize};
use anyhow::Result;
use derive_new::new;

pub enum LudoPacketType {
    /* packets sent from one or more clients to the server. */
    Income,
    /* packets sent from the server to one or more clients. */
    Outcome,
}

/* Packet framework for all packets in the game. Incoming and outgoing. */
pub trait LudoPacket: Serialize + for<'de> Deserialize<'de> {
    /* only method to be implemented to determine the type of the packet. */
    fn packet_type(&self) -> LudoPacketType;

    //noinspection ALL
    fn into_string(&self) -> Result<String> {
        serde_json::to_string(&self).map_err(|e| e.into())
    }

    fn make_packet<T>(buf: String) -> Result<T> where T: LudoPacket {
        serde_json::from_str(&buf).map_err(|e| e.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct LudoGameIncomeHandshakePacket {
    pub key: [u8; 32]
}

impl LudoPacket for LudoGameIncomeHandshakePacket {
    fn packet_type(&self) -> LudoPacketType {
        LudoPacketType::Income
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct LudoGameOutcomeHandshakeCallbackPacket {
}

impl LudoPacket for LudoGameOutcomeHandshakeCallbackPacket {
    fn packet_type(&self) -> LudoPacketType {
        LudoPacketType::Outcome
    }
}