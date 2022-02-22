use bevy::prelude::*;
use naia_server_socket::{PacketReceiver, PacketSender, ServerAddrs, Socket};

use shared::shared_config;

pub struct App {
    pub packet_sender: PacketSender,
    pub packet_receiver: PacketReceiver,
}

impl App {
    pub fn new() -> Self {
        info!("Server Demo started");

        let server_address = ServerAddrs::new(
            "127.0.0.1:14191"
                .parse()
                .expect("could not parse Session address/port"),
            // IP Address to listen on for UDP WebRTC data channels
            "127.0.0.1:14192"
                .parse()
                .expect("could not parse WebRTC data address/port"),
            // The public WebRTC IP address to advertise
            "http://127.0.0.1:14192",
        );
        let shared_config = shared_config();

        let mut socket = Socket::new(shared_config);
        socket.listen(server_address);

        App {
            packet_sender: socket.packet_sender(),
            packet_receiver: socket.packet_receiver(),
        }
    }
}
