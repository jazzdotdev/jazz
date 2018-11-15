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

struct SchemaBuilder(Option<tantivy::schema::SchemaBuilder>);

impl UserData for IntOptions {}

impl UserData for SchemaBuilder {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "add_u64_field",
            |_, this, (name, options): (String, Vec<rlua::Value>)| {
                let option = to_int_option(options)?;
                let this = this.0.as_mut().expect("Value already moved");
                let field = this.add_u64_field(&name, option.0);
                Ok(Field(field))
            },
        );

        methods.add_method_mut(
            "add_i64_field",
            |_, this, (name, options): (String, Vec<rlua::Value>)| {
                let option = to_int_option(options)?;
                let this = this.0.as_mut().expect("Value already moved");
                let field = this.add_i64_field(&name, option.0);
                Ok(Field(field))
            },
        );

        methods.add_method_mut(
            "add_text_field",
            |_, this, (name, options): (String, Vec<rlua::Value>)| {
                let option = to_text_option(options)?;
                let this = this.0.as_mut().expect("Value already moved");
                let field = this.add_text_field(&name, option.0);
                Ok(Field(field))
            },
        );

        methods.add_method_mut("add_facet_field", |_, this, name: String| {
            let this = this.0.as_mut().expect("Value already moved");
            Ok(Field(this.add_facet_field(&name)))
        });

        methods.add_method_mut("add_bytes_field", |_, this, name: String| {
            let this = this.0.as_mut().expect("Value already moved");
            Ok(Field(this.add_facet_field(&name)))
        });

        methods.add_method_mut("build", |_, this, _: ()| {
            let this = this.0.take().expect("Value already moved");
            Ok(Schema(this.build()))
        })
    }
}

struct Schema(tantivy::schema::Schema);

impl UserData for Schema {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_field", |_, this, name: String| {
            Ok(this.0.get_field(&name).map(|x| Field(x)))
        });
    }
}

struct Index(tantivy::Index);

impl UserData for Index {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("writer", |_, this, size: usize| {
            Ok(IndexWriter(this.0.writer(size).expect("Error getting writer")))
        });
    }
}

struct IndexWriter(tantivy::IndexWriter);

impl UserData for IndexWriter {}

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    let tan = lua.create_table()?;
    tan.set("new_schema_builder", lua.create_function(|_, _: ()| {
        Ok(SchemaBuilder(Default::default()))
    })?)?;

    let globals = lua.globals();
    globals.set("tan", tan)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use rlua::{Lua, Value};
    static SCRIPT: &str = r##"
    local builder = tan.new_schema_builder()
    "##;
    #[test]
    fn test() {
        let lua = Lua::new();
        super::init(&lua).unwrap();

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().as_os_str().to_str().unwrap();
        let globals = lua.globals();
        globals.set("index_path", path).unwrap();
        lua.exec::<_, Value>(SCRIPT, None).unwrap();
    }
}
