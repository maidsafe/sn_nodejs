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

#[js_function(2)]
pub fn unlock(ctx: CallContext) -> Result<JsObject> {
    let passphrase: String = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;
    let password: String = ctx.env.from_js_value(ctx.get::<JsString>(1)?)?;

    let cli = crate::util::unwrap_arc::<SafeAuthdClient>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let mut lock = cli.write().await;
            lock.unlock(&passphrase, &password)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, _| env.get_undefined(),
    )
}

#[js_function(0)]
pub fn lock(ctx: CallContext) -> Result<JsObject> {
    let cli = crate::util::unwrap_arc::<SafeAuthdClient>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let mut lock = cli.write().await;
            lock.lock()
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, _| env.get_undefined(),
    )
}

#[js_function(2)]
pub fn create(ctx: CallContext) -> Result<JsObject> {
    let passphrase: String = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;
    let password: String = ctx.env.from_js_value(ctx.get::<JsString>(1)?)?;

    let cli = crate::util::unwrap_arc::<SafeAuthdClient>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let lock = cli.read().await;
            lock.create(&passphrase, &password)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, _| env.get_undefined(),
    )
}

#[js_function(0)]
pub fn authed_apps(ctx: CallContext) -> Result<JsObject> {
    let cli = crate::util::unwrap_arc::<SafeAuthdClient>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let lock = cli.read().await;
            lock.authed_apps()
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, list| env.to_js_value(&list),
    )
}

#[js_function(1)]
pub fn revoke_app(ctx: CallContext) -> Result<JsObject> {
    let app_id: String = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;

    let cli = crate::util::unwrap_arc::<SafeAuthdClient>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let lock = cli.read().await;
            lock.revoke_app(&app_id)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, _| env.get_undefined(),
    )
}

#[js_function(0)]
pub fn auth_reqs(ctx: CallContext) -> Result<JsObject> {
    let cli = crate::util::unwrap_arc::<SafeAuthdClient>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let lock = cli.read().await;
            lock.auth_reqs()
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, reqs| env.to_js_value(&reqs),
    )
}

#[js_function(1)]
pub fn allow(ctx: CallContext) -> Result<JsObject> {
    let req_id: sn_api::SafeAuthReqId = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;

    let cli = crate::util::unwrap_arc::<SafeAuthdClient>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let lock = cli.read().await;
            lock.allow(req_id)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, _| env.get_undefined(),
    )
}

#[js_function(1)]
pub fn deny(ctx: CallContext) -> Result<JsObject> {
    let req_id: sn_api::SafeAuthReqId = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;

    let cli = crate::util::unwrap_arc::<SafeAuthdClient>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let lock = cli.read().await;
            lock.deny(req_id)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, _| env.get_undefined(),
    )
}

// TODO: subscribe
// TODO: subscribe_url
// TODO: unsubscribe

#[js_function(0)]
pub fn authd_endpoint_getter(ctx: CallContext) -> Result<JsUnknown> {
    let cli = crate::util::unwrap_arc::<SafeAuthdClient>(&ctx)?;

    // Possibly buggy because getter should never block.
    let endpoint = tokio::runtime::Runtime::new()?.block_on(async move {
        let lock = cli.read().await;
        lock.authd_endpoint.clone()
    });
    ctx.env.to_js_value(&endpoint)
}

pub fn define_class(env: &Env) -> Result<JsFunction> {
    env.define_class(
        "SafeAuthdClient",
        constructor,
        &[
            Property::new(&env, "status")?.with_method(status),
            Property::new(&env, "unlock")?.with_method(unlock),
            Property::new(&env, "lock")?.with_method(lock),
            Property::new(&env, "create")?.with_method(create),
            Property::new(&env, "authed_apps")?.with_method(authed_apps),
            Property::new(&env, "revoke_app")?.with_method(revoke_app),
            Property::new(&env, "auth_reqs")?.with_method(auth_reqs),
            Property::new(&env, "allow")?.with_method(allow),
            Property::new(&env, "deny")?.with_method(deny),
            Property::new(&env, "authd_endpoint")?.with_getter(authd_endpoint_getter),
        ],
    )
}
