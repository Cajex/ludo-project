use std::any::type_name;
use std::fmt::Display;
use serde::{Deserialize, Serialize};
use anyhow::{Error, Result};
use derive_new::new;
use crate::game::{LudoGameProfile, LudoGameProfileData};
use crate::LudoPacketType::{Income, Outcome};

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
    fn into_string<T>(&self) -> Result<String> where T: LudoPacket {
        Ok(format!("type: {0}, data: #[{1}]#", std::any::type_name::<T>(), serde_json::to_string(&self)?))
    }

    fn make_packet<T>(buf: String) -> Result<T> where T: LudoPacket {
        if buf.contains(type_name::<T>()) {
            serde_json::from_str(regex(buf)?.as_str()).map_err(|e| e.into())
        } else {
            Err(Error::msg(format!("Invalid packet type: {}", buf)))
        }
    }
}

fn regex<T>(input: T) -> Result<T> where T: Display + ToString + for<'a> From<&'a str> {
    let input = input.to_string();
    if let Some(start) = input.find("data: #[") {
        let start = start + "data: #[".len();
        if let Some(end) = input[start..].find("]#") {
            let data = &input[start..start + end];
            Ok(data.into())
        } else {
            Err(Error::msg(format!("invalid input: {}", input)))
        }
    } else {
        Err(Error::msg(format!("invalid input: {}", input)))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct LudoGameIncomeHandshakePacket {
    pub key: [u8; 32],
}

impl LudoPacket for LudoGameIncomeHandshakePacket {
    fn packet_type(&self) -> LudoPacketType {
        Income
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct LudoGameOutcomeHandshakeCallbackPacket {

}

impl LudoPacket for LudoGameOutcomeHandshakeCallbackPacket {
    fn packet_type(&self) -> LudoPacketType {
        Outcome
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct LudoGameIncomeProfilePacket {
    pub profile: LudoGameProfile,
}

impl LudoPacket for LudoGameIncomeProfilePacket {
    fn packet_type(&self) -> LudoPacketType {
        Income
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct LudoGameOutcomeProfilePacket {
    pub data: LudoGameProfileData,
}

impl LudoPacket for LudoGameOutcomeProfilePacket {
    fn packet_type(&self) -> LudoPacketType {
        Outcome
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct LudoGameOutcomeDisconnectPacket {
    pub reason: String,
}

impl LudoPacket for LudoGameOutcomeDisconnectPacket {
    fn packet_type(&self) -> LudoPacketType {
        Outcome
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct LudoGameOutcomeGameStartPacket {

}

impl LudoPacket for LudoGameOutcomeGameStartPacket {
    fn packet_type(&self) -> LudoPacketType {
        Outcome
    }
}