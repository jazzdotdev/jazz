use rlua::prelude::*;
use rlua::UserData;
use rlua::UserDataMethods;

#[derive(Serialize, Deserialize)]
struct IntOptions(tantivy::schema::IntOptions);

impl std::ops::BitOr for IntOptions {
    type Output = IntOptions;
    fn bitor(self, other: IntOptions) -> IntOptions {
        IntOptions(self.0 | other.0)
    }
}

fn to_int_option(vec: Vec<rlua::Value>) -> rlua::Result<IntOptions> {
    assert!(!vec.is_empty());
    let mut res = rlua_serde::from_value(vec[0].clone())?;
    for op in vec {
        let op = rlua_serde::from_value(op)?;
        res = res | op;
    }
    Ok(res)
}

#[derive(Serialize, Deserialize)]
struct TextOptions(tantivy::schema::TextOptions);

impl std::ops::BitOr for TextOptions {
    type Output = TextOptions;
    fn bitor(self, other: TextOptions) -> TextOptions {
        TextOptions(self.0 | other.0)
    }
}

fn to_text_option(vec: Vec<rlua::Value>) -> rlua::Result<TextOptions> {
    assert!(!vec.is_empty());
    let mut res = rlua_serde::from_value(vec[0].clone())?;
    for op in vec {
        let op = rlua_serde::from_value(op)?;
        res = res | op;
    }
    Ok(res)
}

struct Field(tantivy::schema::Field);
impl UserData for Field {}

struct SchemaBuilder(tantivy::schema::SchemaBuilder);

impl UserData for IntOptions {}

impl UserData for SchemaBuilder {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "add_u64_field",
            |_, this, (name, options): (String, Vec<rlua::Value>)| {
                let option = to_int_option(options)?;
                let field = this.0.add_u64_field(&name, option.0);
                Ok(Field(field))
            },
        );

        methods.add_method_mut(
            "add_i64_field",
            |_, this, (name, options): (String, Vec<rlua::Value>)| {
                let option = to_int_option(options)?;
                let field = this.0.add_i64_field(&name, option.0);
                Ok(Field(field))
            },
        );

        methods.add_method_mut(
            "add_text_field",
            |_, this, (name, options): (String, Vec<rlua::Value>)| {
                let option = to_text_option(options)?;
                let field = this.0.add_text_field(&name, option.0);
                Ok(Field(field))
            },
        );

        methods.add_method_mut("add_facet_field", |_, this, name: String| {
            Ok(Field(this.0.add_facet_field(&name)))
        });

        methods.add_method_mut("add_bytes_field", |_, this, name: String| {
            Ok(Field(this.0.add_facet_field(&name)))
        });
    }
}

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    Ok(())
}
