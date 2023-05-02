use crate::packet_proto::packets::PacketType;

#[derive(Debug, Clone)]
pub struct Topic {
    pub topic_id: String,
    pub topic: PacketType
}