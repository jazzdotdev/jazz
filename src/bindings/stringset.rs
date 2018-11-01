use rlua::prelude::*;
use rlua::{UserDataMethods, UserData, MetaMethod, Value, Table, Lua, FromLua};
use std::collections::HashSet;

#[derive(Clone)]
struct StringSet (HashSet<String>);

impl UserData for StringSet {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

        methods.add_method_mut("insert", |_, this, elem: String| {
            this.0.insert(elem);
            Ok(())
        });

        methods.add_method_mut("remove", |_, this, elem: String| {
            this.0.remove(&elem);
            Ok(())
        });

        methods.add_method("contains", |_, this, elem: String| {
            Ok(this.0.contains(&elem))
        });

        methods.add_method_mut("clear", |_, this, _: ()| {
            this.0.clear();
            Ok(())
        });

        methods.add_method("is_empty", |_, this, _: ()| {
            Ok(this.0.is_empty())
        });

        methods.add_method("difference", |_, this, other: StringSet| {
            let result: HashSet<String> = this.0.difference(&other.0).cloned().collect();
            Ok(StringSet(result))
        });

        methods.add_method("symmetric", |_, this, other: StringSet| {
            let result: HashSet<String> = this.0.symmetric_difference(&other.0).cloned().collect();
            Ok(StringSet(result))
        });

        methods.add_method("intersection", |_, this, other: StringSet| {
            let result: HashSet<String> = this.0.intersection(&other.0).cloned().collect();
            Ok(StringSet(result))
        });

        methods.add_method("union", |_, this, other: StringSet| {
            let result: HashSet<String> = this.0.union(&other.0).cloned().collect();
            Ok(StringSet(result))
        });

        methods.add_method("is_disjoint", |_, this, other: StringSet| {
            Ok(this.0.is_disjoint(&other.0))
        });

        methods.add_method("is_subset", |_, this, other: StringSet| {
            Ok(this.0.is_subset(&other.0))
        });

        methods.add_method("is_superset", |_, this, other: StringSet| {
            Ok(this.0.is_superset(&other.0))
        });

        methods.add_meta_method(MetaMethod::ToString, |_, this, _: ()| {
            Ok(format!("{:?}", this.0))
        });

        methods.add_meta_method(MetaMethod::Len, |_, this, _: ()| {
            Ok(this.0.len())
        });

        methods.add_method("clone", |_, this, _: ()| {
            Ok(StringSet(this.0.clone()))
        });

        methods.add_method("into_table", |lua, this, _: ()| {
            let table = lua.create_sequence_from(this.0.iter().cloned())?;
            Ok(table)
        });
    }
}

pub fn init(lua: &Lua) -> Result<(), LuaError> {

    type Set = HashSet<String>;

    fn from_table (table: Table) -> Result<Set, LuaError> {
        let mut set = HashSet::new();
        for elem in table.sequence_values() {
            set.insert(elem?);
        }
        Ok(set)
    }

    fn get_sets (lua: &Lua, args: (Value, Value)) -> Result<(Set, Set), LuaError> {
        fn from_value (a: Value, lua: &Lua) -> Result<Set, LuaError> {
            Ok(match a {
                Value::Table(t) => from_table(t)?,
                a@_ => StringSet::from_lua(a, lua)?.0
            })
        }
        let (a, b) = args;
        Ok((from_value(a, lua)?, from_value(b, lua)?))
    }

    let module = lua.create_table()?;

    module.set("create", lua.create_function(
        |_, _: ()|  Ok(StringSet(HashSet::new()))
    )? )?;

    module.set("from_table", lua.create_function(
        |_, t: Table|  Ok(StringSet(from_table(t)?))
    )? )?;

    let g = lua.globals();
    g.set("stringset", module)?;

    g.set("difference", lua.create_function( |lua, args: (Value, Value)| {
        let (a, b) = get_sets(lua, args)?;
        let c = a.difference(&b).cloned().collect();
        Ok(StringSet(c))
    })? )?;

    g.set("symmetric", lua.create_function( |lua, args: (Value, Value)| {
        let (a, b) = get_sets(lua, args)?;
        let c = a.symmetric_difference(&b).cloned().collect();
        Ok(StringSet(c))
    })? )?;

    g.set("intersection", lua.create_function( |lua, args: (Value, Value)| {
        let (a, b) = get_sets(lua, args)?;
        let c = a.intersection(&b).cloned().collect();
        Ok(StringSet(c))
    })? )?;

    g.set("union", lua.create_function( |lua, args: (Value, Value)| {
        let (a, b) = get_sets(lua, args)?;
        let c = a.union(&b).cloned().collect();
        Ok(StringSet(c))
    })? )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_methods() {
        let lua = Lua::new();
        init(&lua).unwrap();
        lua.exec::<_, Value>(r#"
            local a = stringset.create()
            a:insert("Colombia")
            a:insert("Canada")
            a:insert("China")

            local b = stringset.create()
            b:insert("Venezuela")
            b:insert("Colombia")
            b:insert("Brazil")

            local c = a:union(b)
            assert(#c == 5)
            assert(c:contains("Colombia"))
            assert(c:contains("Venezuela"))
            assert(c:contains("Canada"))

            c = a:intersection(b)
            assert(#c == 1)
            assert(c:contains("Colombia"))
            assert(not c:contains("Canada"))

            c = a:difference(b)
            assert(#c == 2)
            assert(c:contains("Canada"))
            assert(not c:contains("Colombia"))

            c = a:symmetric(b)
            assert(#c == 4)
            assert(c:contains("Canada"))
            assert(c:contains("Venezuela"))
            assert(not c:contains("Colombia"))

            d = a:clone()
            d:remove("Canada")
            assert(a:is_superset(d))
            assert(d:is_subset(a))
            assert(not a:is_disjoint(b))
            assert(a:is_disjoint(b:difference(a)))

            d:clear()
            assert(d:is_empty())

            local t = a:union(b):into_table()
            for i, v in ipairs(t) do
                print(i, v)
            end
        "#, None).unwrap();
    }

    #[test]
    fn shortcut_syntax() {
        let lua = Lua::new();
        init(&lua).unwrap();
        lua.exec::<_, Value>(r#"
            local a = stringset.create()
            a:insert("Canada")
            a:insert("China")
            a:insert("Colombia")

            local b = {"Colombia", "Brazil", "Venezuela"}
            local c = intersection(a, b)
            assert(c:contains("Colombia"))
            assert(not c:contains("Canada"))

            union(a, b)
            difference(a, b)
            symmetric(a, b)
        "#, None).unwrap();
    }
}