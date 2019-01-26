use rlua::prelude::*;
use std::net::Ipv4Addr;
use crate::bindings::net::ipv6::LuaIpv6;

pub struct LuaIpv4(pub Ipv4Addr);

impl LuaUserData for LuaIpv4 {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("octets", |_, this: &LuaIpv4, _: ()| {
            Ok(this.0.octets().to_vec())
        });
        methods.add_method("is_unspecified", |_, this: &LuaIpv4, _: ()| {
            Ok(this.0.is_unspecified())
        });
        methods.add_method("is_loopback", |_, this: &LuaIpv4, _: ()| {
            Ok(this.0.is_loopback())
        });
        methods.add_method("is_private", |_, this: &LuaIpv4, _: ()| {
            Ok(this.0.is_private())
        });
        methods.add_method("is_link_local", |_, this: &LuaIpv4, _: ()| {
            Ok(this.0.is_link_local())
        });
        methods.add_method("is_multicast", |_, this: &LuaIpv4, _: ()| {
            Ok(this.0.is_multicast())
        });
        methods.add_method("is_broadcast", |_, this: &LuaIpv4, _: ()| {
            Ok(this.0.is_broadcast())
        });
        methods.add_method("is_documentation", |_, this: &LuaIpv4, _: ()| {
            Ok(this.0.is_documentation())
        });
        methods.add_method("to_ipv6_compatible", |_, this: &LuaIpv4, _: ()| {
            Ok(LuaIpv6(this.0.to_ipv6_compatible()))
        });
        methods.add_method("to_ipv6_mapped", |_, this: &LuaIpv4, _: ()| {
            Ok(LuaIpv6(this.0.to_ipv6_mapped()))
        });
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, _: ()| {
            Ok(this.0.to_string())
        });
    }
}

pub fn init(lua: &Lua) -> crate::Result<()> {
    let module = lua.create_table()?;

    module.set("new", lua.create_function( |_, ip: String| {
        ip.parse().map(LuaIpv4).map_err(LuaError::external)
    })? )?;

    module.set("from_table", lua.create_function( |_, ip: Vec<u8>| {
        if ip.len() != 4 {
            return Ok(None);
        }
        let (a, b, c, d) = (
            ip.get(0).unwrap_or(&0),
            ip.get(1).unwrap_or(&0),
            ip.get(2).unwrap_or(&0),
            ip.get(3).unwrap_or(&0));

        Ok(Some(LuaIpv4(Ipv4Addr::new(*a, *b, *c, *d))))
    })? )?;

    lua.globals().set("ipv4", module)?;

    Ok(())
}