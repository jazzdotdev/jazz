use rlua::{UserData, UserDataMethods, Error as LuaError, Lua, Value};
use rust_sodium::crypto::box_;
use base64;
use std::cell::Ref;
use error::Error;

pub struct KeyPair {
    secret: box_::SecretKey,
    public: box_::PublicKey,
}

pub struct Nonce(box_::Nonce);

/// Returns a new KeyPair
pub fn new_keypair(_lua: &Lua, _: ()) -> Result<KeyPair, LuaError> {
    let (pk, sk) = box_::gen_keypair();
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
        secret: box_::SecretKey::from_slice(&secret).ok_or(LuaError::external(Error::InvalidKeys))?,
        public: box_::PublicKey::from_slice(&public).ok_or(LuaError::external(Error::InvalidKeys))?
    };
    Ok(keypair)
}

/// Constructs and returns a Nonce object from passed in `nonce_str` base64 string
pub fn load_nonce(_lua: &Lua, nonce_str: String) -> Result<Nonce, LuaError> {
    let nonce_bytes = base64::decode(&nonce_str).map_err(LuaError::external)?;
    box_::Nonce::from_slice(&nonce_bytes).map(|n| Nonce(n)).ok_or(LuaError::external(Error::InvalidNonce))
}

/// Returns a new nonce
pub fn new_nonce(_lua: &Lua, _: ()) -> Result<Nonce, LuaError> {
    let nonce = box_::gen_nonce();
    Ok(Nonce(nonce))
}

fn get_nonce<'a>(val: &'a Option<Value<'a>>) -> Result<Ref<'a, Nonce>, LuaError> {
    let user_data = match val {
        Some(Value::UserData(data)) => Ok(data),
        _ => Err(LuaError::external(Error::InvalidNonceObject))
    }?;

    user_data.borrow::<Nonce>().map_err(LuaError::external)
}

// Encrypts and authenticates a message `msg` using the senders secret key, the receivers public key and a nonce `nonce_value`. It returns a base64 ciphertext.
pub fn seal(_lua: &Lua, this: &KeyPair, (msg, nonce_value): (String, Option<Value>)) -> Result<String, LuaError> {
    let nonce = get_nonce(&nonce_value).map_err(LuaError::external)?;
    let cipher = box_::seal(msg.as_bytes(), &nonce.0, &this.public, &this.secret);
    Ok(base64::encode(&cipher))
}

/// `open()` verifies and decrypts a ciphertext `msg` using the receiver's secret key,
/// the senders public key `pk`, and a nonce `nonce_value`. It returns a plaintext.
/// If the ciphertext fails verification, `open()` returns `Err(())`.
/// msg is expected to be encoded with base64
pub fn open(_lua: &Lua, this: &KeyPair, (msg, nonce_value): (String, Option<Value>)) -> Result<String, LuaError> {
    let nonce = get_nonce(&nonce_value).map_err(LuaError::external)?;
    let msg_bytes = base64::decode(&msg).map_err(LuaError::external)?;
    let text_bytes = box_::open(&msg_bytes, &nonce.0, &this.public, &this.secret)
        .map_err(|_| LuaError::external(Error::FailedToDecrypt))?;
    String::from_utf8(text_bytes).map_err(LuaError::external)
}


/// Returns the authentication tag as a base64 encoded string and the ciphertext.
/// `seal_detached()` encrypts and authenticates a message `msg` using the senders secret key,
/// the receivers public key and a nonce `nonce_value`.
pub fn seal_detached(_lua: &Lua, this: &KeyPair, (msg, nonce_value): (String, Option<Value>)) -> Result<(String, String), LuaError> {
    let msg: &mut [u8] = &mut msg.into_bytes().clone();
    let nonce = get_nonce(&nonce_value).map_err(LuaError::external)?;
    let tag = box_::seal_detached(msg, &nonce.0, &this.public, &this.secret);

    Ok((base64::encode(&msg), base64::encode(tag.0.as_ref())))
}

/// Returns base64 encoded keypair in a 2-tuple
pub fn get_keys(_lua: &Lua, this: &KeyPair, _: ()) -> Result<(String, String), LuaError> {
    let secret = base64::encode(this.secret.0.as_ref());
    let public = base64::encode(this.public.0.as_ref());
    Ok((secret, public))
}

impl UserData for KeyPair {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_keys", get_keys);
        methods.add_method("seal", seal);
        methods.add_method("open", open);
        methods.add_method("seal_detached", seal_detached);
    }
}

/// returns the nonce base64 encoded
pub fn nonce_to_string(_lua: &Lua, this: &Nonce, _: ()) -> Result<String, LuaError> {
    Ok(base64::encode((this.0).0.as_ref()))
}

impl UserData for Nonce {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("tostring", nonce_to_string);
    }
}