use rlua::prelude::*;
use rlua::{UserDataMethods, UserData, MetaMethod};
use std::collections::HashSet;

#[derive(Clone)]
struct StringSet (HashSet<String>);

impl UserData for StringSet {
    fn add_methods(methods: &mut UserDataMethods<Self>) {

        methods.add_method_mut("insert", |_, this, elem: String| {
            this.0.insert(elem);
            Ok(())
        });

        methods.add_method("union", |_, this, other: StringSet| {
            let union: HashSet<String> = this.0.union(&other.0).map(|s| s.clone()).collect();
            Ok(StringSet(union))
        });

        methods.add_meta_method(MetaMethod::ToString, |_, this, _: ()| {
            Ok(format!("{:?}", this.0))
        });
    }
}

pub fn init(lua: &Lua) -> Result<(), LuaError> {

    let create_fn = lua.create_function( |_, _: ()|  Ok(StringSet(HashSet::new())) )?;

    let module = lua.create_table()?;
    module.set("create", create_fn)?;
    lua.globals().set("stringset", module)?;

    Ok(())
}
