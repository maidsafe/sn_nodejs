use napi::*;
use napi_derive::js_function;

use sn_api::{Keypair};
use std::{collections::HashSet, net::SocketAddr, path::PathBuf};
use tokio_compat_02::FutureExt;

use crate::safe;
use crate::util;

#[js_function(3)]
pub fn connect(ctx: CallContext) -> Result<JsObject> {
    // let kp: Option<Keypair> = ctx.env.from_js_value(ctx.get::<JsObject>(0)?)?;
    let kp: Option<JsObject> = util::get_value_opt(&ctx, ValueType::Object, 0)?;
    let kp = match kp {
        Some(kp) => {
            let kp: &Keypair = ctx.env.unwrap(&kp)?;
            Some(kp.clone())
        }
        None => None,
    };
    // let path = util::get_string_opt(&ctx, 1)?.map(|v| std::path::PathBuf::from(v));
    let path: Option<PathBuf> = ctx.env.from_js_value(ctx.get::<JsString>(1)?)?;
    let addr: Option<HashSet<SocketAddr>> = ctx.env.from_js_value(ctx.get::<JsObject>(2)?)?;

    let safe = safe::unwrap_arc(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let mut lock = safe.write().await;
            lock.connect(kp, path.as_deref(), addr)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, _| env.get_undefined(),
    )
}
