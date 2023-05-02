use eda_demo_rust::{rabbit_mq_transporter::RabbitMqTransporter, mq::Transporter, packet_proto::packets::PacketDiscover};

fn main() {
    // Instantiate a RabbitMqTransporter and open the connection
    let transporter = RabbitMqTransporter::new("amqp://localhost:5672/%2f".to_string()).unwrap_or_else(
        |e| {
            panic!("Error creating RabbitMqTransporter: {}", e);
        }
    );

    let discover_packet = PacketDiscover {
        ..Default::default()
    };

    transporter.discover(discover_packet).unwrap_or_else(
        |e| {
            panic!("Error sending discover packet: {}", e);
        }
    );

    transporter.subscribe_all().unwrap_or_else(
        |e| {
            panic!("Error subscribing to all topics: {}", e);
        }
    );
}
