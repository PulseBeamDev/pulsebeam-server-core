#![deny(clippy::all)]
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn create_token(id: String, signing_key_hex: String) -> Result<String> {
  server_sdk_core::create_token(&id, &signing_key_hex)
    .map_err(|e| napi::Error::from_reason(e.to_string()))
}
