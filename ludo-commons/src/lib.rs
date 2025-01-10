pub mod security;
pub mod packets;

use derive_new::new;
pub use crate::security::SECRET_KEY;
pub use crate::packets::{LudoPacket, LudoPacketType};

#[derive(new)]
pub struct Pair<F, S>(pub F, pub S);