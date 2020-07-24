mod handler;
mod luaengine;

use std::{
    env,
    process,
    time::Duration,
};
use futures::{
    executor::block_on,
    stream::StreamExt
};
use paho_mqtt as mqtt;
use log::{info, warn, error};

use crate::handler::mqtt::MqttHandler;
use crate::luaengine::luaengine::LuaEngine;

// The topics to which we subscribe.c
const TOPICS: &[&str] = &[ "test", "hello" ];
const QOS: &[i32] = &[2, 2];

fn main() {
    // Initialize the logger from the environment
    env_logger::init();

    info!("Lucifep starting...");

    // initialize LUA engine
    let lua_engine: crate::luaengine::luaengine::LuaEngineImpl = luaengine::luaengine::LuaEngine::new();
    lua_engine.initialize();

    // initialize MQTT handler
    let mqtt_handler: crate::handler::mqtt::MqttHandlerImpl = handler::mqtt::MqttHandler::new(lua_engine);

    let host = env::args().nth(1).unwrap_or_else(||
        "tcp://localhost:1883".to_string()
    );

    // Create the client. Use an ID for a persistent session.
    // A real system should try harder to use a unique ID.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id("lucifep")
        .finalize();

    // Create the client connection
    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        error!("Error creating the MQTT client: {:?}", e);
        process::exit(1);
    });


    if let Err(err) = block_on(async {
        // Get message stream before connecting.
        let mut strm = cli.get_stream(25);

        // Define the set of options for the connection
        let lwt = mqtt::Message::new("test", "Lucifep lost connection", mqtt::QOS_2);

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(20))
            .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
            .clean_session(false)
            .keep_alive_interval(Duration::from_secs(15))
            .will_message(lwt)
            .finalize();

        // Make the connection to the broker
        info!("Connecting to the MQTT server...");
        cli.connect(conn_opts).await?;
        info!("Connected to the MQTT server.");

        info!("Subscribing to topics: {:?}", TOPICS);
        cli.subscribe_many(TOPICS, QOS).await?;

        // Just loop on incoming messages.
        info!("Waiting for MQTT messages...");

        while let Some(msg_opt) = strm.next().await {            
            if let Some(msg) = msg_opt {
                // A "Some" means we have a new message
                mqtt_handler.handle_mqtt_message(msg);
            } else {
                // A "None" means we were disconnected. Try to reconnect...
                warn!("Lost connection. Attempting reconnect.");
                loop {
                    match cli.reconnect().await {
                        Err(err) => {
                            error!("Error reconnecting: {}", err);                    
                            async_std::task::sleep(Duration::from_millis(2000)).await;
                        }
                        Ok(_) => {
                            info!("Reconnected to the MQTT server.");
                            break;
                        }
                    }
                }
            }
        }

        // Explicit return type for the async block
        Ok::<(), mqtt::Error>(())
    }) {
        error!("{}", err);
    }

}
