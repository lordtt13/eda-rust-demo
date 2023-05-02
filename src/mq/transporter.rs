use crate::{mq::topic::Topic, packet_proto::packets::PacketDiscover};

pub trait Transporter {
    fn subscribe(&self, node_id: Option<String>, topic: Topic) -> Result<(), std::io::Error>;
    fn discover(&self, discover_packet: PacketDiscover) -> Result<(), std::io::Error>;
    fn subscribe_all(&self) -> Result<(), std::io::Error>;
}
