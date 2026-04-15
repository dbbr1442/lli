use mlua::{FromLuaMulti, IntoLua, IntoLuaMulti, Lua, MaybeSend, Result, Table};

pub struct Library<'vm> {
    vm: &'vm Lua,
    lib: Table,
}

//pub struct LibError {
//    reason: String,
//}

type LRes = Result<()>;

impl<'vm> Library<'vm> {
    pub fn new(lua: &'vm Lua) -> Result<Self> {
        let table = lua.create_table()?;
        Ok(Self {
            vm: lua,
            lib: table,
        })
    }

    pub fn register_function<F, A, R>(&self, name: &str, func: F) -> LRes
    where 
        F: Fn(&Lua, A) -> mlua::Result<R> + MaybeSend + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        let function = self.vm.create_function(func)?;
        self.lib.set(name, function)?;
        Ok(())
    }

    pub fn register_class<T: GetPrototype>(&self, name: &str) -> LRes {
        let mut proto = ProtoTable::new(self.vm)?;
        T::prototype(&mut proto);
        self.lib.set(name, proto.take_table())?;
        Ok(())
    }

    pub fn inject_as_module(self, mod_name: &str) -> LRes {
        self.vm.register_module(mod_name, self.lib)?;
        Ok(())
    }

    pub fn inject_as_global(self, name: &str) -> LRes {
        self.vm.globals().set(name, self.lib)?;
        Ok(())
    }
}


pub struct ProtoTable<'vm> {
    lua: &'vm Lua,
    inner: Table,
}

impl<'vm> ProtoTable<'vm> {
    pub fn new(lua: &'vm Lua) -> Result<Self> {
        let table = lua.create_table()?;
        Ok(Self { inner: table, lua })
    }

    pub fn add_val(&self, key: &str, val: impl IntoLua) {
        self.inner.set(key, val).unwrap();
    }

    pub fn add_fn<A: FromLuaMulti, R: IntoLuaMulti>(&self, key: &str, func: impl Fn(&Lua, A) -> Result<R> + 'static) {
        let lua_fn = self.lua.create_function(func).unwrap();
        self.inner.set(key, lua_fn).unwrap();
    }

    pub fn take_table(self) -> Table {
        self.inner
    }
}

pub trait GetPrototype {
    fn prototype(proto: &mut ProtoTable);
}
