use napi::*;
use napi_derive::js_function;

use sn_api::{fetch::{Range}};
use crate::util::Tag;

#[js_function(2)]
pub fn fetch(ctx: CallContext) -> Result<JsObject> {
    let url: String = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;
    let range: Range = ctx.env.from_js_value(ctx.get::<JsObject>(1)?)?;

    let safe = crate::util::clone_wrapped::<super::Type>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let lock = safe.read().await;
            lock.fetch(&url, range)
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, data| env.to_js_value(&Tag::V(data)),
    )
}