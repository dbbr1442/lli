use mlua::{FromLuaMulti, IntoLua, IntoLuaMulti, Lua, MaybeSend, Table};

pub struct Library<'vm> {
    vm: &'vm Lua,
    lib: Table,
}

impl<'vm> Library<'vm> {
    pub fn new(lua: &'vm Lua) -> Self {
        let table = lua.create_table().unwrap();
        Self {
            vm: lua,
            lib: table,
        }
    }

    pub fn register_function<F, A, R>(&self, name: &str, func: F)
    where 
        F: Fn(&Lua, A) -> mlua::Result<R> + MaybeSend + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        let function = self.vm.create_function(func).unwrap();
        self.lib.set(name, function).unwrap();
    }

    pub fn register_class<T: Default + IntoLua>(&self, name: &str) {
        self.lib.set(name, T::default()).unwrap();
    }

    pub fn inject(self, mod_name: &str) {
        self.vm.register_module(mod_name, self.lib).unwrap();
    }
}
