use eda_demo_rust::rabbit_mq_transporter::RabbitMqTransporter;
fn main() {
    // Instantiate a RabbitMqTransporter and open the connection
    RabbitMqTransporter::new("amqp://guest:guest@localhost:5672/%2f")
        .expect("Error creating transporter");
}
