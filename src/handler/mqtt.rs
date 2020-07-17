use log::{info};

pub fn handle_mqtt_message(msg: paho_mqtt::message::Message) {
    info!("{}", msg);
}