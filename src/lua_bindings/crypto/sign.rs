use rlua::{UserData, UserDataMethods, Error as LuaError, Lua};
use rust_sodium::crypto::sign;
use base64;

pub struct KeyPair {
    secret: sign::SecretKey,
    public: sign::PublicKey,
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Failed to verify signed message.")]
    VerifyError,
    #[fail(display = "Failed to load keys, data is invalid.")]
    InvalidKeys,
    #[fail(display = "Failed verify, signature is invalid.")]
    InvalidSignature,
}

/// Returns base64 encoded keypair in a 2-tuple
pub fn get_keys(_lua: &Lua, this: &KeyPair, _: ()) -> Result<(String, String), LuaError> {
    let secret = base64::encode(this.secret.0.as_ref());
    let public = base64::encode(this.public.0.as_ref());
    Ok((secret, public))
}

/// Returns `msg` signed and base64 encoded
pub fn sign(_lua: &Lua, this: &KeyPair, msg: String) -> Result<String, LuaError> {
    let signed_data = sign::sign(msg.as_bytes(), &this.secret);
    Ok(base64::encode(&signed_data))
}

/// Returns verified (decrypted) `base64_msg`
/// Expects `base64_msg` to be base64 encoded encrypted msg
pub fn verify(_lua: &Lua, this: &KeyPair, base64_msg: String) -> Result<String, LuaError> {
    let signed_msg = base64::decode(&base64_msg).map_err(LuaError::external)?;

    match sign::verify(&signed_msg, &this.public) {
        Ok(v) => {
            Ok(String::from_utf8(v).map_err(LuaError::external)?)
        }
        Err(_) => Err(LuaError::external(Error::VerifyError))
    }
}

/// Returns base64 encoded signature for `msg`
pub fn sign_detached(_lua: &Lua, this: &KeyPair, msg: String) -> Result<String, LuaError> {
    let signature = sign::sign_detached(msg.as_bytes(), &this.secret);
    Ok(base64::encode(signature.0.as_ref()))
}

/// Returns true/false if the given `signature` verifies for the given `msg`
/// Expects `signature` to be base64 encoded
pub fn verify_detached(_lua: &Lua, this: &KeyPair, (msg, base64_signature): (String, String)) -> Result<bool, LuaError> {
    let signature_bytes = base64::decode(&base64_signature).map_err(LuaError::external)?;

    let signature = match sign::Signature::from_slice(&signature_bytes) {
        Some(signature) => Ok(signature),
        _ => Err(LuaError::external(Error::InvalidSignature))
    }?;

    Ok(sign::verify_detached(&signature, msg.as_bytes(), &this.public))
}

impl UserData for KeyPair {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_keys", get_keys);
        methods.add_method("sign", sign);
        methods.add_method("verify", verify);
        methods.add_method("sign_detached", sign_detached);
        methods.add_method("verify_detached", verify_detached);
    }
}

/// Returns a new KeyPair
pub fn new_keypair(_lua: &Lua, _: ()) -> Result<KeyPair, LuaError> {
    let (pk, sk) = sign::gen_keypair();
    Ok(KeyPair {
        secret: sk,
        public: pk,
    })
}

/// Constructs and returns a KeyPair object from the secret and public keys, which are passed
/// in as strings in base64 encoding
pub fn load_keypair(_lua: &Lua, (secret_key_base64, public_key_base64): (String, String)) -> Result<KeyPair, LuaError> {
    let secret = base64::decode(&secret_key_base64).map_err(LuaError::external)?;
    let public = base64::decode(&public_key_base64).map_err(LuaError::external)?;

    let keypair = KeyPair {
        secret: match sign::SecretKey::from_slice(&secret) {
            Some(key) => Ok(key),
            _ => Err(LuaError::external(Error::InvalidKeys))
        }?,
        public: match sign::PublicKey::from_slice(&public) {
            Some(key) => Ok(key),
            _ => Err(LuaError::external(Error::InvalidKeys))
        }?,
    };

    Ok(keypair)
}
