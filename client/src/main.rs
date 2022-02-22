use std::net::SocketAddr;

use bevy::prelude::*;
use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin, Packet};
use shared::SERVER_PORT;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(setup)
        .add_system(send_packets)
        .add_system(handle_packets)
        .run();
}

fn setup(mut commands: Commands, mut net: ResMut<NetworkResource>) {
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

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let mut server_address: SocketAddr = "192.168.1.105:0".parse().unwrap();
            server_address.set_port(SERVER_PORT);
        } else {
            let ip_address =
                bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
            let server_address = SocketAddr::new(ip_address, SERVER_PORT);
        }
    }

    info!("Starting client");
    net.connect(server_address);
}

fn send_packets(mut net: ResMut<NetworkResource>, time: Res<Time>) {
    // Client context
    if (time.seconds_since_startup() * 60.) as i64 % 60 == 0 {
        info!("PING");
        net.broadcast(Packet::from("PING"));
    }
}

fn handle_packets(
    mut net: ResMut<NetworkResource>,
    time: Res<Time>,
    mut reader: EventReader<NetworkEvent>,
) {
    for event in reader.iter() {
        match event {
            NetworkEvent::Packet(handle, packet) => {
                let message = String::from_utf8_lossy(packet);
                info!("Got packet on [{}]: {}", handle, message);
                if message == "PING" {
                    let message = format!("PONG @ {}", time.seconds_since_startup());
                    match net.send(*handle, Packet::from(message)) {
                        Ok(()) => {
                            info!("Sent PONG");
                        }
                        Err(error) => {
                            info!("PONG send error: {}", error);
                        }
                    }
                }
            }
            event => info!("{event:?} received!"),
        }
    }
}
