use std::{io, string, time};
use serde_json;
use serde_yaml;
use rlua_serde;
use rlua;
use git2;
use base64;
use scl;
use rlua::Error as LuaError;

//Due to number of number of crates that have different errors, we will handle them this way for the time being which would just create a base for
//handling errors properly. The errors will be bound to change in the near future

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Failed to initialize libsodium.")]
    SodiumInitFailure,
    #[fail(display = "Failed to load nonce, data is invalid.")]
    InvalidNonce,
    #[fail(display = "Object passed is in not Nonce.")]
    InvalidNonceObject,
    #[fail(display = "Failed to decrypt.")]
    FailedToDecrypt,
    #[fail(display = "Failed to verify signed message.")]
    VerifyError,
    #[fail(display = "Failed to load key, data is invalid.")]
    InvalidKeys,
    #[fail(display = "Failed verify, signature is invalid.")]
    InvalidSignature,
    #[fail(display = "Internal error as occured.")]
    InternalError,
    #[fail(display = "{}", _0)]
    LuaError(rlua::Error),
    #[fail(display = "{}", _0)]
    IoError(io::Error),
    #[fail(display = "{}", _0)]
    JsonError(serde_json::Error),
    #[fail(display = "{}", _0)]
    YamlError(serde_yaml::Error),
    #[fail(display = "{}", _0)]
    LuaSerdeError(rlua_serde::error::Error),   
    #[fail(display = "{}", _0)]
    GitError(git2::Error),
    #[fail(display = "{}", _0)]
    Base64Error(base64::DecodeError),
    #[fail(display = "{}", _0)]
    StringUtf8Error(string::FromUtf8Error),
    #[fail(display = "{}", _0)]
    SclError(scl::Error),
    #[fail(display = "{}", _0)]
    SysTimeError(time::SystemTimeError),
    //TODO: This is only temp as a place holder for anything making use of a string for error messages
    #[fail(display = "{}", _0)]
    Other(String),
}

impl From<rlua::Error> for Error {
    fn from(err: rlua::prelude::LuaError) -> Error {
        Error::LuaError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JsonError(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::YamlError(err)
    }
}

impl From<rlua_serde::error::Error> for Error {
    fn from(err: rlua_serde::error::Error) -> Error {
        Error::LuaSerdeError(err)
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Error {
        Error::GitError(err)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Error {
        Error::Base64Error(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::StringUtf8Error(err)
    }
}

impl From<scl::Error> for Error {
    fn from(err: scl::Error) -> Error {
        Error::SclError(err)
    }
}

impl From<time::SystemTimeError> for Error {
    fn from(err: time::SystemTimeError) -> Error {
        Error::SysTimeError(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Other(err)
    }
}

pub fn create_lua_error <T> (err: T) -> LuaError
    where T: std::error::Error + Sync + Send + 'static {
    LuaError::ExternalError(
        std::sync::Arc::new(
            ::failure::Error::from_boxed_compat(
                Box::new(err)
            )
        )
    )
}