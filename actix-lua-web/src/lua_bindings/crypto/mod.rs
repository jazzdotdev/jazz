use rlua::{Error as LuaError, Lua, Value};
use rust_sodium;

mod hash;
mod sign;
mod random;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Failed to initialize libsodium.")]
    SodiumInitFailure,
}

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    match rust_sodium::init() {
        Ok(_) => {},
        Err(_) => return Err(LuaError::external(Error::SodiumInitFailure))
    };

    let crypto = lua.create_table()?;
    let sign = lua.create_table()?;

    sign.set("new_keypair", lua.create_function(sign::new_keypair)?)?;
    sign.set("load_keypair", lua.create_function(sign::load_keypair)?)?;
    crypto.set("sign", sign)?;

    crypto.set("random_bytes", lua.create_function(random::random_bytes)?)?;
    crypto.set("hash", lua.create_function(hash::hash)?)?;

    lua.globals().set("crypto", crypto)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let lua = Lua::new();
        init(&lua).unwrap();
        let result = lua.exec::<Value>(
            r#"
                local keypair = crypto.sign.new_keypair()
                local secret, public = keypair:get_keys()
                print( "secret=" .. secret )
                print( "public=" .. public )

                local keypair2 = crypto.sign.load_keypair(secret, public)
                local source = "this is a test!"
                print( "source=" .. source )
                local signed = keypair2:sign(source)
                print("signed=" .. signed)

                local verified = keypair2:verify(signed)
                print("verified=" .. verified)

                local signature = keypair:sign_detached(source)
                print("signature=" .. signature)

                local is_sig_valid = keypair:verify_detached(source, signature)
                print("is_sig_valid=" .. tostring(is_sig_valid))

                local random_bytes = crypto.random_bytes(8)
                print("random_bytes length=" .. #random_bytes)

                local hash = crypto.hash(source)
                print("hash=" .. hash)

                return true
            "#, None);

        println!("returned signed {:?}", result);
    }
}



