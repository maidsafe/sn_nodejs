use napi::*;
use napi_derive::js_function;

use tokio_compat_02::FutureExt;

#[js_function(5)]
pub fn map_container_create(ctx: CallContext) -> Result<JsObject> {
    let name: String = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;
    let link: String = ctx.env.from_js_value(ctx.get::<JsString>(1)?)?;
    let default: bool = ctx.env.from_js_value(ctx.get::<JsBoolean>(2)?)?;
    let hard_link: bool = ctx.env.from_js_value(ctx.get::<JsBoolean>(3)?)?;
    let dry_run: bool = ctx.env.from_js_value(ctx.get::<JsBoolean>(4)?)?;

    let safe = crate::util::clone_wrapped::<super::Type>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let mut lock = safe.write().await;
            lock.nrs_map_container_create(&name, &link, default, hard_link, dry_run)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, (xor, entries, map)| {
            let mut arr = env.create_array_with_length(3)?;
            arr.set_element(0, env.create_string(&xor)?)?;
            arr.set_element(1, env.to_js_value(&entries)?)?;
            arr.set_element(2, env.to_js_value(&map)?)?;

            Ok(arr)
        },
    )
}

#[js_function(5)]
pub fn map_container_add(ctx: CallContext) -> Result<JsObject> {
    let name: String = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;
    let link: String = ctx.env.from_js_value(ctx.get::<JsString>(1)?)?;
    let default: bool = ctx.env.from_js_value(ctx.get::<JsBoolean>(2)?)?;
    let hard_link: bool = ctx.env.from_js_value(ctx.get::<JsBoolean>(3)?)?;
    let dry_run: bool = ctx.env.from_js_value(ctx.get::<JsBoolean>(4)?)?;

    let safe = crate::util::clone_wrapped::<super::Type>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let lock = safe.read().await;
            lock.nrs_map_container_add(&name, &link, default, hard_link, dry_run)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, (version, xor, entries, map)| {
            let mut arr = env.create_array_with_length(4)?;
            arr.set_element(0, env.to_js_value(&version)?)?;
            arr.set_element(1, env.to_js_value(&xor)?)?;
            arr.set_element(2, env.to_js_value(&entries)?)?;
            arr.set_element(3, env.to_js_value(&map)?)?;

            Ok(arr)
        },
    )
}

#[js_function(1)]
pub fn map_container_get(ctx: CallContext) -> Result<JsObject> {
    let url: String = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;

    let safe = crate::util::clone_wrapped::<super::Type>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let lock = safe.read().await;
            lock.nrs_map_container_get(&url)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, (version, map)| {
            let mut arr = env.create_array_with_length(2)?;
            arr.set_element(0, env.to_js_value(&version)?)?;
            arr.set_element(1, env.to_js_value(&map)?)?;

            Ok(arr)
        },
    )
}
