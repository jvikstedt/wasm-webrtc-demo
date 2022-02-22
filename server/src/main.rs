use std::net::SocketAddr;

use bevy::{log::LogPlugin, prelude::*};

use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin, Packet};
use shared::SERVER_PORT;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(setup)
        .add_system(handle_packets)
        .run();
}

fn setup(mut net: ResMut<NetworkResource>) {
    let ip_address =
        bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    let server_address = SocketAddr::new(ip_address, SERVER_PORT);

    info!("Starting server");
    net.listen(server_address, None, None);
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
