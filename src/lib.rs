use std::{net::SocketAddr, str::FromStr};

use napi::*;
use napi_derive::{js_function, module_exports};

use sn_api::{Keypair, Safe, SecretKey};
use tokio_compat_02::FutureExt;

use std::sync::Arc;
use tokio::sync::RwLock;

mod keys;

#[js_function(0)]
fn constructor(ctx: CallContext) -> Result<JsUndefined> {
    // TODO: Configurable arguments.
    let safe = Safe::new(None, std::time::Duration::from_secs(120));
    let safe = Arc::new(RwLock::new(safe));

    let mut this: JsObject = ctx.this_unchecked();
    ctx.env.wrap(&mut this, safe)?;

    ctx.env.get_undefined()
}

/// See [`get_value_opt`]. Get String value from arg and decode.
///
/// # Errors
///
/// Err on out-of-bounds, String error or wrong argument type.
fn get_string_opt(ctx: &CallContext, index: usize) -> Result<Option<String>> {
    let val: Option<JsString> = get_value_opt(&ctx, ValueType::String, index)?;
    let val = match val {
        Some(v) => v,
        None => return Ok(None),
    };

    Ok(Some(val.into_utf8()?.into_owned()?))
}

fn get_array_opt(ctx: &CallContext, index: usize) -> Result<Option<JsObject>> {
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
fn get_value_opt<T: NapiValue>(ctx: &CallContext, value_type: ValueType, index: usize) -> Result<Option<T>> {
    let val = ctx.get::<JsUnknown>(index)?;
    match val.get_type()? {
        vt if vt == value_type => return Ok(Some(unsafe { val.cast() } )),
        ValueType::Undefined => return Ok(None),
        ValueType::Null => return Ok(None),
        _ => return Err(Error::from_reason("argument has wrong type".to_string())),
    }
}

#[js_function(3)]
fn connect(ctx: CallContext) -> Result<JsObject> {
    let kp: Option<JsObject> = get_value_opt(&ctx, ValueType::Object, 0)?;
    let kp = match kp {
        Some(kp) => {
            let kp: &Keypair = ctx.env.unwrap(&kp)?;
            Some(kp.clone())
        }
        None => None,
    };

    let path = get_string_opt(&ctx, 1)?.map(|v| std::path::PathBuf::from(v));
    let addr = get_array_opt(&ctx, 2)?;
    let addr = match addr {
        Some(arr) => {
            let mut hs = std::collections::HashSet::new();

            for i in 0..arr.get_array_length()? {
                let s = arr.get_element::<JsString>(i)?.into_utf8()?.into_owned()?;
                let a =
                    SocketAddr::from_str(&s).map_err(|e| Error::from_reason(format!("{:?}", e)))?;
                hs.insert(a);
            }

            Some(hs)
        },
        None => None,
    };

    let this: JsObject = ctx.this_unchecked();
    let safe: &Arc<RwLock<Safe>> = ctx.env.unwrap(&this)?;
    let safe = Arc::clone(&safe);

    ctx.env.execute_tokio_future(
        async move {
            safe.write()
                .await
                .connect(kp, path.as_deref(), addr)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, _| env.get_undefined(),
    )
}

#[js_function(1)]
fn keys_create_preload_test_coins(ctx: CallContext) -> Result<JsObject> {
    let preload_amount = ctx.get::<JsString>(0)?.into_utf8()?.into_owned()?;

    let this: JsObject = ctx.this_unchecked();
    let safe: &Arc<RwLock<Safe>> = ctx.env.unwrap(&this)?;
    let safe = Arc::clone(&safe);

    ctx.env.execute_tokio_future(
        async move {
            safe.read()
                .await
                .keys_create_preload_test_coins(&preload_amount)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, (s, kp)| {
            let s = env.create_string(&s)?;
            let mut kp_js = get_constructor(&env, "Keypair")?.new(&[] as &[JsNull])?;
            env.wrap(&mut kp_js, kp)?;

            // Convert tuple into array of two elements.
            let mut arr = env.create_array_with_length(2)?;
            arr.set_element(0, s)?;
            arr.set_element(1, kp_js)?;
            Ok(arr)
        },
    )
}

#[js_function(1)]
fn keys_balance_from_sk(ctx: CallContext) -> Result<JsObject> {
    let sk = ctx.get::<JsObject>(0)?;
    let sk: &SecretKey = ctx.env.unwrap(&sk)?;

    // TODO: Fix dirty hack to get owned value (preferably by cloning).
    let sk: Vec<u8> = bincode::serialize(&sk).unwrap();
    let sk: SecretKey = bincode::deserialize(&sk[..]).unwrap();

    let this: JsObject = ctx.this_unchecked();
    let safe: &Arc<RwLock<Safe>> = ctx.env.unwrap(&this)?;
    let safe = Arc::clone(&safe);

    ctx.env.execute_tokio_future(
        async move {
            safe.read()
                .await
                .keys_balance_from_sk(sk)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, s| env.create_string(&s),
    )
}

/// Retrieve exports from instance data. Should contain constructors.
pub fn get_constructor(env: &Env, s: &str) -> Result<JsFunction> {
    let exports: &mut Ref<()> = env
        .get_instance_data()?
        .ok_or_else(|| Error::from_reason("no instance data".to_string()))?;

    let exports: JsObject = env.get_reference_value(exports)?;
    exports.get_named_property(s)
}

#[module_exports]
pub fn init(mut exports: JsObject, env: Env) -> Result<()> {
    let safe = env.define_class(
        "Safe",
        constructor,
        &[
            Property::new(&env, "connect")?.with_method(connect),
            Property::new(&env, "keys_create_preload_test_coins")?
                .with_method(keys_create_preload_test_coins),
            Property::new(&env, "keys_balance_from_sk")?.with_method(keys_balance_from_sk),
        ],
    )?;

    let keypair = keys::keypair::define_class(&env)?;
    let secret_key = keys::secret_key::define_class(&env)?;

    exports.set_named_property("Safe", safe)?;
    exports.set_named_property("Keypair", keypair)?;
    exports.set_named_property("SecretKey", secret_key)?;

    // Store a reference to the exports in the Agent instance. This way all functions
    // can call the constructors to instantiate new JS objects.
    let reference = env.create_reference(exports)?;
    env.set_instance_data(reference, 0, |fc| {
        fc.value.unref(fc.env).unwrap();
    })?;

    Ok(())
}
