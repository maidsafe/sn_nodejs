use std::{net::SocketAddr, str::FromStr};

use napi::{CallContext, Env, Error, JsObject, JsString, JsUndefined, Property, Result};
use napi_derive::{js_function, module_exports};

use sn_api::Safe;
use tokio_compat_02::FutureExt;

#[js_function(0)]
fn constructor(ctx: CallContext) -> Result<JsUndefined> {
    // TODO: Configurable arguments.
    let safe = Safe::new(None, std::time::Duration::from_secs(120));

    let mut this: JsObject = ctx.this_unchecked();
    ctx.env.wrap(&mut this, safe)?;

    ctx.env.get_undefined()
}

#[js_function(3)]
fn connect(ctx: CallContext) -> Result<JsUndefined> {
    // let keypair = ctx.get::<JsString>(0)?;
    let path = match ctx.get::<JsString>(1) {
        Ok(v) => {
            match v.into_utf8() {
                Ok(v) => Some(std::path::PathBuf::from(v.into_owned()?)),
                Err(_) => None,
            }
        },
        Err(_) => None,
    };

    let addr = match ctx.get::<JsObject>(2) {
        Ok(obj) if obj.is_array()? == false => None,
        Ok(obj) => {
            let mut hs = std::collections::HashSet::new();

            for i in 0..obj.get_array_length()? {
                let s = obj.get_element::<JsString>(i)?.into_utf8()?.into_owned()?;
                let a =
                    SocketAddr::from_str(&s).map_err(|e| Error::from_reason(format!("{:?}", e)))?;
                hs.insert(a);
            }

            Some(hs)
        }
        Err(_) => None,
    };

    let this: JsObject = ctx.this_unchecked();
    let safe: &mut Safe = ctx.env.unwrap(&this)?;

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(safe.connect(None, path.as_deref(), addr).compat())
        .unwrap();

    ctx.env.get_undefined()
}

#[js_function(1)]
fn keys_create_preload_test_coins(ctx: CallContext) -> Result<JsObject> {
    let preload_amount = ctx.get::<JsString>(0)?.into_utf8()?;

    let this: JsObject = ctx.this_unchecked();
    let safe: &mut Safe = ctx.env.unwrap(&this)?;

    let (s, kp) = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(
            safe.keys_create_preload_test_coins(preload_amount.as_str()?)
                .compat(),
        )
        .unwrap();
    let s = ctx.env.create_string(&s)?;
    let kp = ctx.env.create_string(format!("{:?}", &kp).as_str())?;

    let mut arr = ctx.env.create_array_with_length(2)?;
    arr.set_element(0, s)?;
    arr.set_element(1, kp)?;
    Ok(arr)
}

#[js_function(1)]
fn keys_create_preload_test_coins_async(ctx: CallContext) -> Result<JsObject> {
    let preload_amount = ctx.get::<JsString>(0)?.into_utf8()?.into_owned()?;

    let this: JsObject = ctx.this_unchecked();
    let safe: &mut Safe = ctx.env.unwrap(&this)?;
    let mut safe = safe.clone();

    ctx.env.execute_tokio_future(
        async move {
            safe.keys_create_preload_test_coins(&preload_amount)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, (s, kp)| {
            let s = env.create_string(&s)?;
            let kp = env.create_string(format!("{:?}", &kp).as_str())?;

            let mut arr = env.create_array_with_length(2)?;
            arr.set_element(0, s)?;
            arr.set_element(1, kp)?;
            Ok(arr)
        },
    )
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
            Property::new(&env, "keys_create_preload_test_coins_async")?
                .with_method(keys_create_preload_test_coins_async),
        ],
    )?;

    exports.set_named_property("Safe", safe)?;

    Ok(())
}
