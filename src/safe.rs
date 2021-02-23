use napi::*;
use napi_derive::js_function;

use sn_api::Safe;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{auth, keys};

#[js_function(0)]
fn constructor(ctx: CallContext) -> Result<JsUndefined> {
    // TODO: Configurable arguments.
    let safe = Safe::new(None, std::time::Duration::from_secs(120));
    let safe = Arc::new(RwLock::new(safe));

    let mut this: JsObject = ctx.this_unchecked();
    ctx.env.wrap(&mut this, safe)?;

    ctx.env.get_undefined()
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
        ],
    )
}
