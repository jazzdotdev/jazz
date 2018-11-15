use rlua::prelude::*;
use rlua::UserData;
use rlua::UserDataMethods;

fn combine<T: std::ops::BitOr<Output = T> + Clone>(vec: Vec<T>) -> T {
    assert!(!vec.is_empty());
    let mut res = vec[0].clone();
    for i in vec {
        res = res | i;
    }
    res
}

#[derive(Clone)]
struct IntOptions(tantivy::schema::IntOptions);

impl std::ops::BitOr for IntOptions {
    type Output = IntOptions;
    fn bitor(self, other: IntOptions) -> IntOptions {
        IntOptions(self.0 | other.0)
    }
}

impl UserData for IntOptions {}

#[derive(Clone)]
struct TextOptions(tantivy::schema::TextOptions);

impl UserData for TextOptions {}

impl std::ops::BitOr for TextOptions {
    type Output = TextOptions;
    fn bitor(self, other: TextOptions) -> TextOptions {
        TextOptions(self.0 | other.0)
    }
}

#[derive(Clone)]
struct Field(tantivy::schema::Field);
impl UserData for Field {}

struct SchemaBuilder(Option<tantivy::schema::SchemaBuilder>);

impl UserData for SchemaBuilder {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "add_u64_field",
            |_, this, (name, options): (String, Vec<IntOptions>)| {
                let option = combine(options);
                let this = this.0.as_mut().expect("Value already moved");
                let field = this.add_u64_field(&name, option.0);
                Ok(Field(field))
            },
        );

        methods.add_method_mut(
            "add_i64_field",
            |_, this, (name, options): (String, Vec<IntOptions>)| {
                let option = combine(options);
                let this = this.0.as_mut().expect("Value already moved");
                let field = this.add_i64_field(&name, option.0);
                Ok(Field(field))
            },
        );

        methods.add_method_mut(
            "add_text_field",
            |_, this, (name, options): (String, Vec<TextOptions>)| {
                let option = combine(options);
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
            Ok(IndexWriter(
                this.0.writer(size).expect("Error getting writer"),
            ))
        });
    }
}

struct IndexWriter(tantivy::IndexWriter);

impl UserData for IndexWriter {}

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    let tan = lua.create_table()?;
    tan.set(
        "new_schema_builder",
        lua.create_function(|_, _: ()| Ok(SchemaBuilder(Some(Default::default()))))?,
    )?;

    tan.set("TEXT", TextOptions(tantivy::schema::TEXT))?;
    tan.set("STRING", TextOptions(tantivy::schema::STRING))?;
    tan.set("STORED", TextOptions(tantivy::schema::STORED))?;
    tan.set("INT_STORED", IntOptions(tantivy::schema::INT_STORED))?;
    tan.set("INT_INDEXED", IntOptions(tantivy::schema::INT_INDEXED))?;
    tan.set("FAST", IntOptions(tantivy::schema::FAST))?;
    tan.set("FACET_SEP_BYTE", tantivy::schema::FACET_SEP_BYTE)?;

    let globals = lua.globals();
    globals.set("tan", tan)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use rlua::{Lua, Value};
    static SCRIPT: &str = r##"
    local builder = tan.new_schema_builder()
    builder:add_text_field("title", {tan.TEXT, tan.STORED})
    builder:add_text_field("body", {tan.TEXT})
    local schema = builder:build()
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
