#[cfg(test)]
use mockall::{automock, predicate::*};

use log::{info};

use crate::luaengine::luaengine::LuaEngine;
use crate::luaengine::luaengine::LuaEngineImpl;

#[cfg_attr(test, automock)]
pub trait MqttHandler {
    fn new(lua_engine: LuaEngineImpl) -> Self;
    fn handle_mqtt_message(&self, msg: paho_mqtt::message::Message);
}

pub struct MqttHandlerImpl {
    pub lua_engine: LuaEngineImpl
}

impl MqttHandler for MqttHandlerImpl {
    fn new(lua_engine: LuaEngineImpl) -> MqttHandlerImpl {
        MqttHandlerImpl {
            lua_engine: lua_engine
        }
    }

    fn handle_mqtt_message(&self, msg: paho_mqtt::message::Message) {
        info!("{}", msg);
        self.lua_engine.test();
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