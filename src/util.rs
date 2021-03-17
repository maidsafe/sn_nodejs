use napi::*;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// Workaround to make sure an enum type T is serialised by napi-rs with an 'external tag'.
// This is so JS code can check what enum variant is actually returned.
// (see https://github.com/napi-rs/napi-rs/issues/507).
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Tag<T> {
    V(T),
}

/// Helper function to clone Arc to RwLock.
pub fn unwrap_arc<T: 'static>(ctx: &CallContext) -> Result<Arc<RwLock<T>>> {
    let obj: &Arc<RwLock<T>> = ctx.env.unwrap(&ctx.this()?)?;
    Ok(Arc::clone(&obj))
}

/// Retrieve exports from instance data. Should contain constructors.
pub fn get_constructor(env: &Env, s: &str) -> Result<JsFunction> {
    let exports: &mut Ref<()> = env
        .get_instance_data()?
        .ok_or_else(|| Error::from_reason("no instance data".to_string()))?;

    let exports: JsObject = env.get_reference_value(exports)?;
    exports.get_named_property(s)
}

/// See [`get_value_opt`]. Get String value from arg and decode.
///
/// # Errors
///
/// Err on out-of-bounds, String error or wrong argument type.
#[allow(dead_code)]
pub fn get_string_opt(ctx: &CallContext, index: usize) -> Result<Option<String>> {
    let val: Option<JsString> = get_value_opt(&ctx, ValueType::String, index)?;
    let val = match val {
        Some(v) => v,
        None => return Ok(None),
    };

    Ok(Some(val.into_utf8()?.into_owned()?))
}

#[allow(dead_code)]
pub fn get_array_opt(ctx: &CallContext, index: usize) -> Result<Option<JsObject>> {
    let val: Option<JsObject> = get_value_opt(&ctx, ValueType::Object, index)?;
    let val = match val {
        Some(v) => v,
        None => return Ok(None),
    };

    if !val.is_array()? {
        return Err(Error::from_reason("argument has wrong type".to_string()));
    }

    Ok(Some(val))
}

/// Helper function to get optional value at arg index of call context.
///
/// # Errors
///
/// Err on out-of-bounds or wrong argument type.
pub fn get_value_opt<T: NapiValue>(
    ctx: &CallContext,
    value_type: ValueType,
    index: usize,
) -> Result<Option<T>> {
    let val = ctx.get::<JsUnknown>(index)?;
    match val.get_type()? {
        vt if vt == value_type => Ok(Some(unsafe { val.cast() })),
        ValueType::Undefined => Ok(None),
        ValueType::Null => Ok(None),
        _ => Err(Error::from_reason("argument has wrong type".to_string())),
    }
}
