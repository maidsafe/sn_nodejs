#[macro_use]
extern crate neon;

#[allow(unused_imports)]
#[macro_use]
extern crate neon_serde;

use neon::prelude::*;
use safe_cli::Safe;

// Temporary patch to have it work for electron v6
#[no_mangle]
pub extern "C" fn __cxa_pure_virtual() {
    loop {}
}

declare_types! {
    /// JS class wrapping Safe struct
    pub class JsSafe for Safe {
        // Initialise a new Safe instance
        init(mut cx) {
            let xorurl_base = match cx.argument_opt(0) {
                Some(arg) => arg.downcast::<JsString>().or_throw(&mut cx)?.value(),
                None => "".to_string()
            };
            println!("Creating Safe API instance with xorurl base: '{}'", xorurl_base);
            let safe = Safe::new(&xorurl_base);

            Ok(safe)
        }

        // Gets the XOR-URL base encoding set to be used for XOR-URLs generated
        method xorurl_base(mut cx) {
            let base = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.xorurl_base.clone()
            };
            println!("{}", &base);
            Ok(cx.string(&base).upcast())
        }

        // Generate an authorisation request string and send it to a SAFE Authenticator.
        // Ir returns the credentials necessary to connect to the network, encoded in a single string.
        // pub fn auth_app(&mut self, app_id: &str, app_name: &str, app_vendor: &str, port: Option<u16>) -> ResultReturn<String>
        method auth_app(mut cx) {
            let app_id = cx.argument::<JsString>(0)?.value();
            let app_name = cx.argument::<JsString>(1)?.value();
            let app_vendor = cx.argument::<JsString>(2)?.value();
            let port = match cx.argument_opt(3) {
                Some(arg) => Some(arg.downcast::<JsNumber>().or_throw(&mut cx)?.value() as u16),
                None => None
            };
            let auth_credentials = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                println!("Sending application authorisation request...");
                user.auth_app(&app_id, &app_name, &app_vendor, port).unwrap_or_else(|err| { panic!(format!("Failed to authorise application: {:?}", err)) } )
            };
            println!("Application successfully authorised!");
            Ok(cx.string(&auth_credentials).upcast())
        }

        // Connect to the SAFE Network using the provided app id and auth credentials
        // pub fn connect(&mut self, app_id: &str, auth_credentials: Option<&str>) -> ResultReturn<()>
        method connect(mut cx) {
            let app_id = cx.argument::<JsString>(0)?.value();
            #[allow(unused_assignments)]
            let mut str = String::default();
            let credentials = match cx.argument_opt(1) {
                Some(arg) => {
                    str = arg.downcast::<JsString>().or_throw(&mut cx)?.value();
                    Some(str.as_str())
                },
                None => None
            };

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let _ = user.connect(&app_id, credentials).unwrap_or_else(|err| { panic!(format!("Failed to connect: {:?}", err)) } );
                println!("Successfully connected to the Network!");
            }
            Ok(cx.undefined().upcast())
        }

        // Retrieve data from a safe:// URL
        // pub fn fetch(&self, url: &str) -> ResultReturn<SafeData>
        method fetch(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            println!("Fetching from: {}", url);
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.fetch(&url).unwrap_or_else(|err| { panic!(format!("Failed to fetch content: {:?}", err)) } )
            };

            let js_value = neon_serde::to_value(&mut cx, &data)?;
            Ok(js_value)
        }

        // Upload files/folder into a new FilesContainer returning its XOR-URL
        // pub fn files_container_create(&mut self, location: &str, dest: Option<String>, recursive: bool, dry_run: bool) -> ResultReturn<(XorUrl, ProcessedFiles, FilesMap)>
        method files_container_create(mut cx) {
            let location = cx.argument::<JsString>(0)?.value();
            let dest = match cx.argument_opt(1) {
                Some(arg) => {
                    Some(arg.downcast::<JsString>().or_throw(&mut cx)?.value())
                },
                None => None
            };

            let recursive = cx.argument::<JsBoolean>(2)?.value();
            let dry_run = cx.argument::<JsBoolean>(3)?.value();
            println!("Creating FilesContainer: {} - {:?} - {} - {}", location, dest, recursive, dry_run);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                user.files_container_create(&location, dest, recursive, dry_run).unwrap_or_else(|err| { panic!(format!("Failed to create FilesContainer: {:?}", err)) } )
            };

            let js_value = neon_serde::to_value(&mut cx, &data)?;
            Ok(js_value)
        }

        // Sync up files/folder with an existing FilesContainer
        // pub fn files_container_sync(&mut self, location: &str, url: &str, recursive: bool, delete: bool, update_nrs: bool, dry_run: bool) -> ResultReturn<(u64, ProcessedFiles, FilesMap)>
        method files_container_sync(mut cx) {
            let location = cx.argument::<JsString>(0)?.value();
            let url = cx.argument::<JsString>(1)?.value();
            let recursive = cx.argument::<JsBoolean>(2)?.value();
            let delete = cx.argument::<JsBoolean>(3)?.value();
            let update_nrs = cx.argument::<JsBoolean>(4)?.value();
            let dry_run = cx.argument::<JsBoolean>(5)?.value();
            println!("Sync-ing FilesContainer: {} - {} - {} - {} - {} - {}", location, url, recursive, delete, update_nrs, dry_run);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                user.files_container_sync(&location, &url, recursive, delete, update_nrs, dry_run).unwrap_or_else(|err| { panic!(format!("Failed to sync up FilesContainer: {:?}", err)) } )
            };

            let js_value = neon_serde::to_value(&mut cx, &data)?;
            Ok(js_value)
        }

        // Fetch an existing FilesContainer
        // pub fn files_container_get(&self, url: &str) -> ResultReturn<(u64, FilesMap)>
        method files_container_get(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            println!("Fetching FilesContainer from: {}", url);
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.files_container_get(&url).unwrap_or_else(|err| { panic!(format!("Failed to fetch FilesContainer: {:?}", err)) } )
            };

            let js_value = neon_serde::to_value(&mut cx, &data)?;
            Ok(js_value)
        }

        // Pub PublishedImmutableData
        // pub fn files_put_published_immutable(&mut self, data: &[u8]) -> ResultReturn<XorUrl>
        method files_put_published_immutable(mut cx) {
            let b: Handle<JsArrayBuffer> = cx.argument(0)?;
            let data = cx.borrow(&b, |data| data.as_slice::<u8>());
            println!("Putting PublishedImmutableData: {:?}", data);
            let url = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                user.files_put_published_immutable(&data).unwrap_or_else(|err| { panic!(format!("Failed to put PublishedImmutableData: {:?}", err)) } )
            };

            Ok(cx.string(&url).upcast())
        }

        // Get a PublishedImmutableData
        // pub fn files_get_published_immutable(&self, url: &str) -> ResultReturn<Vec<u8>>
        method files_get_published_immutable(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            println!("Fetching PublishedImmutableData from: {}", url);
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.files_get_published_immutable(&url).unwrap_or_else(|err| { panic!(format!("Failed to fetch PublishedImmutableData: {:?}", err)) } )
            };

            let js_value = neon_serde::to_value(&mut cx, &data)?;
            Ok(js_value)
        }

        // NRS Map Container create
        // pub fn nrs_map_container_create(&mut self, name: &str, link: &str, default: bool, hard_link: bool, dry_run: bool) -> ResultReturn<(XorUrl, ProcessedEntries, NrsMap)>
        method nrs_map_container_create(mut cx) {
            let name = cx.argument::<JsString>(0)?.value();
            let link = cx.argument::<JsString>(1)?.value();
            let default = cx.argument::<JsBoolean>(2)?.value();
            let hard_link = cx.argument::<JsBoolean>(3)?.value();
            let dry_run = cx.argument::<JsBoolean>(4)?.value();
            println!("Creating an NRS MAP Container: {} - {} - {} - {} - {}", name, link, default, hard_link, dry_run);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                user.nrs_map_container_create(&name, &link, default, hard_link, dry_run).unwrap_or_else(|err| { panic!(format!("Failed to create NRS Map Container: {:?}", err)) } )
            };

            let js_value = neon_serde::to_value(&mut cx, &data)?;
            Ok(js_value)
        }

        // Add/update a subname to an existing NRS Map Container
        // pub fn nrs_map_container_add(&mut self, name: &str, link: &str, default: bool, hard_link: bool, dry_run: bool) -> ResultReturn<(u64, XorUrl, ProcessedEntries, NrsMap)>
        method nrs_map_container_add(mut cx) {
            let name = cx.argument::<JsString>(0)?.value();
            let link = cx.argument::<JsString>(1)?.value();
            let default = cx.argument::<JsBoolean>(2)?.value();
            let hard_link = cx.argument::<JsBoolean>(3)?.value();
            let dry_run = cx.argument::<JsBoolean>(4)?.value();
            println!("Creating an NRS MAP Container: {} - {} - {} - {} - {}", name, link, default, hard_link, dry_run);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                user.nrs_map_container_add(&name, &link, default, hard_link, dry_run).unwrap_or_else(|err| { panic!(format!("Failed to create NRS Map Container: {:?}", err)) } )
            };

            let js_value = neon_serde::to_value(&mut cx, &data)?;
            Ok(js_value)
        }

        // Remove an NRS Map Container
        // pub fn nrs_map_container_remove(&mut self, name: &str, dry_run: bool) -> ResultReturn<(u64, XorUrl, ProcessedEntries, NrsMap)>
        method nrs_map_container_remove(mut cx) {
            let name = cx.argument::<JsString>(0)?.value();
            let dry_run = cx.argument::<JsBoolean>(1)?.value();
            println!("Removing an NRS Map Container: {} - {}", name, dry_run);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                user.nrs_map_container_remove(&name, dry_run).unwrap_or_else(|err| { panic!(format!("Failed to remove an NRS Map Container: {:?}", err)) } )
            };

            let js_value = neon_serde::to_value(&mut cx, &data)?;
            Ok(js_value)
        }

        // Fetch an NRS Map Container
        // pub fn nrs_map_container_get(&self, url: &str) -> ResultReturn<(u64, NrsMap)>
        method nrs_map_container_get(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            println!("Fetching NRS Map Container from: {}", url);
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.nrs_map_container_get(&url).unwrap_or_else(|err| { panic!(format!("Failed to fetch an NRS Map Container: {:?}", err)) } )
            };

            let js_value = neon_serde::to_value(&mut cx, &data)?;
            Ok(js_value)
        }

        // Parses a safe:// URL and returns all the info in a XorUrlEncoder instance.
        // pub fn parse_url(url: &str) -> ResultReturn<XorUrlEncoder>
        method parse_url(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            println!("Parsing a safe:// URL: {}", url);
            let _data = Safe::parse_url(&url).unwrap_or_else(|err| { panic!(format!("Failed to parse a safe:// URL: {:?}", err)) } );

            // TODO: create XorUrlEncoder class binding to return it from here
            //let js_value = neon_serde::to_value(&mut cx, &data)?;
            Ok(cx.boolean(true).upcast())
        }

        // Parses a safe:// URL and returns all the info in a XorUrlEncoder instance.
        // It also returns a flag indicating if it the URL has to be resolved as NRS-URL
        // pub fn parse_and_resolve_url(&self, url: &str) -> ResultReturn<(XorUrlEncoder, bool)>
        method parse_and_resolve_url(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            println!("Parsing and resolving a safe:// URL: {}", url);
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.parse_and_resolve_url(&url).unwrap_or_else(|err| { panic!(format!("Failed to parse/resolve a safe:// URL: {:?}", err)) } )
            };

            // TODO: create XorUrlEncoder class binding to return it from here
            //let js_value = neon_serde::to_value(&mut cx, &data)?;
            Ok(cx.boolean(data.1).upcast())
        }

    }
}

register_module!(mut m, {
    m.export_class::<JsSafe>("Safe")?;
    Ok(())
});
