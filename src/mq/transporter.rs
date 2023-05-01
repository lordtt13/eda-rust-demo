use crate::mq::message::Message;

pub trait Transporter {
    fn send(&mut self, queue_name: &str, message: &Message, channel_id: Option<u16>) -> Result<(), std::io::Error>;
    fn recv(&mut self, queue_name: &str, channel_id: Option<u16>) -> Result<Message, std::io::Error>;
}
