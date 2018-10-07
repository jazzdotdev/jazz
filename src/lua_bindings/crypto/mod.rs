use rlua::{Error as LuaError, Lua, Value};
use rust_sodium;

mod hash;
mod sign;
mod random;
mod box_;

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

    crypto.set("random_bytes", lua.create_function(random::random_bytes)?)?;
    crypto.set("hash", lua.create_function(hash::hash)?)?;

    let sign = lua.create_table()?;
    sign.set("new_keypair", lua.create_function(sign::new_keypair)?)?;
    sign.set("load_keypair", lua.create_function(sign::load_keypair)?)?;
    crypto.set("sign", sign)?;

    let box_ = lua.create_table()?;
    box_.set("new_keypair", lua.create_function(box_::new_keypair)?)?;
    box_.set("new_nonce", lua.create_function(box_::new_nonce)?)?;
    box_.set("load_keypair", lua.create_function(box_::load_keypair)?)?;
    box_.set("load_nonce", lua.create_function(box_::load_nonce)?)?;
    crypto.set("box", box_)?;

    lua.globals().set("crypto", crypto)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_() {
        let lua = Lua::new();
        init(&lua).unwrap();
        let result = lua.exec::<_, Value>(
            r#"
                local nonce1 = crypto.box.new_nonce()
                local nonce_str = nonce1:tostring()
                print("nonce_str=" .. nonce_str)
                local nonce2 = crypto.box.load_nonce(nonce_str)

                local keypair1 = crypto.box.new_keypair()
                local secret, public = keypair1:get_keys()
                print( "secret=" .. secret )
                print( "public=" .. public )

                local keypair2 = crypto.box.load_keypair(secret, public)
                local source = "Hello World!"
                print( "source=" .. source )
                local sealed = keypair2:seal(source, nonce2)
                print( "sealed=" .. sealed )
                local openned = keypair2:open(sealed, nonce2)
                print( "openned=" .. openned )

                local source2 = "Good morning!"
                print( "source2=" .. source2 )
                local detach_sealed, tag = keypair2:seal_detached(source, nonce2)
                print( "detach_sealed =" .. sealed .. " tag= " .. tag)

                return true
            "#, None);

        println!("returned {:?}", result);
    }

    #[test]
    fn test_crypto() {
        let lua = Lua::new();
        init(&lua).unwrap();
        let result = lua.exec::<_, Value>(
            r#"
                local random_bytes = crypto.random_bytes(8)
                print("random_bytes length=" .. #random_bytes)

                local source = "this is a test!"
                print( "source=" .. source )
                local hash = crypto.hash(source)
                print("hash=" .. hash)

                return true
            "#, None);

        println!("returned {:?}", result);
    }

    #[test]
    fn test_sign() {
        let lua = Lua::new();
        init(&lua).unwrap();
        let result = lua.exec::<_, Value>(
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

                local secret3 = "+qEY1pRSYy7gTfJ58GLrDQTuhgiTf49Cy9yEgvix3vHGkq2b5t55E36RPtVYgnTn+2SF0Of8nEeVOyTvcvlnnQ=="
                local public3 = "xpKtm+beeRN+kT7VWIJ05/tkhdDn/JxHlTsk73L5Z50="
                local keypair3 = crypto.sign.load_keypair(secret3, public3)
                source = "I'm going to get signed"
                print( "source=" .. source )
                local signed = keypair3:sign(source)
                print("signed=" .. signed)
                local verified = keypair3:verify(signed)
                print("verified=" .. verified)

                return true
            "#, None);

        println!("returned {:?}", result);
    }
}



