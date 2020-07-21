#[cfg(test)]
use mockall::{automock, predicate::*};

use log::{info};

use rlua::{Lua, Function, Error};

pub struct MqttHandler {
    pub lua: Lua
}

impl MqttHandler {}

#[cfg_attr(test, automock)]
pub trait HandleMqttMessage {
    fn handle_mqtt_message(&self, msg: paho_mqtt::message::Message);
}

impl HandleMqttMessage for MqttHandler {
    fn handle_mqtt_message(&self, msg: paho_mqtt::message::Message) {
        info!("{}", msg);

        let _res = self.lua.context(|lua_context| {
    
            let globals = lua_context.globals();
            let fn_test: Function = globals.get("test")?;        
            fn_test.call::<_,_>(())?;
     
            Ok::<(), Error>(())
         });        
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