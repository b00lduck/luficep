#[cfg(test)]
use mockall::{automock, predicate::*};

use log::{info};

pub struct MqttHandler {}

impl MqttHandler {}

#[cfg_attr(test, automock)]
pub trait HandleMqttMessage {
    fn handle_mqtt_message(&self, msg: paho_mqtt::message::Message);
}

impl HandleMqttMessage for MqttHandler {
    fn handle_mqtt_message(&self, msg: paho_mqtt::message::Message) {
        info!("{}", msg);
    }    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        assert_eq!(2 + 2, 4);
        Ok(())
    }
}