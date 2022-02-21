use bevy::prelude::*;

use naia_client_socket::{Packet, PacketReceiver, PacketSender, ServerAddr, Socket};
use shared::{shared_config, PING_MSG};

pub struct Conn {
    packet_sender: PacketSender,
    packet_receiver: PacketReceiver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello_wasm_system)
        .add_startup_system(setup)
        .add_system(poll)
        .run();
}

fn hello_wasm_system() {
    info!("hello wasm");
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1. / 50.;
    commands.spawn_bundle(camera_bundle);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..Default::default()
        },
        ..Default::default()
    });

    let shared_config = shared_config();

    let mut socket = Socket::new(shared_config);
    socket.connect("http://127.0.0.1:14191");

    commands.insert_resource(Conn {
        packet_sender: socket.packet_sender(),
        packet_receiver: socket.packet_receiver(),
    });
}

fn poll(mut conn_res: ResMut<Conn>) {
    match conn_res.packet_receiver.receive() {
        Ok(Some(packet)) => {
            let message_from_server = String::from_utf8_lossy(packet.payload());

            let server_addr = match conn_res.packet_receiver.server_addr() {
                ServerAddr::Found(addr) => addr.to_string(),
                _ => "".to_string(),
            };
            info!("Client recv <- {}: {}", server_addr, message_from_server);
        }
        Ok(None) => {
            let message_to_server: String = PING_MSG.to_string();

            let server_addr = match conn_res.packet_receiver.server_addr() {
                ServerAddr::Found(addr) => addr.to_string(),
                _ => "".to_string(),
            };
            info!("Client send -> {}: {}", server_addr, message_to_server);

            conn_res
                .packet_sender
                .send(Packet::new(message_to_server.into_bytes()));
        }
        Err(err) => {
            info!("Client Error: {}", err);
        }
    }
}
