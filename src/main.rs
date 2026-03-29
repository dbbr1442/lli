use std::io::Read;

use mlua::prelude::*;

//fn r_print(_lua: &Lua, arg: LuaString) -> mlua::Result<()> {
//    println!("{:?}", arg);
//    Ok(())
//}
//
//fn load_library(lua: &Lua, functions) {
//    let r_print = lua.create_function(r_print).unwrap();
//    lua.globals().set("r_print", r_print).unwrap();
//}

fn main() {
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open("./main.lua")
        .unwrap();

    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    drop(file);

    let lua = Lua::new();

    //load_library(&lua);
    //lua.create_require_function(require)

    let chunk = lua.load(source);
    chunk.exec().unwrap();
}
