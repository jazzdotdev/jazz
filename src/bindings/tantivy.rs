use rlua::prelude::*;
use rlua::UserData;
use rlua::UserDataMethods;
use tantivy;

fn combine<T: ::std::ops::BitOr<Output = T> + Clone>(vec: Vec<T>) -> T {
    assert!(!vec.is_empty());
    let mut res = vec[0].clone();
    for i in vec {
        res = res | i;
    }
    res
}

#[derive(Clone)]
struct IntOptions(tantivy::schema::IntOptions);

impl ::std::ops::BitOr for IntOptions {
    type Output = IntOptions;
    fn bitor(self, other: IntOptions) -> IntOptions {
        IntOptions(self.0 | other.0)
    }
}

impl UserData for IntOptions {}

#[derive(Clone)]
struct TextOptions(tantivy::schema::TextOptions);

impl UserData for TextOptions {}

impl ::std::ops::BitOr for TextOptions {
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

#[derive(Clone)]
struct Schema(tantivy::schema::Schema);

impl UserData for Schema {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_field", |_, this, name: String| {
            Ok(this.0.get_field(&name).map(|x| Field(x)))
        });
        methods.add_method("to_json", |_, this, doc: Document| {
            Ok(this.0.to_json(&doc.0))
        });
    }
}

#[derive(Clone)]
struct Index(tantivy::Index);

impl UserData for Index {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("writer", |_, this, size: usize| {
            Ok(IndexWriter(
                this.0.writer(size).expect("Error getting writer"),
            ))
        });
        methods.add_method("load_searchers", |_, this, _: ()| {
            this.0.load_searchers().expect("load_searchers failed");
            Ok(())
        });
        methods.add_method(
            "search",
            |_, this, (p, q, a): (QueryParser, String, ::rlua::AnyUserData)| {
                if a.is::<TopCollector>() {
                    Ok(this.search(p, q, &mut *a.borrow_mut::<TopCollector>().unwrap()))
                } else {
                    Err(::rlua::Error::UserDataTypeMismatch)
                }
            },
        );
    }
}

impl Index {
    fn search<T: GetCollector>(&self, p: QueryParser, q: String, c: &mut T) -> Vec<Document> {
        let searcher = self.0.searcher();
        let query = p.0.parse_query(&q).unwrap();
        searcher.search(&*query, c.get_collector()).unwrap();
        c.get_doc_addresses()
            .into_iter()
            .map(|x| Document(searcher.doc(x).unwrap()))
            .collect()
    }
}

trait GetCollector {
    type Output: tantivy::collector::Collector;
    fn get_collector(&mut self) -> &mut Self::Output;
    fn get_doc_addresses(&self) -> Vec<tantivy::DocAddress>;
}

#[derive(Clone)]
struct TopCollector(::std::sync::Arc<tantivy::collector::TopCollector>);
impl UserData for TopCollector {}
impl GetCollector for TopCollector {
    type Output = tantivy::collector::TopCollector;
    fn get_collector(&mut self) -> &mut Self::Output {
        ::std::sync::Arc::get_mut(&mut self.0).unwrap()
    }
    fn get_doc_addresses(&self) -> Vec<tantivy::DocAddress> {
        self.0.docs()
    }
}

#[derive(Clone)]
struct QueryParser(::std::sync::Arc<tantivy::query::QueryParser>);
impl UserData for QueryParser {}

struct IndexWriter(tantivy::IndexWriter);

impl UserData for IndexWriter {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("add_document", |_, this, doc: Document| {
            Ok(this.0.add_document(doc.0))
        });
        methods.add_method_mut("commit", |_, this, _: ()| {
            Ok(this.0.commit().expect("IndexWriter commit failed"))
        });
        methods.add_method_mut("delete_term", |_, this, t: Term| {
            Ok(this.0.delete_term(t.0))
        });
    }
}

#[derive(Clone)]
struct Document(tantivy::Document);

impl UserData for Document {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("add_text", |_, this, (f, t): (Field, String)| {
            this.0.add_text(f.0, &t);
            Ok(())
        });

        use tantivy::schema::Value;

        fn get_value (tan: Value) -> LuaValue {
            match tan {
                Value::Str(s) => LuaValue::String(s),
                Value::U64(n) => LuaValue::Integer(n as lua::Integer),
                Value::I64(n) => LuaValue::Integer(n as lua::Integer),
                _ => unimplemented!()
            }
        }

        methods.add_method_mut("get_first", |_, this, f: Field| {
            Ok(this.0.get_first(f.0).map(get_value))
        });
        methods.add_method_mut("get_all", |_, this, f: Field| {
            Ok(this.0.get_first(f.0).map(get_value))
        });
    }
}

#[derive(Clone)]
struct Term(tantivy::Term);

impl UserData for Term {}

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

    tan.set(
        "index_in_ram",
        lua.create_function(|_, schema: Schema| {
            Ok(Index(tantivy::Index::create_in_ram(schema.0)))
        })?,
    )?;
    tan.set(
        "index_in_dir",
        lua.create_function(|_, (path, schema): (String, Schema)| {
            Ok(Index(
                tantivy::Index::create_in_dir(&path, schema.0).expect("create_in_dir failed"),
            ))
        })?,
    )?;

    tan.set(
        "new_document",
        lua.create_function(|_, _: ()| Ok(Document(Default::default())))?,
    )?;

    tan.set(
        "top_collector_with_limit",
        lua.create_function(|_, l: usize| {
            Ok(TopCollector(::std::sync::Arc::new(
                tantivy::collector::TopCollector::with_limit(l),
            )))
        })?,
    )?;
    tan.set(
        "query_parser_for_index",
        lua.create_function(|_, (i, f): (Index, Vec<Field>)| {
            let f = f.into_iter().map(|x| x.0).collect();
            Ok(QueryParser(::std::sync::Arc::new(
                tantivy::query::QueryParser::for_index(&i.0, f),
            )))
        })?,
    )?;

    tan.set(
        "term_from_field_text",
        lua.create_function(|_, (f, s): (Field, String)| {
            Ok(Term(tantivy::Term::from_field_text(f.0, &s)))
        })?,
    )?;

    let globals = lua.globals();
    globals.set("tan", tan)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use rlua::{Lua, Value};
    static SCRIPT: &str = r##"
    local builder = tan.new_schema_builder()
    builder:add_text_field("id", {tan.STRING, tan.STORED})
    builder:add_text_field("title", {tan.TEXT, tan.STORED})
    builder:add_text_field("body", {tan.TEXT})
    local schema = builder:build()
    local index = tan.index_in_dir(index_path, schema)
    local index_writer = index:writer(50000000)
    local id = schema:get_field("id")
    local title = schema:get_field("title")
    local body = schema:get_field("body")

    local doc
    doc = tan.new_document()
    doc:add_text(id, "1")
    doc:add_text(title, "Lorem ipsum dolor sit amet")
    doc:add_text(body, "consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.")
    index_writer:add_document(doc)

    doc = tan.new_document()
    doc:add_text(id, "2")
    doc:add_text(title, "Ut enim ad minim veniam")
    doc:add_text(body, "quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.")
    index_writer:add_document(doc)

    index_writer:commit()
    index:load_searchers()

    local parser = tan.query_parser_for_index(index, {title, body})
    local coll = tan.top_collector_with_limit(10)
    local result = index:search(parser, "laboris", coll)
    for i = 1, #result do
        print(schema:to_json(result[i]))
    end

    local term = tan.term_from_field_text(id, "1")
    index_writer:delete_term(term)
    index_writer:commit()

    index:load_searchers()
    coll = tan.top_collector_with_limit(10)
    result = index:search(parser, "Lorem ipsum dolor sit amet", coll)
    assert(#result == 0)
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
