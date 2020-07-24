#[cfg(test)]
use mockall::{automock, predicate::*};

use log::{info, error};

use rlua::{Lua, Function, Error};

#[cfg_attr(test, automock)]
pub trait LuaEngine {
    fn new() -> Self;
    fn initialize(&self);
    fn test(&self);
}

pub struct LuaEngineImpl {
    pub lua: Lua
}

impl LuaEngine for LuaEngineImpl {

    fn new() -> LuaEngineImpl {
        LuaEngineImpl {
            lua: Lua::new()
        }
    }

    fn initialize(&self) {
        
        info!("Initializing LUA engine");

        let res = self.lua.context(|lua_context| {
    
           lua_context.load(r#"
                print("hello world from LUA!")
                function test()
                    print("HELLO from TEST")
                end
           "#).exec()?;
           Ok::<(), Error>(())
        });
    
        match res {
            Err(err) => {
                error!("Error loading LUA script: {}", err);
                return            
            }
            Ok(_) => {
                info!("Loaded LUA script.");
            }
        }
    }   

    fn test(&self) {
        let _res = self.lua.context(|lua_context| {
    
            let globals = lua_context.globals();
            let fn_test: Function = globals.get("test")?;        
            fn_test.call::<_,_>(())?;
     
            Ok::<(), Error>(())
         });        
    }
}