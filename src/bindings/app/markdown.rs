use comrak::{markdown_to_html, ComrakOptions};
use rlua::prelude::*;
use std::collections::HashMap;

const HARDBREAKS: &str = "hardbreaks";
const SMART: &str = "smart";
const GITHUB_PRE_LANG: &str = "github_pre_lang";
const WIDTH: &str = "width";
const DEFAULT_INFO_STRING: &str = "default_info_string";
const UNSAFE: &str = "unsafe";
const EXT_STRIKETHROUGH: &str = "ext_strikethrough";
const EXT_TAGFILTER: &str = "ext_tagfilter";
const EXT_TABLE: &str = "ext_table";
const EXT_AUTOLINK: &str = "ext_autolink";
const EXT_TASKLIST: &str = "ext_tasklist";
const EXT_SUPERSCRIPT: &str = "ext_superscript";
const EXT_HEADER_IDS: &str = "ext_header_ids";
const EXT_FOOTNOTES: &str = "ext_footnotes";

fn comrak_options_from_table(table: &HashMap<String, LuaValue>) -> Result<ComrakOptions, LuaError> {
    let mut options = ComrakOptions {
        ..ComrakOptions::default()
    };

    for (k, v) in table.iter() {
        match (k.as_str(), v) {
            (HARDBREAKS, LuaValue::Boolean(val)) => options.hardbreaks = *val,
            (SMART, LuaValue::Boolean(val)) => options.smart = *val,
            (GITHUB_PRE_LANG, LuaValue::Boolean(val)) => options.smart = *val,
            (WIDTH, LuaValue::Integer(val)) => options.width = *val as usize,
            (DEFAULT_INFO_STRING, LuaValue::String(val)) => {
                options.default_info_string = Some(val.to_str()?.to_string())
            }
            (UNSAFE, LuaValue::Boolean(val)) => options.unsafe_ = *val,
            (EXT_STRIKETHROUGH, LuaValue::Boolean(val)) => options.ext_strikethrough = *val,
            (EXT_TAGFILTER, LuaValue::Boolean(val)) => options.ext_tagfilter = *val,
            (EXT_TABLE, LuaValue::Boolean(val)) => options.ext_table = *val,
            (EXT_AUTOLINK, LuaValue::Boolean(val)) => options.ext_autolink = *val,
            (EXT_TASKLIST, LuaValue::Boolean(val)) => options.ext_tasklist = *val,
            (EXT_SUPERSCRIPT, LuaValue::Boolean(val)) => options.ext_superscript = *val,
            (EXT_HEADER_IDS, LuaValue::String(val)) => {
                options.ext_header_ids = Some(val.to_str()?.to_string())
            }
            (EXT_FOOTNOTES, LuaValue::Boolean(val)) => options.ext_footnotes = *val,
            (k, v) => unimplemented!(
                "Unknown option {:?}, with value {:?}, passed to markdown_to_html",
                k,
                v
            ),
        }
    }

    Ok(options)
}

pub fn init(lua: &Lua) -> crate::Result<()> {
    let render_markdown = lua.create_function(
        |_, (markdown_str, options): (String, Option<HashMap<String, LuaValue>>)| {
            let html_string = match options {
                Some(options) => {
                    let opts = comrak_options_from_table(&options)?;
                    markdown_to_html(&markdown_str, &opts)
                }
                None => markdown_to_html(&markdown_str, &ComrakOptions::default()),
            };

            Ok(html_string)
        },
    )?;

    lua.globals().set("markdown_to_html", render_markdown)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_html() {
        let lua = Lua::new();
        init(&lua).unwrap();
        let result = lua.exec::<_, LuaValue>(r#"return markdown_to_html("Hello, **世界**!")"#, None);

        match result {
            Ok(LuaValue::String(html)) => {
                assert_eq!(html, "<p>Hello, <strong>世界</strong>!</p>\n")
            }
            _ => unimplemented!("Unexpected value returned from markdown_to_html"),
        }
    }

    #[test]
    fn test_markdown_to_html_with_options() {
        let lua = Lua::new();
        init(&lua).unwrap();
        let result = lua.exec::<_, LuaValue>(
            r#"return markdown_to_html("Hello, **世界**!<script></script>", {unsafe = false})"#,
            None,
        );

        match result {
            Ok(LuaValue::String(html)) => {
                let s = html.to_str().unwrap().to_string();
                //                println!("html = {}", s);
                assert!(s.contains("<p>Hello, <strong>世界</strong>!"));
                assert!(!s.contains("<script>"));
            }
            _ => unimplemented!("Unexpected value returned from markdown_to_html"),
        }
    }
}
