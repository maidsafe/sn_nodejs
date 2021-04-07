use napi::*;
use napi_derive::js_function;

use sn_api::SecretKey;

#[js_function(1)]
pub fn create_preload_test_coins(ctx: CallContext) -> Result<JsObject> {
    let preload_amount: String = ctx.env.from_js_value(ctx.get::<JsString>(0)?)?;

    let safe = crate::util::clone_wrapped::<super::Type>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let lock = safe.read().await;
            lock.keys_create_preload_test_coins(&preload_amount)
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, (s, kp)| {
            let s = env.create_string(&s)?;
            let mut kp_js = crate::util::get_constructor(&env, "Keypair")?.new(&[] as &[JsNull])?;
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
    let sk: &std::sync::Arc<SecretKey> = ctx.env.unwrap(&sk)?;
    let sk = sk.clone();

    let safe = crate::util::clone_wrapped::<super::Type>(&ctx)?;

    ctx.env.execute_tokio_future(
        async move {
            let lock = safe.read().await;
            lock.keys_balance_from_sk(&sk)
                .await
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
        },
        |&mut env, s| env.create_string(&s),
    )
}
