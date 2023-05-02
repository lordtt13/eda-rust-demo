use amiquip::{Connection, ConsumerOptions, QueueDeclareOptions, ConsumerMessage, Publish, Exchange};
use protobuf::Message;
use crate::{mq::{transporter::Transporter, topic::Topic}, packet_proto::packets::PacketDiscover};
use cuid;
use crate::packet_proto::packets::{PacketType};
use std::{sync::{Arc, Mutex}};

pub struct RabbitMqTransporter {
    node_id: String,
    connection: Arc<Mutex<Connection>>,
}

impl RabbitMqTransporter {
    pub fn new(url: String) -> Result<Self, std::io::Error> {
        // connect to RabbitMQ
        let connection = Connection::insecure_open(&url).unwrap_or_else(
            |e| {
                panic!("Error connecting to RabbitMQ: {}", e);
            }
        );
        let node_id = std::env::var("NODE_ID").unwrap_or_else(|_| cuid::cuid2());
        Ok(
            RabbitMqTransporter {
                node_id,
                connection: Arc::new(Mutex::new(connection))
            }
        )
    }
}

impl Transporter for RabbitMqTransporter {    
    fn subscribe_all(&self) -> Result<(), std::io::Error> {
        // subscribe to all topics
        let all_topics = vec![
            Topic {
                topic: PacketType::PACKET_DISCONNECT,
                topic_id: "DISCONNECT".to_string()
            }
        ];
        for topic in all_topics.iter() {
            let _ = &self.subscribe(Some(self.node_id.clone()), topic.clone());
        }
        Ok(())
    }

    fn subscribe(&self, node_id: Option<String>, topic: Topic) -> Result<(), std::io::Error> {
        let connection = self.connection.clone();
        let channel = connection.lock().unwrap().open_channel(None).unwrap_or_else(
            |e| {
                panic!("Error opening RabbitMQ channel: {}", e);
            }
        );
        std::thread::spawn(move || -> Result<(), std::io::Error> {
            let queue = channel.queue_declare(
                format!("ATOM.{}.{}", topic.topic_id, node_id.unwrap_or_else(|| "".to_string())),
                QueueDeclareOptions {
                    durable: true,
                    ..QueueDeclareOptions::default()
                }
            ).unwrap_or_else(
                |e| {
                    panic!("Error declaring RabbitMQ queue: {}", e);
                }
            );
            let consumer = queue.consume(ConsumerOptions {
                no_ack: true,
                ..ConsumerOptions::default()
            }).unwrap_or_else(
                |e| {
                    panic!("Error consuming RabbitMQ queue: {}", e);
                }
            );
            for message in consumer.receiver().iter() {
                match message {
                    ConsumerMessage::Delivery(delivery) => {
                        let body = String::from_utf8_lossy(&delivery.body);
                        println!("Received RabbitMQ packet: {:?}", body);
                        consumer.ack(delivery).unwrap_or_else(
                            |e| {
                                panic!("Error acknowledging RabbitMQ delivery: {}", e);
                            }
                        );
                    },
                    other => {
                        println!("Consumer ended: {:?}", other);
                        break;
                    }
                }
            }
            Ok(())
        });
        Ok(())
    }

    fn discover(&self, discover_packet: PacketDiscover) -> Result<(), std::io::Error> {
        let connection = self.connection.clone();
        let channel = connection.lock().unwrap().open_channel(None).unwrap_or_else(
            |e| {
                panic!("Error opening RabbitMQ channel: {}", e);
            }
        );

        let queue_name = format!("ATOM.{}.{}","DISCOVER", &self.node_id);

        let exchange = Exchange::direct(&channel);
        exchange.publish(Publish::new(&discover_packet.write_to_bytes().unwrap(), queue_name)).unwrap_or_else(
            |e| {
                panic!("Error publishing RabbitMQ packet: {}", e);
            }
        );

        Ok(())
    }
}