use std::io::Read;

use lli::Library;
use mlua::prelude::*;

fn rust_print(lua: &Lua, arg: String) -> mlua::Result<()> {
    println!("{}", arg);
    Ok(())
}

fn insert_library(lua: &Lua) {
    let lib = Library::new(lua);
    lib.register_function("rust_print", rust_print);
    lib.inject("@fish");
} 

fn main() {
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open("./main.lua")
        .unwrap();

    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    drop(file);

    let lua = Lua::new();

    insert_library(&lua);

    let chunk = lua.load(source);
    chunk.exec().unwrap();

    loop {}
}
