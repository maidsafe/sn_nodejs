use napi::*;
use napi_derive::js_function;

use rand::rngs::OsRng;
use sn_api::Keypair;

use crate::util;

#[js_function(0)]
fn constructor(ctx: CallContext) -> Result<JsUndefined> {
    // No wrapping here, that will be done by static methods or other objects.
    ctx.env.get_undefined()
}

#[js_function(0)]
fn new_ed25519(ctx: CallContext) -> Result<JsObject> {
    let mut kp_js = util::get_constructor(ctx.env, "Keypair")?.new(&[] as &[JsNull])?;

    let mut rng = OsRng;
    let kp = Keypair::new_ed25519(&mut rng);
    ctx.env.wrap(&mut kp_js, kp)?;

    Ok(kp_js)
}

#[js_function(0)]
fn secret_key(ctx: CallContext) -> Result<JsObject> {
    let kp: &Keypair = ctx.env.unwrap(&ctx.this()?)?;

    let sk = kp.secret_key().map_err(|e| Error::from_reason(format!("{:?}", e)))?;

    let mut sk_js = util::get_constructor(ctx.env, "SecretKey")?.new(&[] as &[JsNull])?;
    ctx.env.wrap(&mut sk_js, sk)?;

    Ok(sk_js)
}

pub fn define_class(env: &Env) -> Result<JsFunction> {
    env.define_class(
        "Keypair",
        constructor,
        &[
            Property::new(&env, "new_ed25519")?
                .with_method(new_ed25519)
                .with_property_attributes(PropertyAttributes::Static),
            Property::new(&env, "secret_key")?.with_method(secret_key),
        ],
    )
}
