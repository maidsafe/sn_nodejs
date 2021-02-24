use napi::*;
use napi_derive::js_function;

use sn_api::{fetch::XorUrlBase, Safe};
use std::{str::FromStr, sync::Arc, time::Duration};
use tokio::sync::RwLock;

use crate::{auth, files, keys};

#[js_function(2)]
fn constructor(ctx: CallContext) -> Result<JsUndefined> {
    let xor_url_base: Option<String> = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;
    let xor_url_base = match xor_url_base {
        Some(x) => {
            Some(XorUrlBase::from_str(&x).map_err(|e| Error::from_reason(format!("{:?}", e)))?)
        }
        None => None,
    };

    let time: Duration = ctx.env.from_js_value(ctx.get::<JsNumber>(1)?)?;

    let safe = Safe::new(xor_url_base, time);
    let safe = Arc::new(RwLock::new(safe));

    let mut this: JsObject = ctx.this_unchecked();
    ctx.env.wrap(&mut this, safe)?;

    ctx.env.get_undefined()
}

/// Helper function to clone Arc to wrapped Safe RwLock.
pub fn unwrap_arc(ctx: &CallContext) -> Result<Arc<RwLock<Safe>>> {
    let safe: &Arc<RwLock<Safe>> = ctx.env.unwrap(&ctx.this()?)?;
    Ok(Arc::clone(&safe))
}

pub fn define_class(env: &Env) -> Result<JsFunction> {
    env.define_class(
        "Safe",
        constructor,
        &[
            Property::new(&env, "connect")?.with_method(auth::connect),
            Property::new(&env, "keys_create_preload_test_coins")?
                .with_method(keys::create_preload_test_coins),
            Property::new(&env, "keys_balance_from_sk")?.with_method(keys::balance_from_sk),
            Property::new(&env, "files_container_create")?.with_method(files::container_create),
        ],
    )
}
