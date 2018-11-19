use rlua::ToLua;

fn to_value(lua: &rlua::Lua, val: scl::Value) -> rlua::Value {
    match val {
        scl::Value::Boolean(b) => b.to_lua(lua).unwrap(),
        scl::Value::Integer(i) => i.to_lua(lua).unwrap(),
        scl::Value::Float(f) => f.to_lua(lua).unwrap(),
        scl::Value::String(s) => s.to_lua(lua).unwrap(),
        scl::Value::Dict(d) => d
            .into_iter()
            .map(|(k, v)| (k, to_value(lua, v)))
            .collect::<std::collections::BTreeMap<_, _>>()
            .to_lua(lua)
            .unwrap(),
        scl::Value::Array(a) => a
            .into_iter()
            .map(|v| to_value(lua, v))
            .collect::<Vec<_>>()
            .to_lua(lua)
            .unwrap(),
        scl::Value::Date(d) => {
            let mut map = std::collections::BTreeMap::new();
            map.insert("day", d.day as u16);
            map.insert("month", d.month as u16);
            map.insert("year", d.year);
            map.to_lua(lua).unwrap()
        }
    }
}

pub fn init(lua: &rlua::Lua) -> Result<(), rlua::Error> {
    // Decode string to a table
    let module = lua.create_table()?;
    module.set(
        "to_table",
        lua.create_function(|lua, text: String| {
            let dict: scl::Dict = scl::parse_str(&text).unwrap();
            Ok(to_value(lua, scl::Value::Dict(dict)))
        })?,
    )?;

    lua.globals().set("scl", module)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        static TEXT: &str = r##"
owner = {
  name = "Vincent Prouillet",
  dob = 1979-05-27, # first-class date type
}

database = {
  ports = [ 8001, 8001, 8002 ],
  connection_max = 5000,
  enabled = true,
}

servers = {
  max_upload_size = 10MB, # first class byte size
  alpha = {
    ip = "10.0.0.1",
    dc = "eqdc10",
  },
}

clients = {
  data = [
    ["gamma"],
    [1],
  ],
}
        "##;
        let lua = rlua::Lua::new();
        super::init(&lua).unwrap();
        lua.globals().set("TEXT", TEXT).unwrap();
        lua.exec::<_, rlua::Value>(r##"
        local x = scl.to_table(TEXT)
        assert(x.owner.name == "Vincent Prouillet")
        assert(x.owner.dob.year == 1979)
        assert(x.owner.dob.month == 5)
        assert(x.owner.dob.day == 27)
        assert(#x.database.ports == 3)
        assert(x.database.ports[1] == 8001)
        assert(x.database.ports[2] == 8001)
        assert(x.database.ports[3] == 8002)
        assert(x.database.connection_max == 5000)
        assert(x.database.enabled == true)
        assert(x.servers.max_upload_size == 10000000)
        assert(x.servers.alpha.ip == "10.0.0.1")
        assert(x.servers.alpha.dc == "eqdc10")
        assert(x.clients.data[1][1] == "gamma")
        assert(x.clients.data[2][1] == 1)
        "##, None).unwrap();
    }
}
