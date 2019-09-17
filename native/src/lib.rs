#[macro_use]
extern crate neon;

#[allow(unused_imports)]
#[macro_use]
extern crate neon_serde;

extern crate log;

use env_logger;
use log::debug;
use neon::prelude::*;
use safe_cli::{Safe, SafeContentType, SafeDataType, XorName, XorUrlEncoder};

// Temporary patch to have it work for electron v6
#[no_mangle]
pub extern "C" fn __cxa_pure_virtual() {
    loop {}
}

const SAFE_CONTENT_TYPE: &[SafeContentType] = &[
    SafeContentType::Raw,             // 0x00
    SafeContentType::Wallet,          // 0x01
    SafeContentType::FilesContainer,  // 0x02
    SafeContentType::NrsMapContainer, // 0x03
];

const SAFE_DATA_TYPE: &[SafeDataType] = &[
    SafeDataType::SafeKey,                        // 0x00
    SafeDataType::PublishedImmutableData,         // 0x01
    SafeDataType::UnpublishedImmutableData,       // 0x02
    SafeDataType::SeqMutableData,                 // 0x03
    SafeDataType::UnseqMutableData,               // 0x04
    SafeDataType::PublishedSeqAppendOnlyData,     // 0x05
    SafeDataType::PublishedUnseqAppendOnlyData,   // 0x06
    SafeDataType::UnpublishedSeqAppendOnlyData,   // 0x07
    SafeDataType::UnpublishedUnseqAppendOnlyData, // 0x08
];

declare_types! {
    /// JS class wrapping XorUrlEncoder struct
    pub class JsXorUrlEncoder for XorUrlEncoder {
        // Initialise a new XorUrlEncoder instance
        // pub fn new(xorname: XorName, type_tag: u64, data_type: SafeDataType, content_type: SafeContentType, path: Option<&str>, sub_names: Option<Vec<String>>, content_version: Option<u64>) -> Self
        init(mut cx) {
            let v0: Handle<JsValue> = cx.argument(0)?;
            let buffer: Handle<JsBuffer>;
            let array_buffer: Handle<JsArrayBuffer>;
            let xorname_slice = if v0.is_a::<JsBuffer>() {
                buffer = cx.argument(0)?;
                cx.borrow(&buffer, |data| data.as_slice::<u8>())
            } else if v0.is_a::<JsArrayBuffer>() {
                array_buffer = cx.argument(0)?;
                cx.borrow(&array_buffer, |data| data.as_slice::<u8>())
            } else {
                panic!("A Buffer or ArrayBuffer is expected as first argument");
            };
            let mut xorname = XorName::default();
            xorname.0.copy_from_slice(&xorname_slice);

            let type_tag = cx.argument::<JsNumber>(1)?.value() as u64;

            let data_type_index = cx.argument::<JsNumber>(2)?.value();
            let data_type = SAFE_DATA_TYPE[data_type_index as usize].clone();

            let v3: Handle<JsValue> = cx.argument(3)?;
            let content_type = if v3.is_a::<JsNumber>() {
                let content_type_index = cx.argument::<JsNumber>(3)?.value();
                SAFE_CONTENT_TYPE[content_type_index as usize].clone()
            } else if v3.is_a::<JsString>() {
                let media_type_str = cx.argument::<JsString>(3)?.value();
                SafeContentType::MediaType(media_type_str.to_string())
            } else {
                panic!("MediaType argument contains an invalid value");
            };

            #[allow(unused_assignments)]
            let mut str = String::default();
            let path = match cx.argument_opt(4) {
                Some(arg) => {
                    str = arg.downcast::<JsString>().or_throw(&mut cx)?.value();
                    Some(str.as_str())
                },
                None => None
            };

            let js_arr_handle: Handle<JsArray> = cx.argument(5)?;
            // Convert a JsArray to a Rust Vec
            let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;
            // Interate over the Rust Vec and return a new Vec of Vec<JsNumber>
            let sub_names: Option<Vec<String>> = if vec.is_empty() {
                None
            } else {
                let sub_names_vec = vec
                .iter()
                .map(|js_value| {
                    js_value
                        .downcast::<JsString>()
                        // If downcast fails, default to using 0
                        .unwrap_or(cx.string(""))
                        // Get the value of the unwrapped value
                        .value()
                })
                .collect();
                Some(sub_names_vec)
            };

            let content_version = match cx.argument_opt(6) {
                Some(arg) => Some(arg.downcast::<JsNumber>().or_throw(&mut cx)?.value() as u64),
                None => None
            };

            debug!("Creating XorUrlEncoder instance");
            let xorurl_encoder = XorUrlEncoder::new(xorname, type_tag, data_type, content_type, path, sub_names, content_version).unwrap_or_else(|err| { panic!(format!("Failed to instantiate XorUrlEncoder: {:?}", err)) } );
            Ok(xorurl_encoder)
        }

        // pub fn encoding_version(&self) -> u64
        method encoding_version(mut cx) {
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.encoding_version()
            };
            Ok(cx.number(data as f64).upcast())
        }

        // pub fn data_type(&self) -> SafeDataType
        method data_type(mut cx) {
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.data_type()
            };
            let index = SAFE_DATA_TYPE.iter().position(|r| r == &data).unwrap();
            Ok(cx.number(index as f64).upcast())
        }

        // pub fn content_type(&self) -> SafeContentType
        method content_type(mut cx) {
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.content_type()
            };
            if let SafeContentType::MediaType(media_type_str) = data {
                Ok(cx.string(media_type_str).upcast())
            } else {
                let index = SAFE_CONTENT_TYPE.iter().position(|r| r == &data).unwrap();
                Ok(cx.number(index as f64).upcast())
            }
        }

        // pub fn xorname(&self) -> XorName
        method xorname(mut cx) {
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.xorname()
            };

            let js_array = JsArray::new(&mut cx, data.0.len() as u32);
            for (i, obj) in data.0.iter().enumerate() {
                let js_number = cx.number(*obj as i8);
                js_array.set(&mut cx, i as u32, js_number).unwrap();
            }

            Ok(js_array.upcast())
        }

        // pub fn type_tag(&self) -> u64
        method type_tag(mut cx) {
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.type_tag()
            };
            Ok(cx.number(data as f64).upcast())
        }

        // pub fn path(&self) -> &str
        method path(mut cx) {
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.path().to_string()
            };
            Ok(cx.string(&data).upcast())
        }

        // pub fn set_path(&mut self, path: &str)
        method set_path(mut cx) {
            let path = cx.argument::<JsString>(0)?.value();
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                user.set_path(&path);
            }

            Ok(cx.undefined().upcast())
        }

        // pub fn sub_names(&self) -> Vec<String>
        method sub_names(mut cx) {
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.sub_names()
            };

            let js_array = JsArray::new(&mut cx, data.len() as u32);
            for (i, obj) in data.iter().enumerate() {
                let js_string = cx.string(obj);
                js_array.set(&mut cx, i as u32, js_string).unwrap();
            }

            Ok(js_array.upcast())
        }

        // pub fn content_version(&self) -> Option<u64>
        method content_version(mut cx) {
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.content_version().map_or_else(|| 0 as f64, |v| v as f64)
            };
            Ok(cx.number(data).upcast())
        }

        // pub fn set_content_version(&mut self, version: Option<u64>)
        method set_content_version(mut cx) {
            let version = match cx.argument_opt(0) {
                Some(arg) => Some(arg.downcast::<JsNumber>().or_throw(&mut cx)?.value() as u64),
                None => None
            };
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                user.set_content_version(version);
            }

            Ok(cx.undefined().upcast())
        }

        // pub fn to_string(&self) -> ResultReturn<String>
        method to_string(mut cx) {
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.to_string()
            };
            Ok(cx.string(&data).upcast())
        }

        // pub fn to_base(&self, base: &str) -> ResultReturn<String>
        method to_base(mut cx) {
            let base = cx.argument::<JsString>(0)?.value();
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.to_base(&base).unwrap_or_else(|err| { panic!(format!("Failed to encode with base {}: {:?}", base, err)) } )
            };
            Ok(cx.string(&data).upcast())
        }
    }
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
            debug!("Creating Safe API instance with xorurl base: '{}'", xorurl_base);
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
            debug!("{}", &base);
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
                debug!("Sending application authorisation request...");
                user.auth_app(&app_id, &app_name, &app_vendor, port).unwrap_or_else(|err| { panic!(format!("Failed to authorise application: {:?}", err)) } )
            };
            debug!("Application successfully authorised!");
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
                debug!("Successfully connected to the Network!");
            }
            Ok(cx.undefined().upcast())
        }

        // Retrieve data from a safe:// URL
        // pub fn fetch(&self, url: &str) -> ResultReturn<SafeData>
        method fetch(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            debug!("Fetching from: {}", url);
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
            #[allow(unused_assignments)]
            let mut str = String::default();
            let dest = match cx.argument_opt(1) {
                Some(arg) => {
                    str = arg.downcast::<JsString>().or_throw(&mut cx)?.value();
                    Some(str.as_str())
                },
                None => None
            };

            let recursive = cx.argument::<JsBoolean>(2)?.value();
            let dry_run = cx.argument::<JsBoolean>(3)?.value();
            debug!("Creating FilesContainer: {} - {:?} - {} - {}", location, dest, recursive, dry_run);

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
            debug!("Sync-ing FilesContainer: {} - {} - {} - {} - {} - {}", location, url, recursive, delete, update_nrs, dry_run);

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
            debug!("Fetching FilesContainer from: {}", url);
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.files_container_get(&url).unwrap_or_else(|err| { panic!(format!("Failed to fetch FilesContainer: {:?}", err)) } )
            };

            let js_value = neon_serde::to_value(&mut cx, &data)?;
            Ok(js_value)
        }

        // Add file to an existing FilesContainer
        // pub fn files_container_add(&mut self, source_file: &str, url: &str, force: bool, update_nrs: bool, dry_run: bool) -> ResultReturn<(u64, ProcessedFiles, FilesMap)>
        method files_container_add(mut cx) {
            let source_file = cx.argument::<JsString>(0)?.value();
            let url = cx.argument::<JsString>(1)?.value();
            let force = cx.argument::<JsBoolean>(2)?.value();
            let update_nrs = cx.argument::<JsBoolean>(3)?.value();
            let dry_run = cx.argument::<JsBoolean>(4)?.value();
            debug!("Adding to FilesContainer: {} - {} - {} - {} - {}", source_file, url, force, update_nrs, dry_run);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                user.files_container_add(&source_file, &url, force, update_nrs, dry_run).unwrap_or_else(|err| { panic!(format!("Failed to add file to FilesContainer: {:?}", err)) } )
            };

            let js_value = neon_serde::to_value(&mut cx, &data)?;
            Ok(js_value)
        }

        // Add file from raw bytes to an existing FilesContainer
        // pub fn files_container_add_from_raw(&mut self, data: &[u8], url: &str, force: bool, update_nrs: bool, dry_run: bool) -> ResultReturn<(u64, ProcessedFiles, FilesMap)>
        method files_container_add_from_raw(mut cx) {
            let v: Handle<JsValue> = cx.argument(0)?;
            let buffer: Handle<JsBuffer>;
            let array_buffer: Handle<JsArrayBuffer>;
            let data = if v.is_a::<JsBuffer>() {
                buffer = cx.argument(0)?;
                cx.borrow(&buffer, |data| data.as_slice::<u8>())
            } else if v.is_a::<JsArrayBuffer>() {
                array_buffer = cx.argument(0)?;
                cx.borrow(&array_buffer, |data| data.as_slice::<u8>())
            } else {
                panic!("A Buffer or ArrayBuffer is expected as first argument");
            };

            let url = cx.argument::<JsString>(1)?.value();
            let force = cx.argument::<JsBoolean>(2)?.value();
            let update_nrs = cx.argument::<JsBoolean>(3)?.value();
            let dry_run = cx.argument::<JsBoolean>(4)?.value();
            debug!("Adding from raw bytes to FilesContainer: {:?} - {} - {} - {} - {}", data, url, force, update_nrs, dry_run);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                user.files_container_add_from_raw(&data, &url, force, update_nrs, dry_run).unwrap_or_else(|err| { panic!(format!("Failed to add file form raw bytes to FilesContainer: {:?}", err)) } )
            };

            let js_value = neon_serde::to_value(&mut cx, &data)?;
            Ok(js_value)
        }

        // Pub PublishedImmutableData
        // pub fn files_put_published_immutable(&mut self, data: &[u8]) -> ResultReturn<XorUrl>
        method files_put_published_immutable(mut cx) {
            let v: Handle<JsValue> = cx.argument(0)?;
            let buffer: Handle<JsBuffer>;
            let array_buffer: Handle<JsArrayBuffer>;
            let data = if v.is_a::<JsBuffer>() {
                buffer = cx.argument(0)?;
                cx.borrow(&buffer, |data| data.as_slice::<u8>())
            } else if v.is_a::<JsArrayBuffer>() {
                array_buffer = cx.argument(0)?;
                cx.borrow(&array_buffer, |data| data.as_slice::<u8>())
            } else {
                panic!("A Buffer or ArrayBuffer is expected as first argument");
            };

            #[allow(unused_assignments)]
            let mut str = String::default();
            let media_type = match cx.argument_opt(1) {
                Some(arg) => {
                    str = arg.downcast::<JsString>().or_throw(&mut cx)?.value();
                    Some(str.as_str())
                },
                None => None
            };
            debug!("Putting PublishedImmutableData: {:?}", data);
            let url = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                user.files_put_published_immutable(&data, media_type).unwrap_or_else(|err| { panic!(format!("Failed to put PublishedImmutableData: {:?}", err)) } )
            };

            Ok(cx.string(&url).upcast())
        }

        // Get a PublishedImmutableData
        // pub fn files_get_published_immutable(&self, url: &str) -> ResultReturn<Vec<u8>>
        method files_get_published_immutable(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            debug!("Fetching PublishedImmutableData from: {}", url);
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
            debug!("Creating an NRS MAP Container: {} - {} - {} - {} - {}", name, link, default, hard_link, dry_run);

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
            debug!("Creating an NRS MAP Container: {} - {} - {} - {} - {}", name, link, default, hard_link, dry_run);

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
            debug!("Removing an NRS Map Container: {} - {}", name, dry_run);

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
            debug!("Fetching NRS Map Container from: {}", url);
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
            debug!("Parsing a safe:// URL: {}", url);
            let _xorurl_encoder = Safe::parse_url(&url).unwrap_or_else(|err| { panic!(format!("Failed to parse a safe:// URL: {:?}", err)) } );
            //let xorurl_encoder_js = JsXorUrlEncoder::new();

            //let xorurl_encoder_js = JsXorUrlEncoder::new(&mut cx, xorurl_encoder.xorname())?;
            //Ok(xorurl_encoder_js.upcast())
            Ok(cx.boolean(true).upcast())
        }

        // Parses a safe:// URL and returns all the info in a XorUrlEncoder instance.
        // It also returns a flag indicating if it the URL has to be resolved as NRS-URL
        // pub fn parse_and_resolve_url(&self, url: &str) -> ResultReturn<(XorUrlEncoder, bool)>
        method parse_and_resolve_url(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            debug!("Parsing and resolving a safe:// URL: {}", url);
            let _data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.parse_and_resolve_url(&url).unwrap_or_else(|err| { panic!(format!("Failed to parse/resolve a safe:// URL: {:?}", err)) } )
            };

            // TODO: create XorUrlEncoder class binding to return it from here
            //let js_value = neon_serde::to_value(&mut cx, &data)?;
            Ok(cx.boolean(true).upcast())
        }

    }
}

register_module!(mut m, {
    env_logger::init();
    m.export_class::<JsSafe>("Safe")?;
    m.export_class::<JsXorUrlEncoder>("XorUrlEncoder")?;

    let safe_data_type = JsObject::new(&mut m);
    for (i, data_type) in SAFE_DATA_TYPE.iter().enumerate() {
        let js_number = m.number(i as f64);
        safe_data_type
            .set(&mut m, data_type.to_string().as_str(), js_number)
            .unwrap();
    }
    m.export_value("SafeDataType", safe_data_type)?;

    let safe_content_type = JsObject::new(&mut m);
    for (i, content_type) in SAFE_CONTENT_TYPE.iter().enumerate() {
        let js_number = m.number(i as f64);
        safe_content_type
            .set(&mut m, content_type.to_string().as_str(), js_number)
            .unwrap();
    }
    m.export_value("SafeContentType", safe_content_type)?;

    Ok(())
});
