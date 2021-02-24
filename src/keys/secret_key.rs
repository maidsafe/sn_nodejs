use napi::*;
use napi_derive::js_function;

use sn_api::SecretKey;

#[js_function(0)]
fn constructor(ctx: CallContext) -> Result<JsUndefined> {
    ctx.env.get_undefined()
}

#[js_function(0)]
fn to_string(ctx: CallContext) -> Result<JsString> {
    let sk: &SecretKey = ctx.env.unwrap(&ctx.this()?)?;

    ctx.env.create_string_from_std(format!("{:?}", sk))
}

pub fn define_class(env: &Env) -> Result<JsFunction> {
    env.define_class(
        "SecretKey",
        constructor,
        &[
            Property::new(&env, "toString")?.with_method(to_string)
        ],
    )
}
