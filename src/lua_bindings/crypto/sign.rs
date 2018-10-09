use rlua::prelude::*;
use rlua::{UserDataMethods, UserData, MetaMethod, Lua};
use rust_sodium::crypto::sign;
use base64;

pub struct LuaSecretKey (sign::SecretKey);
pub struct LuaPublicKey (sign::PublicKey);

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Failed to verify signed message.")]
    VerifyError,
    #[fail(display = "Failed to load key, data is invalid.")]
    InvalidKeys,
    #[fail(display = "Failed verify, signature is invalid.")]
    InvalidSignature,
}

/// Returns `msg` signed and base64 encoded
pub fn sign(_: &Lua, this: &LuaSecretKey, msg: String) -> LuaResult<String> {
    let signed_data = sign::sign(msg.as_bytes(), &this.0);
    Ok(base64::encode(&signed_data))
}

/// Returns verified (decrypted) `base64_msg`
/// Expects `base64_msg` to be base64 encoded encrypted msg
pub fn verify(_: &Lua, this: &LuaPublicKey, base64_msg: String) -> LuaResult<String> {
    let signed_msg = base64::decode(&base64_msg).map_err(LuaError::external)?;

    match sign::verify(&signed_msg, &this.0) {
        Ok(v) => {
            Ok(String::from_utf8(v).map_err(LuaError::external)?)
        }
        Err(_) => Err(LuaError::external(Error::VerifyError))
    }
}

/// Returns base64 encoded signature for `msg`
pub fn sign_detached(_: &Lua, this: &LuaSecretKey, msg: String) -> LuaResult<String> {
    let signature = sign::sign_detached(msg.as_bytes(), &this.0);
    Ok(base64::encode(signature.0.as_ref()))
}

/// Returns true/false if the given `signature` verifies for the given `msg`
/// Expects `signature` to be base64 encoded
pub fn verify_detached(_: &Lua, this: &LuaPublicKey, (msg, base64_signature): (String, String)) -> LuaResult<bool> {
    let signature_bytes = base64::decode(&base64_signature).map_err(LuaError::external)?;

    let signature = match sign::Signature::from_slice(&signature_bytes) {
        Some(signature) => Ok(signature),
        _ => Err(LuaError::external(Error::InvalidSignature))
    }?;

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
pub fn load_secret(_: &Lua, base64_key: String) -> LuaResult<LuaSecretKey> {
    match base64::decode(&base64_key) {
        Ok(vec) =>
            match sign::SecretKey::from_slice(&vec) {
                Some(key) => Ok(LuaSecretKey(key)),
                None => Err(LuaError::external(Error::InvalidKeys))
            },
        Err(e) => Err(LuaError::external(e))
    }
}

/// Constructs and returns a LuaPublicKey object from it's base64 string encoding
pub fn load_public(_: &Lua, base64_key: String) -> LuaResult<LuaPublicKey> {
    match base64::decode(&base64_key) {
        Ok(vec) =>
            match sign::PublicKey::from_slice(&vec) {
                Some(key) => Ok(LuaPublicKey(key)),
                None => Err(LuaError::external(Error::InvalidKeys))
            },
        Err(e) => Err(LuaError::external(e))
    }
}