use amiquip::{Connection, Channel, Exchange, Publish, ConsumerOptions, QueueDeclareOptions, ConsumerMessage};
use cuid;
use crate::mq::transporter::Transporter;
use crate::mq::message::Message;

pub struct RabbitMqTransporter {
    connection: Connection,
}

impl RabbitMqTransporter {
  pub fn new(url: &str) -> Result<RabbitMqTransporter, std::io::Error> {
      let connection = Connection::insecure_open(url).unwrap_or_else(
          |err| {
              panic!("Connection error: {}", err);
          }
      );
      Ok(Self { connection })
  }

  fn create_channel(&mut self, channel_id: Option<u16>) -> Result<Channel, std::io::Error> {
      let channel = self.connection.open_channel(channel_id).unwrap_or_else(
          |err| {
              panic!("Error opening channel: {}", err);
          }
      );
      Ok(channel)
  }
}

impl Transporter for RabbitMqTransporter {
    fn send(&mut self, queue_name: &str, message: &Message, channel_id: Option<u16>) -> Result<(), std::io::Error> {
        let channel = self.create_channel(channel_id).unwrap();
        let exchange = Exchange::direct(&channel);
        exchange.publish(Publish::new(message.body.as_bytes(), queue_name)).unwrap_or_else(
            |err| {
                panic!("Error publishing message: {}", err);
            }
        );
        Ok(())
    }

    fn recv(&mut self, queue_name: &str, channel_id: Option<u16>) -> Result<Message, std::io::Error> {
        let channel = self.create_channel(channel_id).unwrap();
        let queue = channel.queue_declare(queue_name, QueueDeclareOptions::default()).unwrap_or_else(
            |err| {
                panic!("Error declaring queue: {}", err);
            }
        );
        let consumer = queue.consume(ConsumerOptions::default()).unwrap_or_else(
            |err| {
                panic!("Error consuming message: {}", err);
            }
        );
        let message = consumer.receiver().recv().unwrap_or_else(
            |err| {
                panic!("Error receiving message: {}", err);
            }
        );
        
        match message {
            ConsumerMessage::Delivery(message) => {
                let body = String::from_utf8(message.body).unwrap();
                let message_recv = Message {
                    trace_id: cuid::cuid2(),
                    body,
                };
                Ok(message_recv)
            },
            other => {
                panic!("Consumer ended unexpectedly: {:?}", other);
            }
        }
    }
}