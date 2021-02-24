use napi::*;
use napi_derive::js_function;

use sn_api::{SecretKey};
use tokio_compat_02::FutureExt;

use crate::{safe, util};

pub mod keypair;
pub mod secret_key;

#[js_function(1)]
pub fn create_preload_test_coins(ctx: CallContext) -> Result<JsObject> {
    let preload_amount: String = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;

    let safe = safe::unwrap_arc(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let lock = safe.read().await;
            lock.keys_create_preload_test_coins(&preload_amount)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, (s, kp)| {
            let s = env.create_string(&s)?;
            let mut kp_js = util::get_constructor(&env, "Keypair")?.new(&[] as &[JsNull])?;
            env.wrap(&mut kp_js, kp)?;

            // Convert tuple into array of two elements.
            let mut arr = env.create_array_with_length(2)?;
            arr.set_element(0, s)?;
            arr.set_element(1, kp_js)?;
            Ok(arr)
        },
    )
}

#[js_function(1)]
pub fn balance_from_sk(ctx: CallContext) -> Result<JsObject> {
    let sk = ctx.get::<JsObject>(0)?;
    let sk: &SecretKey = ctx.env.unwrap(&sk)?;

    // TODO: Fix dirty hack to get owned value (preferably by cloning) (upstream).
    let sk: Vec<u8> = bincode::serialize(&sk).unwrap();
    let sk: SecretKey = bincode::deserialize(&sk[..]).unwrap();

    let safe = safe::unwrap_arc(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let lock = safe.read().await;
            lock.keys_balance_from_sk(sk)
                .compat()
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, s| env.create_string(&s),
    )
}
