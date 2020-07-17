pub fn handle_mqtt_message(msg: paho_mqtt::message::Message) {
    println!("{}", msg);
}