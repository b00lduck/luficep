mod handler;

pub use crate::handler::mqtt as mqtt_handler; 

use std::{
    env,
    process,
    time::Duration
};
use futures::{
    executor::block_on,
    stream::StreamExt
};
use paho_mqtt as mqtt;
use log::{info, warn, error};

// The topics to which we subscribe.
const TOPICS: &[&str] = &[ "test", "hello" ];
const QOS: &[i32] = &[1, 1];

fn main() {
    // Initialize the logger from the environment
    env_logger::init();

    let host = env::args().nth(1).unwrap_or_else(||
        "tcp://localhost:1883".to_string()
    );

    // Create the client. Use an ID for a persistent session.
    // A real system should try harder to use a unique ID.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id("rust_async_subscribe")
        .finalize();

    // Create the client connection
    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        error!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    if let Err(err) = block_on(async {
        // Get message stream before connecting.
        let mut strm = cli.get_stream(25);

        // Define the set of options for the connection
        let lwt = mqtt::Message::new("test", "Async subscriber lost connection",
                                     mqtt::QOS_1);

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(20))
            .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
            .clean_session(false)
            .will_message(lwt)
            .finalize();

        // Make the connection to the broker
        info!("Connecting to the MQTT server...");
        cli.connect(conn_opts).await?;

        info!("Subscribing to topics: {:?}", TOPICS);
        cli.subscribe_many(TOPICS, QOS).await?;

        // Just loop on incoming messages.
        info!("Waiting for messages...");

        while let Some(msg_opt) = strm.next().await {            
            if let Some(msg) = msg_opt {
                mqtt_handler::handle_mqtt_message(msg);                
            }
            else {
                // A "None" means we were disconnected. Try to reconnect...
                warn!("Lost connection. Attempting reconnect.");
                while let Err(err) = cli.reconnect().await {
                    error!("Error reconnecting: {}", err);
                    // For tokio use: tokio::time::delay_for()
                    async_std::task::sleep(Duration::from_millis(1000)).await;
                }
            }
        }

        // Explicit return type for the async block
        Ok::<(), mqtt::Error>(())
    }) {
        error!("{}", err);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}