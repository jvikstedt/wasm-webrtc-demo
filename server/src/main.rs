use bevy::prelude::*;
use naia_server_socket::Packet;
use shared::{PING_MSG, PONG_MSG};

mod app;
use app::App as MyApp;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(MyApp::new());
}

fn update(mut app_res: ResMut<MyApp>) {
    match app_res.packet_receiver.receive() {
        Ok(Some(packet)) => {
            let address = packet.address();
            let message_from_client = String::from_utf8_lossy(packet.payload());
            info!("Server recv <- {}: {}", address, message_from_client);

            if message_from_client.eq(PING_MSG) {
                let message_to_client: String = PONG_MSG.to_string();
                info!("Server send -> {}: {}", address, message_to_client);
                app_res
                    .packet_sender
                    .send(Packet::new(address, message_to_client.into_bytes()));
            }
        }
        Ok(None) => {}
        Err(error) => {
            info!("Server Error: {}", error);
        }
    }
}
