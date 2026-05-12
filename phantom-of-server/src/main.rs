use log::{debug, error, info, warn};
use renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use renet::{
    ChannelConfig, ConnectionConfig, RenetServer, SendType, ServerEvent as RenetServerEvent,
};
use scopa_lib::event::{ClientEvent, ServerEvent};
use scopa_lib::ScopaGame;
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, SystemTime};

const SERVER_IP: &str = "0.0.0.0";
const RELIABLE_CHANNEL: u8 = 2;
const PROTOCOL_ID: u64 = 1252;
const DELTA_TIME: Duration = Duration::from_millis(16);

fn main() {
    #[cfg(debug_assertions)]
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    #[cfg(not(debug_assertions))]
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let server_port: String = if let Ok(port) = std::env::var("PORT") {
        port
    } else {
        "6969".into()
    };

    let server_addr: SocketAddr = format!("{}:{}", SERVER_IP, server_port)
        .parse()
        .expect("Failed to parse server socket addr");

    let (mut server, mut transport) = setup_server(server_addr);
    let mut game = ScopaGame::default();

    info!("phantom-of-server listening on '{}'", server_addr);
    loop {
        server.update(DELTA_TIME);
        transport
            .update(DELTA_TIME, &mut server)
            .expect("Failed to update transport");

        while let Some(event) = server.get_event() {
            match event {
                RenetServerEvent::ClientConnected { client_id } => {
                    info!("Client {client_id} connected");
                }
                RenetServerEvent::ClientDisconnected { client_id, reason } => {
                    info!("Client {client_id} disconnected: {reason}");
                }
            }
        }

        for client_id in server.clients_id() {
            while let Some(message) = server.receive_message(client_id, RELIABLE_CHANNEL) {
                if let Ok(event) = bincode::deserialize::<ClientEvent>(&message) {
                    debug!("Event from player {client_id}: {:#?}", event);
                    if let Err(e) = game.validate(client_id.raw(), &event) {
                        debug!("Event validation error: {e}");
                        let response = ServerEvent::ActionDenied {
                            reason: e.to_string(),
                        };
                        server.send_message(
                            client_id,
                            RELIABLE_CHANNEL,
                            bincode::serialize::<ServerEvent>(&response)
                                .expect("Failed to serialize server response"),
                        );
                        continue;
                    }
                    match game.consume(client_id.raw(), &event) {
                        Ok(response) => {
                            debug!("Sending response: {:#?}", response);
                            server.broadcast_message(
                                RELIABLE_CHANNEL,
                                bincode::serialize::<ServerEvent>(&response)
                                    .expect("Failed to serialize server response"),
                            )
                        }
                        Err(e) => {
                            error!("Error while consuming validated event: {e}");
                            let response = ServerEvent::Error {
                                description: e.to_string(),
                            };
                            server.send_message(
                                client_id,
                                RELIABLE_CHANNEL,
                                bincode::serialize::<ServerEvent>(&response)
                                    .expect("Failed to serialize server response"),
                            );
                        }
                    }
                } else {
                    warn!("Invalid message from player {client_id}: {:#?}", message);
                    let response = ServerEvent::Error {
                        description: "Invalid message received".into(),
                    };
                    server.send_message(
                        client_id,
                        RELIABLE_CHANNEL,
                        bincode::serialize::<ServerEvent>(&response)
                            .expect("Failed to serialize server response"),
                    );
                }
            }
        }

        transport.send_packets(&mut server);
    }
}

fn setup_server(addr: SocketAddr) -> (RenetServer, NetcodeServerTransport) {
    let channel_config = vec![ChannelConfig {
        channel_id: RELIABLE_CHANNEL,
        send_type: SendType::ReliableOrdered {
            resend_time: Duration::from_millis(300),
        },
        max_memory_usage_bytes: 5 * 1024 * 1024,
    }];

    let server: RenetServer = RenetServer::new(ConnectionConfig {
        client_channels_config: channel_config.clone(),
        server_channels_config: channel_config,
        available_bytes_per_tick: 60_000,
    });

    let socket: UdpSocket = UdpSocket::bind(addr).expect("Failed to bind UDP socker");
    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to get system time"),
        max_clients: 64,
        public_addresses: vec![addr],
        authentication: ServerAuthentication::Unsecure,
        protocol_id: PROTOCOL_ID,
    };

    let transport =
        NetcodeServerTransport::new(server_config, socket).expect("Failed to initialize transport");

    (server, transport)
}
