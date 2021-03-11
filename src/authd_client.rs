use napi::*;
use napi_derive::js_function;

use sn_api::SafeAuthdClient;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_compat_02::FutureExt;

#[js_function(1)]
fn constructor(ctx: CallContext) -> Result<JsUndefined> {
    let endpoint: Option<String> = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;

    let cli = SafeAuthdClient::new(endpoint);
    let cli = Arc::new(RwLock::new(cli));

    let mut this: JsObject = ctx.this_unchecked();
    ctx.env.wrap(&mut this, cli)?;

    ctx.env.get_undefined()
}

// #[js_function(0)]
// fn start(ctx: CallContext) -> Result<JsUndefined> {
//     let authd_path: Option<String> = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;

//     let cli: &SafeAuthdClient = ctx.env.unwrap(&ctx.this()?)?;
//     cli.start(authd_path.as_deref())
//         .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

//     ctx.env.get_undefined()
// }

#[js_function(0)]
pub fn status(ctx: CallContext) -> Result<JsObject> {
    let cli = crate::util::unwrap_arc::<SafeAuthdClient>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let mut lock = cli.write().await;
            lock.status()
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, status| Ok(env.to_js_value(&status)?),
    )
}

pub fn define_class(env: &Env) -> Result<JsFunction> {
    env.define_class(
        "SafeAuthdClient",
        constructor,
        &[
            // Property::new(&env, "start")?.with_method(start),
            Property::new(&env, "status")?.with_method(status),
        ],
    )
}
