mod hash;
mod sign;
mod random;
mod box_;
mod checksumdir;

use rlua::{Error as LuaError, Lua};
use sodiumoxide;
use crate::error::Error;

pub fn init(lua: &Lua) -> crate::Result<()> {
    sodiumoxide::init().map_err(|_| LuaError::external(Error::SodiumInitFailure))?;
    lua.context(|lua| {
        let crypto = lua.create_table()?;

        crypto.set("random_bytes", lua.create_function(random::random_bytes)?)?;
        crypto.set("hash", lua.create_function(hash::hash)?)?;
        crypto.set("blake2b", lua.create_function(hash::blake2_hash)?)?;
        crypto.set("checksumdir", lua.create_function(checksumdir::checksum)?)?;

        let sign = lua.create_table()?;
        sign.set("new_keypair", lua.create_function(sign::new_keypair)?)?;
        sign.set("load_secret", lua.create_function(sign::load_secret)?)?;
        sign.set("load_public", lua.create_function(sign::load_public)?)?;
        crypto.set("sign", sign)?;

        let box_ = lua.create_table()?;
        box_.set("new_keypair", lua.create_function(box_::new_keypair)?)?;
        box_.set("new_nonce", lua.create_function(box_::new_nonce)?)?;
        box_.set("load_keypair", lua.create_function(box_::load_keypair)?)?;
        box_.set("load_nonce", lua.create_function(box_::load_nonce)?)?;
        crypto.set("box", box_)?;

        lua.globals().set("crypto", crypto)?;

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rlua::Value;

    #[test]
    fn test_box_() {
        let lua = Lua::new();
        init(&lua).unwrap();
        lua.context(|lua| {
            let result = lua.load(
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
            "#).eval::<Value>();
            println!("returned {:?}", result);
        })
    }

    #[test]
    fn test_crypto() {
        let lua = Lua::new();
        init(&lua).unwrap();
        lua.context(|lua| {
            let result = lua.load(
                r#"
                local random_bytes = crypto.random_bytes(8)
                print("random_bytes length=" .. #random_bytes)

                local source = "this is a test!"
                print( "source=" .. source )
                local hash = crypto.hash(source)
                print("SHA512=" .. hash)
                local blakehash = crypto.blake2b(source)
                print("BLAKE2B=" .. blakehash)

                return true
            "#).eval::<Value>();
            println!("returned {:?}", result);
        })
    }

    #[test]
    fn lua_sign() {
        let lua = Lua::new();
        init(&lua).unwrap();
        lua.context(|lua| {
            let result = lua.load(
                r#"
                local secret, public = crypto.sign.new_keypair()
                print( "secret", secret )
                print( "public", public )

                local secret2 = crypto.sign.load_secret( tostring(secret) )
                local public2 = crypto.sign.load_public( tostring(public) )
                local source = "this is a test!"
                print( "source=" .. source )
                local signed = secret2:sign(source)
                print("signed=" .. signed)

                local verified = public2:verify(signed)
                print("verified=" .. verified)

                local signature = secret:sign_detached(source)
                print("signature=" .. signature)

                local is_sig_valid = public:verify_detached(source, signature)
                print("is_sig_valid=" .. tostring(is_sig_valid))

                local secret3 = crypto.sign.load_secret(
                    "+qEY1pRSYy7gTfJ58GLrDQTuhgiTf49Cy9yEgvix3vHGkq2b5t55E36RPtVYgnTn+2SF0Of8nEeVOyTvcvlnnQ=="
                )
                local public3 = crypto.sign.load_public(
                    "xpKtm+beeRN+kT7VWIJ05/tkhdDn/JxHlTsk73L5Z50="
                )

                source = "I'm going to get signed"
                print( "source=" .. source )
                local signed = secret3:sign(source)
                print("signed=" .. signed)
                local verified = public3:verify(signed)
                print("verified=" .. verified)

                return true
            "#).eval::<Value>();
            println!("returned {:?}", result);
        })

    }
}



