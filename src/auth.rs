use napi::*;
use napi_derive::js_function;

use sn_api::{Keypair, Safe};
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use tokio_compat_02::FutureExt;

use crate::util;

#[js_function(3)]
pub fn connect(ctx: CallContext) -> Result<JsObject> {
    let kp: Option<JsObject> = util::get_value_opt(&ctx, ValueType::Object, 0)?;
    let kp = match kp {
        Some(kp) => {
            let kp: &Keypair = ctx.env.unwrap(&kp)?;
            Some(kp.clone())
        }
        None => None,
    };

    let path = util::get_string_opt(&ctx, 1)?.map(|v| std::path::PathBuf::from(v));
    let addr = util::get_array_opt(&ctx, 2)?;
    let addr = match addr {
        Some(arr) => {
            let mut hs = std::collections::HashSet::new();

            for i in 0..arr.get_array_length()? {
                let s = arr.get_element::<JsString>(i)?.into_utf8()?.into_owned()?;
                let a =
                    SocketAddr::from_str(&s).map_err(|e| Error::from_reason(format!("{:?}", e)))?;
                hs.insert(a);
            }

            Some(hs)
        }
        None => None,
    };

    let this: JsObject = ctx.this_unchecked();
    let safe: &Arc<RwLock<Safe>> = ctx.env.unwrap(&this)?;
    let safe = Arc::clone(&safe);

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
