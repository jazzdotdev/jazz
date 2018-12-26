use rlua::prelude::*;
use rlua::{UserDataMethods, UserData, MetaMethod, Lua};
use rust_sodium::crypto::sign;
use base64;
use error::Error;

pub struct LuaSecretKey (sign::SecretKey);
pub struct LuaPublicKey (sign::PublicKey);

/// Returns `msg` signed and base64 encoded
pub fn sign(_: &Lua, this: &LuaSecretKey, msg: String) -> Result<String, LuaError> {
    let signed_data = sign::sign(msg.as_bytes(), &this.0);
    Ok(base64::encode(&signed_data))
}

/// Returns verified (decrypted) `base64_msg`
/// Expects `base64_msg` to be base64 encoded encrypted msg
pub fn verify(_: &Lua, this: &LuaPublicKey, base64_msg: String) -> Result<String, LuaError> {
    let signed_msg = base64::decode(&base64_msg).map_err(LuaError::external)?;

    sign::verify(&signed_msg, &this.0)
        .and_then(|v| String::from_utf8(v).map_err(|_| ()) )
        .map_err(|_| LuaError::external(Error::VerifyError))

}

/// Returns base64 encoded signature for `msg`
pub fn sign_detached(_: &Lua, this: &LuaSecretKey, msg: String) -> Result<String, LuaError> {
    let signature = sign::sign_detached(msg.as_bytes(), &this.0);
    Ok(base64::encode(signature.0.as_ref()))
}

/// Returns true/false if the given `signature` verifies for the given `msg`
/// Expects `signature` to be base64 encoded
pub fn verify_detached(_: &Lua, this: &LuaPublicKey, (msg, base64_signature): (String, String)) -> Result<bool, LuaError> {
    let signature_bytes = base64::decode(&base64_signature).map_err(LuaError::external)?;

    let signature = sign::Signature::from_slice(&signature_bytes).ok_or(LuaError::external(Error::InvalidSignature))?;

    Ok(sign::verify_detached(&signature, msg.as_bytes(), &this.0))
}

impl UserData for LuaSecretKey {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("sign", sign);
        methods.add_method("sign_detached", sign_detached);
        methods.add_meta_method(MetaMethod::ToString, |_, this, _: ()| {
            Ok(base64::encode((this.0).0.as_ref()))
        });
    }
}

impl UserData for LuaPublicKey {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("verify", verify);
        methods.add_method("verify_detached", verify_detached);
        methods.add_meta_method(MetaMethod::ToString, |_, this, _: ()| {
            Ok(base64::encode((this.0).0.as_ref()))
        });
    }
}

/// Returns a new KeyPair
pub fn new_keypair(_lua: &Lua, _: ()) -> LuaResult<(LuaSecretKey, LuaPublicKey)> {
    let (pk, sk) = sign::gen_keypair();
    Ok((LuaSecretKey(sk), LuaPublicKey(pk)))
}

/// Constructs and returns a LuaSecretKey object from it's base64 string encoding
pub fn load_secret(_: &Lua, base64_key: String) -> Result<LuaSecretKey, LuaError> {
    base64::decode(&base64_key).map_err(Error::from)
        .and_then(|vec| sign::SecretKey::from_slice(&vec).ok_or(Error::InvalidKeys).map(LuaSecretKey))
        .map_err(LuaError::external)
}

/// Constructs and returns a LuaPublicKey object from it's base64 string encoding
pub fn load_public(_: &Lua, base64_key: String) -> Result<LuaPublicKey, LuaError> {
    base64::decode(&base64_key).map_err(Error::from)
        .and_then(|vec| sign::PublicKey::from_slice(&vec).ok_or(Error::InvalidKeys).map(LuaPublicKey))
        .map_err(LuaError::external)
} 