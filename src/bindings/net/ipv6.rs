use rlua::prelude::*;
use std::net::Ipv6Addr;
use crate::bindings::net::ipv4::LuaIpv4;

pub struct LuaIpv6(pub Ipv6Addr);

impl LuaUserData for LuaIpv6 {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("segments", |_, this: &LuaIpv6, _: ()| {
            Ok(this.0.octets().to_vec())
        });
        methods.add_method("is_unspecified", |_, this: &LuaIpv6, _: ()| {
            Ok(this.0.is_unspecified())
        });
        methods.add_method("is_loopback", |_, this: &LuaIpv6, _: ()| {
            Ok(this.0.is_loopback())
        });
        methods.add_method("is_multicast", |_, this: &LuaIpv6, _: ()| {
            Ok(this.0.is_multicast())
        });
        methods.add_method("octets", |_, this: &LuaIpv6, _: ()| {
            Ok(this.0.octets().to_vec())
        });
        methods.add_method("to_ipv4", |_, this: &LuaIpv6, _: ()| {
            Ok(this.0.to_ipv4().map(LuaIpv4))
        });
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, _: ()| {
            Ok(this.0.to_string())
        });
    }
}

pub fn init(lua: &Lua) -> crate::Result<()> {
    let module = lua.create_table()?;

    module.set("new", lua.create_function( |_, ip: String| {
        ip.parse().map(LuaIpv6).map_err(LuaError::external)
    })? )?;

    module.set("from_table", lua.create_function( |_, ip: Vec<u16>| {
        if ip.len() != 8 {
            return Ok(None);
        }
        let (a, b, c, d, e, f, g, h) = (
            ip.get(0).unwrap_or(&0),
            ip.get(1).unwrap_or(&0),
            ip.get(2).unwrap_or(&0),
            ip.get(3).unwrap_or(&0),
            ip.get(4).unwrap_or(&0),
            ip.get(5).unwrap_or(&0),
            ip.get(6).unwrap_or(&0),
            ip.get(7).unwrap_or(&0));

        Ok(Some(LuaIpv6(Ipv6Addr::new(*a, *b, *c, *d, *e, *f, *g, *h))))
    })? )?;

    lua.globals().set("ipv6", module)?;

    Ok(())
}