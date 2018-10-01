use rlua::{Lua};


pub struct ScriptMaster {
    _lua: Lua
}

impl ScriptMaster {
    pub fn new() -> Self {
        Self {
            _lua: Lua::new()
        }
    }
}