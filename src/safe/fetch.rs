use napi::*;
use napi_derive::js_function;

use sn_api::{Safe, fetch::{Range}};
use tokio_compat_02::FutureExt;
use crate::util::Tag;

#[js_function(2)]
pub fn fetch(ctx: CallContext) -> Result<JsObject> {
    let url: String = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;
    let range: Range = ctx.env.from_js_value(ctx.get::<JsObject>(1)?)?;

    let safe = crate::util::unwrap_arc::<Safe>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let mut lock = safe.write().await;
            lock.fetch(&url, range)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, data| env.to_js_value(&Tag::V(data)),
    )
}