use napi::*;

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
pub fn get_string_opt(ctx: &CallContext, index: usize) -> Result<Option<String>> {
    let val: Option<JsString> = get_value_opt(&ctx, ValueType::String, index)?;
    let val = match val {
        Some(v) => v,
        None => return Ok(None),
    };

    Ok(Some(val.into_utf8()?.into_owned()?))
}

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
        vt if vt == value_type => return Ok(Some(unsafe { val.cast() })),
        ValueType::Undefined => return Ok(None),
        ValueType::Null => return Ok(None),
        _ => return Err(Error::from_reason("argument has wrong type".to_string())),
    }
}