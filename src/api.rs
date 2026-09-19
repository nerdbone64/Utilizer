use std::collections::HashMap;

use mlua::prelude::{
    FromLuaMulti,
    IntoLua,
    IntoLuaMulti,
    Lua,
    LuaResult,
    LuaTable,
    LuaValue,
    LuaVariadic,
};

use crate::util::{launch, log};

pub const API_VERSION: u32 = 0;

macro_rules! map {
    ($lua:expr; $($key:expr => $value:expr),* $(,)?) => {{
        let mut map: HashMap<&str, LuaValue> = HashMap::new();
        $(map.insert(
            $key,
            ($value)
                .into_lua($lua)
                .expect("failed to convert Lua value"),
        );)*
        map
    }};
}

pub fn injectv(
    tab: &LuaTable,
    vals: HashMap<&str, LuaValue>,
) -> bool {
    for (id, val) in vals {
        if let Err(e) = tab.set(id, val) {
            log(&format!("injectv error: {e}"), 3);
            return false;
        }
    }
    true
}

pub fn injectf<A, R, F>(
    lua: &Lua,
    tab: &LuaTable,
    name: &str,
    function: F,
) -> bool
where
    A: FromLuaMulti,
    R: IntoLuaMulti,
    F: Fn(&Lua, A) -> LuaResult<R> + Send + 'static,
{
    let function = match lua.create_function(function) {
        Ok(function) => function,

        Err(e) => {
            log(&format!("injectf create_function error: {e}"), 3);
            return false;
        }
    };

    match tab.set(name, function) {
        Ok(_) => true,
        Err(e) => {
            log(&format!("injectf insertion error: {e}"), 3);
            false
        }
    }
}

pub fn new_lua() -> Lua {
    let lua = Lua::new();
    log("initing Lua API...", 0);
    let globals = lua.globals();
    injectv(
        &globals,
        map![&lua;
            "_test" => 8.7,
            "_version" => API_VERSION,
        ],
    );
    injectf(
        &lua,
        &globals,
        "log",
        |_lua, (msg, lvl): (String, f64)| {
            log(&msg, lvl.trunc() as i32);
            Ok(())
        },
    );
    injectf(
        &lua,
        &globals,
        "app",
        |_lua, (program, args): (String, LuaVariadic<String>)| {
            let args: Vec<String> =
                args.into_iter().collect();
            log(&format!("launching: {program}"), 1);
            launch(&program, &args)
                .map_err(mlua::Error::external)?;
            Ok(())
        },
    );
    lua
}