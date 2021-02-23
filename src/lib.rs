use napi::*;
use napi_derive::module_exports;

mod auth;
mod files;
mod keys;
mod safe;
mod util;

#[module_exports]
pub fn init(mut exports: JsObject, env: Env) -> Result<()> {
    let safe = safe::define_class(&env)?;
    let keypair = keys::keypair::define_class(&env)?;
    let secret_key = keys::secret_key::define_class(&env)?;

    exports.set_named_property("Safe", safe)?;
    exports.set_named_property("Keypair", keypair)?;
    exports.set_named_property("SecretKey", secret_key)?;

    // Store a reference to the exports in the Agent instance. This way all functions
    // can call the constructors to instantiate new JS objects.
    let reference = env.create_reference(exports)?;
    env.set_instance_data(reference, 0, |fc| {
        fc.value.unref(fc.env).unwrap();
    })?;

    Ok(())
}
