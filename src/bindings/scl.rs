use rlua::ToLua;

fn to_rlua_value(lua: &rlua::Lua, val: scl::Value) -> rlua::Value {
    match val {
        scl::Value::Boolean(b) => b.to_lua(lua).unwrap(),
        scl::Value::Integer(i) => i.to_lua(lua).unwrap(),
        scl::Value::Float(f) => f.to_lua(lua).unwrap(),
        scl::Value::String(s) => s.to_lua(lua).unwrap(),
        scl::Value::Dict(d) => d
            .into_iter()
            .map(|(k, v)| (k, to_rlua_value(lua, v)))
            .collect::<std::collections::BTreeMap<_, _>>()
            .to_lua(lua)
            .unwrap(),
        scl::Value::Array(a) => a
            .into_iter()
            .map(|v| to_rlua_value(lua, v))
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

use rlua::FromLua;

fn to_date(lua: &rlua::Lua, t: &rlua::Table) -> rlua::Result<scl::Date> {
    if t.len()? == 3
        || t.contains_key("day")?
        || t.contains_key("month")?
        || t.contains_key("year")?
    {
        let day = <_>::from_lua(t.get("day")?, lua)?;
        let month = <_>::from_lua(t.get("month")?, lua)?;
        let year = <_>::from_lua(t.get("year")?, lua)?;
        Ok(scl::Date { day, month, year })
    } else {
        Err(rlua::Error::external(failure::err_msg("not a date")))
    }
}

fn to_scl_value(lua: &rlua::Lua, val: rlua::Value) -> scl::Value {
    match &val {
        rlua::Value::Boolean(_) => scl::Value::Boolean(<_>::from_lua(val, lua).unwrap()),
        rlua::Value::Integer(_) => scl::Value::Integer(<_>::from_lua(val, lua).unwrap()),
        rlua::Value::Number(_) => scl::Value::Float(<_>::from_lua(val, lua).unwrap()),
        rlua::Value::String(_) => scl::Value::String(<_>::from_lua(val, lua).unwrap()),
        rlua::Value::Table(_) => {
            let t = if let rlua::Value::Table(t) = val {
                t
            } else {
                unreachable!();
            };
            if t.contains_key(1).unwrap() {
                let len = t.len().unwrap() as usize;
                let mut vec = Vec::new();
                for i in 1..=len {
                    vec.push(to_scl_value(lua, t.get(i).unwrap()))
                }
                scl::Value::Array(vec)
            } else if let Ok(date) = to_date(lua, &t) {
                scl::Value::Date(date)
            } else {
                let mut dict = std::collections::BTreeMap::new();
                for pair in t.pairs::<String, rlua::Value>() {
                    let (k, v) = pair.unwrap();
                    dict.insert(k, to_scl_value(lua, v));
                }
                scl::Value::Dict(dict)
            }
        }
        _ => panic!("Not supported"),
    }
}

fn escape(s: String) -> String {
    "\"".to_owned() + &s.replace('"', r##"\""##) + "\""
}

fn scl_decode(d: scl::Dict) -> String {
    let mut s = String::new();
    for (k, v) in d {
        s += &k;
        s += "=";
        s += &to_string(v);
        s += "\n";
    }
    s
}

fn to_string(val: scl::Value) -> String {
    match val {
        scl::Value::Boolean(b) => if b { "true" } else { "false" }.to_owned(),
        scl::Value::Integer(i) => i.to_string(),
        scl::Value::Float(f) => f.to_string(),
        scl::Value::String(s) => escape(s),
        scl::Value::Date(d) => format!("{:04}-{:02}-{:02}", d.year, d.month, d.day),
        scl::Value::Array(a) => {
            let mut s = "[".to_owned();
            for v in a {
                s += &to_string(v);
                s += ",";
            }
            s += "]";
            s
        }
        scl::Value::Dict(d) => {
            let mut s = "{".to_owned();
            for (k, v) in d {
                s += &k;
                s += "=";
                s += &to_string(v);
                s += ",";
            }
            s += "}";
            s
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
            Ok(to_rlua_value(lua, scl::Value::Dict(dict)))
        })?,
    )?;

    // Encode table to a string
    module.set(
        "from_table",
        lua.create_function(|lua, table: rlua::Table| {
            let val = to_scl_value(lua, rlua::Value::Table(table));
            if let scl::Value::Dict(d) = val {
                Ok(scl_decode(d))
            } else {
                unreachable!();
            }
        })?,
    )?;

    lua.globals().set("scl", module)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_decode() {
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
        lua.exec::<_, rlua::Value>(
            r##"
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
        "##,
            None,
        )
        .unwrap();
    }

    #[test]
    fn test_encode() {
        let lua = rlua::Lua::new();
        super::init(&lua).unwrap();
        lua.exec::<_, rlua::Value>(
            r##"
        local x = {
        x=0,
        y=true,
        z="ab",
        a=1.2,
        b={c=1},
        d={day=1,month=2,year=2018},
        e={2, 3, 4},
        }
        local s = scl.from_table(x)
        print(s)
        local y = scl.to_table(s)
        assert(y.b.c == 1)
        assert(y.d.year == 2018)
        assert(y.d.month == 2)
        assert(y.d.day == 1)
        assert(y.e[1] == 2)
        assert(y.e[2] == 3)
        assert(y.e[3] == 4)
        assert(#y.e == 3)
        "##,
            None,
        )
        .unwrap();
    }
}
