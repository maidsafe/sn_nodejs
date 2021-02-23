use napi::*;
use napi_derive::js_function;

use sn_api::{Safe};
use std::{sync::Arc};
use tokio::sync::RwLock;
use tokio_compat_02::FutureExt;

#[js_function(5)]
pub fn container_create(ctx: CallContext) -> Result<JsObject> {
    let location: Option<String> = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;
    let dest: Option<String> = ctx.env.from_js_value(ctx.get::<JsString>(1)?)?;
    let recursive: bool = ctx.env.from_js_value(ctx.get::<JsBoolean>(2)?)?;
    let follow_links: bool = ctx.env.from_js_value(ctx.get::<JsBoolean>(3)?)?;
    let dry_run: bool = ctx.env.from_js_value(ctx.get::<JsBoolean>(4)?)?;

    let this: JsObject = ctx.this_unchecked();
    let safe: &Arc<RwLock<Safe>> = ctx.env.unwrap(&this)?;
    let safe = Arc::clone(&safe);

    ctx.env.execute_tokio_future(
        async move {
            let mut lock = safe.write().await;
            lock.files_container_create(location.as_deref(), dest.as_deref(), recursive, follow_links, dry_run)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, (xor, files, map)| {
            let mut arr = env.create_array_with_length(3)?;
            arr.set_element(0, env.create_string(&xor)?)?;
            arr.set_element(1, env.to_js_value(&files)?)?;
            arr.set_element(2, env.to_js_value(&map)?)?;

            Ok(arr)
        },
    )
}
