use env_logger;
use log::debug;
use neon::prelude::*;
use neon::result::Throw;
use sn_api::{
    fetch::{SafeContentType, SafeDataType},
    xorurl::{XorUrlBase, XorUrlEncoder},
    AuthReq, Safe, SafeAuthdClient, XorName,
};
use std::{str::FromStr, time::Duration};
use tokio::runtime::Runtime;

const SAFE_CONTENT_TYPE: &[SafeContentType] = &[
    SafeContentType::Raw,             // 0x00
    SafeContentType::Wallet,          // 0x01
    SafeContentType::FilesContainer,  // 0x02
    SafeContentType::NrsMapContainer, // 0x03
];

const SAFE_DATA_TYPE: &[SafeDataType] = &[
    SafeDataType::SafeKey,              // 0x00
    SafeDataType::PublicImmutableData,  // 0x01
    SafeDataType::PrivateImmutableData, // 0x02
    SafeDataType::PublicSequence,       // 0x03
    SafeDataType::PrivateSequence,      // 0x04
    SafeDataType::SeqMutableData,       // 0x05
    SafeDataType::UnseqMutableData,     // 0x06
];

declare_types! {
    /// JS class wrapping XorUrlEncoder struct
    pub class JsXorUrlEncoder for XorUrlEncoder {
        // Initialise a new XorUrlEncoder instance
        // Binding for: pub fn new(xorname: XorName, nrs_name: Option<&str>, type_tag: u64, data_type: SafeDataType, content_type: SafeContentType, path: Option<&str>, sub_names: Option<Vec<String>>, query_string: Option<&str>, fragment: Option<&str>, content_version: Option<u64>) -> Self
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

            let nrs_name = get_opt_str_to_js_undefined(&mut cx, 1)?;

            let type_tag = cx.argument::<JsNumber>(2)?.value() as u64;

            let data_type_index = cx.argument::<JsNumber>(3)?.value();
            let data_type = SAFE_DATA_TYPE[data_type_index as usize].clone();

            let v3: Handle<JsValue> = cx.argument(4)?;
            let content_type = if v3.is_a::<JsNumber>() {
                let content_type_index = cx.argument::<JsNumber>(4)?.value();
                SAFE_CONTENT_TYPE[content_type_index as usize].clone()
            } else if v3.is_a::<JsString>() {
                let media_type_str = cx.argument::<JsString>(4)?.value();
                SafeContentType::MediaType(media_type_str.to_string())
            } else {
                panic!("MediaType argument contains an invalid value");
            };

            let path = get_opt_str_to_js_undefined(&mut cx, 5)?;

            let js_arr_handle: Handle<JsArray> = cx.argument(6)?;
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

            let query_string = get_opt_str_to_js_undefined(&mut cx, 7)?;

            let fragment = get_opt_str_to_js_undefined(&mut cx, 8)?;

            let content_version = match cx.argument_opt(9) {
                Some(arg) => Some(arg.downcast::<JsNumber>().or_throw(&mut cx)?.value() as u64),
                None => None
            };

            debug!("Creating XorUrlEncoder instance");
            let xorurl_encoder = XorUrlEncoder::new(
                xorname,
                nrs_name.as_deref(),
                type_tag,
                data_type,
                content_type,
                path.as_deref(),
                sub_names,
                query_string.as_deref(),
                fragment.as_deref(),
                content_version
            ).unwrap_or_else(|err| { panic!(format!("Failed to instantiate XorUrlEncoder: {:?}", err)) } );

            Ok(xorurl_encoder)
        }

        // Binding for: pub fn encoding_version(&self) -> u64
        method encoding_version(mut cx) {
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.encoding_version()
            };
            Ok(cx.number(data as f64).upcast())
        }

        // Binding for: pub fn data_type(&self) -> SafeDataType
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

        // Binding for: pub fn content_type(&self) -> SafeContentType
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

        // Binding for: pub fn xorname(&self) -> XorName
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

        // Binding for: pub fn type_tag(&self) -> u64
        method type_tag(mut cx) {
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.type_tag()
            };
            Ok(cx.number(data as f64).upcast())
        }

        // Binding for: pub fn path(&self) -> &str
        method path(mut cx) {
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.path().to_string()
            };
            Ok(cx.string(&data).upcast())
        }

        // Binding for: pub fn set_path(&mut self, path: &str)
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

        // Binding for: pub fn sub_names(&self) -> Vec<String>
        method sub_names(mut cx) {
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.sub_names_vec().to_vec()
            };

            let js_array = JsArray::new(&mut cx, data.len() as u32);
            for (i, obj) in data.iter().enumerate() {
                let js_string = cx.string(obj);
                js_array.set(&mut cx, i as u32, js_string).unwrap();
            }

            Ok(js_array.upcast())
        }

        // Binding for: pub fn content_version(&self) -> Option<u64>
        method content_version(mut cx) {
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.content_version().map_or_else(|| 0 as f64, |v| v as f64)
            };
            Ok(cx.number(data).upcast())
        }

        // Binding for: pub fn set_content_version(&mut self, version: Option<u64>)
        method set_content_version(mut cx) {
            let version = get_optional_number(&mut cx, 0).map(|r| r.map(|v| v as u64))?;
            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                user.set_content_version(version);
            }

            Ok(cx.undefined().upcast())
        }

        // Binding for: pub fn to_string(&self) -> sn_api::Result<String>
        method to_string(mut cx) {
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.to_string()
            };
            Ok(cx.string(&data).upcast())
        }

        // Binding for: pub fn to_base(&self, base: &str) -> sn_api::Result<String>
        method to_base(mut cx) {
            let base = cx.argument::<JsString>(0)?.value();
            let xorurl_base = XorUrlBase::from_str(&base).unwrap_or_else(|err| { panic!(format!("Failed to parse base {}: {:?}", base, err)) } );
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.to_base(xorurl_base)
            };
            Ok(cx.string(&data).upcast())
        }
    }

    /// JS class wrapping Safe struct
    pub class JsSafe for Safe {
        // Initialise a new Safe instance
        init(mut cx) {
            let xorurl_base = match cx.argument_opt(0) {
                Some(arg) => {
                    let base = arg.downcast::<JsString>().or_throw(&mut cx)?.value();
                    let xorurl_base = XorUrlBase::from_str(&base).unwrap_or_else(|err| { panic!(format!("Failed to parse base {}: {:?}", base, err)) } );
                    Some(xorurl_base)
                },
                None => None
            };
            debug!("Creating Safe API instance with xorurl base: '{:?}'", xorurl_base);
            let safe = Safe::new(xorurl_base);

            Ok(safe)
        }

        // Gets the XOR-URL base encoding set to be used for XOR-URLs generated
        method xorurl_base(mut cx) {
            let base = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.xorurl_base
            };
            Ok(cx.string(base.to_string()).upcast())
        }

        // Generate an authorisation request string and send it to a SAFE Authenticator.
        // Ir returns the credentials necessary to connect to the network, encoded in a single string.
        // Binding for: pub async fn auth_app(&mut self, app_id: &str, app_name: &str, app_vendor: &str, port: Option<&str>) -> sn_api::Result<String>
        method auth_app(mut cx) {
            let app_id = cx.argument::<JsString>(0)?.value();
            let app_name = cx.argument::<JsString>(1)?.value();
            let app_vendor = cx.argument::<JsString>(2)?.value();
            let port = get_optional_string(&mut cx, 3)?;
            debug!("Sending application authorisation request...");
            let mut rt = Runtime::new().unwrap();
            let auth_credentials = rt.block_on(Safe::auth_app(&app_id, &app_name, &app_vendor, port.as_deref())).unwrap_or_else(|err| { panic!(format!("Failed to authorise application: {:?}", err)) } );
            rt.shutdown_timeout(Duration::from_millis(1));
            debug!("Application successfully authorised!");
            Ok(cx.string(&auth_credentials).upcast())
        }

        // Connect to the SAFE Network using the provided app id and auth credentials
        // Binding for: pub fn connect(&mut self, app_id: &str, auth_credentials: Option<&str>) -> sn_api::Result<()>
        method connect(mut cx) {
            let app_id = cx.argument::<JsString>(0)?.value();
            let credentials = get_optional_string(&mut cx, 1)?;

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                rt.block_on(user.connect(&app_id, credentials.as_deref())).unwrap_or_else(|err| { panic!(format!("Failed to connect: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                debug!("Successfully connected to the Network!");
            }
            debug!("ALL GOOD Successfully connected to the Network!");
            Ok(cx.undefined().upcast())
        }

        // Retrieve data from a safe:// URL
        // Binding for: pub async fn fetch(&self, url: &str, range: Range) -> sn_api::Result<SafeData>
        method fetch(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            let range = match cx.argument_opt(1) {
                Some(arg) => {
                    if arg.is_a::<JsNull>() {
                        None
                    } else {
                        match arg.downcast::<JsObject>() {
                            Ok(range_obj) => {
                                let start = match range_obj.get(&mut cx, "start") {
                                    Ok(attr) => {
                                        let start_value_handle = attr
                                            .downcast::<JsNumber>()
                                            .unwrap_or_else(|err| panic!(format!("Invalid 'start' range value: {:?}", err)));
                                        Some(start_value_handle.value() as u64)
                                    }
                                    Err(_) => None,
                                };
                                let end = match range_obj.get(&mut cx, "end") {
                                    Ok(attr) => {
                                        let end_value_handle = attr
                                            .downcast::<JsNumber>()
                                            .unwrap_or_else(|err| panic!(format!("Invalid 'end' range value: {:?}", err)));
                                        Some(end_value_handle.value() as u64)
                                    }
                                    Err(_) => None,
                                };

                                Some((start, end))
                            },
                            Err(err) => panic!(err.to_string()),
                        }
                    }
                }
                None => None,
            };

            debug!("Fetching from {} with range {:?}", url, range);

            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.fetch(&url, range)).unwrap_or_else(|err| { panic!(format!("Failed to fetch content from '{}': {:?}", url, err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Inspect URL without retrieving the actual targeted data
        // Binding for: pub async fn inspect(&self, url: &str) -> sn_api::Result<SafeData>
        method inspect(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            debug!("Inspecting '{}' ...", url);

            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.inspect(&url)).unwrap_or_else(|err| { panic!(format!("Failed to inspect '{}': {:?}", url, err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        //**** FilesContainer ****//

        // Upload files/folder into a new FilesContainer returning its XOR-URL
        // Binding for: pub async fn files_container_create(&mut self, location: Option<&str>, dest: Option<&str>, recursive: bool, follow_links: bool, dry_run: bool) -> sn_api::Result<(XorUrl, ProcessedFiles, FilesMap)>
        method files_container_create(mut cx) {
            let location = get_optional_string(&mut cx, 0)?;
            let dest = get_optional_string(&mut cx, 1)?;

            let recursive = cx.argument::<JsBoolean>(2)?.value();
            let follow_links = cx.argument::<JsBoolean>(3)?.value();
            let dry_run = cx.argument::<JsBoolean>(4)?.value();
            debug!("Creating FilesContainer: {:?} - {:?} - {} - {} - {}", location, dest, recursive, follow_links, dry_run);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.files_container_create(location.as_deref(), dest.as_deref(), recursive, follow_links, dry_run)).unwrap_or_else(|err| { panic!(format!("Failed to create FilesContainer: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Sync up files/folder with an existing FilesContainer
        // Binding for: pub async fn files_container_sync(&mut self, location: &str, url: &str, recursive: bool, follow_links: bool, delete: bool, update_nrs: bool, dry_run: bool) -> sn_api::Result<(u64, ProcessedFiles, FilesMap)>
        method files_container_sync(mut cx) {
            let location = cx.argument::<JsString>(0)?.value();
            let url = cx.argument::<JsString>(1)?.value();
            let recursive = cx.argument::<JsBoolean>(2)?.value();
            let follow_links = cx.argument::<JsBoolean>(3)?.value();
            let delete = cx.argument::<JsBoolean>(4)?.value();
            let update_nrs = cx.argument::<JsBoolean>(5)?.value();
            let dry_run = cx.argument::<JsBoolean>(6)?.value();
            debug!("Sync-ing FilesContainer: {} - {} - {} - {} - {} - {} - {}", location, url, recursive, follow_links, delete, update_nrs, dry_run);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.files_container_sync(&location, &url, recursive, follow_links, delete, update_nrs, dry_run)).unwrap_or_else(|err| { panic!(format!("Failed to sync up FilesContainer: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Fetch an existing FilesContainer
        // Binding for: pub async fn files_container_get(&self, url: &str) -> sn_api::Result<(u64, FilesMap)>
        method files_container_get(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            debug!("Fetching FilesContainer from: {}", url);

            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.files_container_get(&url)).unwrap_or_else(|err| { panic!(format!("Failed to fetch FilesContainer: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Add file to an existing FilesContainer
        // Binding for: pub async fn files_container_add(&mut self, source_file: &str, url: &str, force: bool, update_nrs: bool, follow_links: bool, dry_run: bool) -> sn_api::Result<(u64, ProcessedFiles, FilesMap)>
        method files_container_add(mut cx) {
            let source_file = cx.argument::<JsString>(0)?.value();
            let url = cx.argument::<JsString>(1)?.value();
            let force = cx.argument::<JsBoolean>(2)?.value();
            let update_nrs = cx.argument::<JsBoolean>(3)?.value();
            let follow_links = cx.argument::<JsBoolean>(4)?.value();
            let dry_run = cx.argument::<JsBoolean>(5)?.value();
            debug!("Adding to FilesContainer: {} - {} - {} - {} - {} - {}", source_file, url, force, update_nrs, follow_links, dry_run);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.files_container_add(&source_file, &url, force, update_nrs, follow_links, dry_run)).unwrap_or_else(|err| { panic!(format!("Failed to add file to FilesContainer: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Add file from raw bytes to an existing FilesContainer
        // Binding for: pub async fn files_container_add_from_raw(&mut self, data: &[u8], url: &str, force: bool, update_nrs: bool, dry_run: bool) -> sn_api::Result<(u64, ProcessedFiles, FilesMap)>
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
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.files_container_add_from_raw(&data, &url, force, update_nrs, dry_run)).unwrap_or_else(|err| { panic!(format!("Failed to add file form raw bytes to FilesContainer: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Remove a file/path from an existing FilesContainer.
        // Binding for: pub async fn files_container_remove_path(&mut self, url: &str, recursive: bool, update_nrs: bool, dry_run: bool) -> Result<(u64, ProcessedFiles, FilesMap)>
        method files_container_remove_path(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            let recursive = cx.argument::<JsBoolean>(1)?.value();
            let update_nrs = cx.argument::<JsBoolean>(2)?.value();
            let dry_run = cx.argument::<JsBoolean>(3)?.value();
            debug!("Removing filepath from FilesContainer: {} - {} - {} - {}", url, recursive, update_nrs, dry_run);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.files_container_remove_path(&url, recursive, update_nrs, dry_run)).unwrap_or_else(|err| { panic!(format!("Failed to remove file/path from FilesContainer: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Pub PublicImmutableData
        // Binding for: pub async fn files_put_public_immutable(&mut self, data: &[u8], media_type: Option<&str>, dry_run: bool) -> sn_api::Result<XorUrl>
        method files_put_public_immutable(mut cx) {
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

            let media_type = get_optional_string(&mut cx, 1)?;
            let dry_run = cx.argument::<JsBoolean>(2)?.value();
            debug!("Putting PublicImmutableData: {:?}", data);

            let url = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.files_put_public_immutable(&data, media_type.as_deref(), dry_run)).unwrap_or_else(|err| { panic!(format!("Failed to put PublicImmutableData: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            Ok(cx.string(&url).upcast())
        }

        // Get a PublicImmutableData
        // Binding for: pub async fn files_get_public_immutable(&self, url: &str) -> sn_api::Result<Vec<u8>>
        method files_get_public_immutable(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            debug!("Fetching PublicImmutableData from: {}", url);

            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.files_get_public_immutable(&url, None)).unwrap_or_else(|err| { panic!(format!("Failed to fetch PublicImmutableData: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }


        //**** NRS ****//

        // NRS Map Container create
        // Binding for: pub async fn nrs_map_container_create(&mut self, name: &str, link: &str, default: bool, hard_link: bool, dry_run: bool) -> sn_api::Result<(XorUrl, ProcessedEntries, NrsMap)>
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
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.nrs_map_container_create(&name, &link, default, hard_link, dry_run)).unwrap_or_else(|err| { panic!(format!("Failed to create NRS Map Container: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Add/update a subname to an existing NRS Map Container
        // Binding for: pub async fn nrs_map_container_add(&mut self, name: &str, link: &str, default: bool, hard_link: bool, dry_run: bool) -> sn_api::Result<(u64, XorUrl, ProcessedEntries, NrsMap)>
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
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.nrs_map_container_add(&name, &link, default, hard_link, dry_run)).unwrap_or_else(|err| { panic!(format!("Failed to create NRS Map Container: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Remove an NRS Map Container
        // Binding for: pub async fn nrs_map_container_remove(&mut self, name: &str, dry_run: bool) -> sn_api::Result<(u64, XorUrl, ProcessedEntries, NrsMap)>
        method nrs_map_container_remove(mut cx) {
            let name = cx.argument::<JsString>(0)?.value();
            let dry_run = cx.argument::<JsBoolean>(1)?.value();
            debug!("Removing an NRS Map Container: {} - {}", name, dry_run);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.nrs_map_container_remove(&name, dry_run)).unwrap_or_else(|err| { panic!(format!("Failed to remove an NRS Map Container: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Fetch an NRS Map Container
        // Binding for: pub async fn nrs_map_container_get(&self, url: &str) -> sn_api::Result<(u64, NrsMap)>
        method nrs_map_container_get(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            debug!("Fetching NRS Map Container from: {}", url);

            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.nrs_map_container_get(&url)).unwrap_or_else(|err| { panic!(format!("Failed to fetch an NRS Map Container: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        //**** Sequence ****///

        // Store a Sequence
        // Binding for: pub async fn sequence_create(&mut self, data: &[u8], name: Option<XorName>, type_tag: u64, private: bool) -> Result<XorUrl>
        method sequence_create(mut cx) {
            let v: Handle<JsValue> = cx.argument(0)?;
            let buffer: Handle<JsBuffer>;
            let data = if v.is_a::<JsBuffer>() || v.is_a::<JsArrayBuffer>() {
                buffer = cx.argument(0)?;
                cx.borrow(&buffer, |data| data.as_slice::<u8>())
            } else {
                panic!("A Buffer or ArrayBuffer is expected as first argument");
            };

            let name = match cx.argument_opt(1) {
                Some(arg) => {
                    if arg.is_a::<JsNull>() {
                        None
                    } else {
                        let buffer: Handle<JsBuffer>;
                        let xorname_slice = if arg.is_a::<JsBuffer>() || arg.is_a::<JsArrayBuffer>() {
                            buffer = cx.argument(1)?;
                            cx.borrow(&buffer, |data| data.as_slice::<u8>())
                        } else {
                            panic!("A Buffer or ArrayBuffer is expected as second argument");
                        };
                        let mut xorname = XorName::default();
                        xorname.0.copy_from_slice(&xorname_slice);
                        Some(xorname)
                    }
                }
                None => None,
            };

            let type_tag = cx.argument::<JsNumber>(2)?.value() as u64;
            let dry_run = cx.argument::<JsBoolean>(3)?.value();

            debug!("Storing Sequence at: {:?} - {} - {}", name, type_tag, dry_run);

            let url = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.sequence_create(&data, name, type_tag, dry_run)).unwrap_or_else(|err| { panic!(format!("Failed to store Sequence: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            Ok(cx.string(&url).upcast())
        }

        // Get a Sequence
        // Binding for: pub async fn sequence_get(&self, url: &str) -> Result<(u64, Vec<u8>)>
        method sequence_get(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            debug!("Fetching Sequence from: {}", url);

            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.sequence_get(&url)).unwrap_or_else(|err| { panic!(format!("Failed to fetch Sequence: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Append to a Sequence
        // Binding for: pub async fn sequence_append(&mut self, url: &str, data: &[u8]) -> Result<()>
        method sequence_append(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();

            let v: Handle<JsValue> = cx.argument(1)?;
            let buffer: Handle<JsBuffer>;
            let data = if v.is_a::<JsBuffer>() || v.is_a::<JsArrayBuffer>() {
                buffer = cx.argument(1)?;
                cx.borrow(&buffer, |data| data.as_slice::<u8>())
            } else {
                panic!("A Buffer or ArrayBuffer is expected as first argument");
            };

            debug!("Appending to Sequence at: {}", url);

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                rt.block_on(user.sequence_append(&url, &data)).unwrap_or_else(|err| { panic!(format!("Failed to append to Sequence: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
            };

            Ok(cx.undefined().upcast())
        }

        //**** Keys ****///

        // Generate a key pair without creating and/or storing a SafeKey on the network
        // Binding for: pub fn keypair(&self) -> sn_api::Result<BlsKeyPair>
        method keypair(mut cx) {
            debug!("Generating random key pair");
            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.keypair().unwrap_or_else(|err| { panic!(format!("Failed to generate a key pair: {:?}", err)) } )
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Create a SafeKey on the network and return its XOR-URL.
        // Binding for: pub async fn keys_create(&mut self, from: Option<&str>, preload_amount: Option<&str>, pk: Option<&str>) -> sn_api::Result<(XorUrl, Option<BlsKeyPair>)>
        method keys_create(mut cx) {
            let from = get_optional_string(&mut cx, 0)?;
            let preload_amount = get_optional_string(&mut cx, 1)?;
            let pk = get_optional_string(&mut cx, 2)?;
            debug!("Creating a SafeKey preloaded with '{:?}' coins", preload_amount);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.keys_create(from.as_deref(), preload_amount.as_deref(), pk.as_deref())).unwrap_or_else(|err| { panic!(format!("Failed to create a SafeKey: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Create a SafeKey on the network, allocates testcoins onto it, and return the SafeKey's XOR-URL
        // Binding for: pub async fn keys_create_preload_test_coins(&mut self, preload_amount: &str) -> sn_api::Result<(XorUrl, Option<BlsKeyPair>)>
        method keys_create_preload_test_coins(mut cx) {
            let preload_amount = cx.argument::<JsString>(0)?.value();
            debug!("Creating SafeKey with ('{}') test-coins", preload_amount);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.keys_create_preload_test_coins(&preload_amount)).unwrap_or_else(|err| { panic!(format!("Failed to create a SafeKey with test coins: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Check SafeKey's balance from the network from a given SecretKey string
        // Binding for: pub async fn keys_balance_from_sk(&self, sk: &str) -> sn_api::Result<String>
        method keys_balance_from_sk(mut cx) {
            let sk = cx.argument::<JsString>(0)?.value();
            debug!("Checking SafeKey balance...");

            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.keys_balance_from_sk(&sk)).unwrap_or_else(|err| { panic!(format!("Failed query the balance from SafeKey: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            Ok(cx.string(data).upcast())
        }

        // Check SafeKey's balance from the network from a given XOR/NRS-URL and secret key string.
        // Binding for: pub async fn keys_balance_from_url(&self, url: &str, sk: &str) -> sn_api::Result<String>
        method keys_balance_from_url(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            let sk = cx.argument::<JsString>(1)?.value();
            debug!("Checking SafeKey balance from URL '{:?}'", url);

            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.keys_balance_from_url(&url, &sk)).unwrap_or_else(|err| { panic!(format!("Failed to check balance from the SafeKey URL '{}': {:?}", url, err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            Ok(cx.string(data).upcast())
        }

        // Check that the XOR/NRS-URL corresponds to the public key derived from the provided secret key
        // Binding for: pub async fn validate_sk_for_url(&self, sk: &str, url: &str) -> sn_api::Result<String>
        method validate_sk_for_url(mut cx) {
            let sk = cx.argument::<JsString>(0)?.value();
            let url = cx.argument::<JsString>(1)?.value();
            debug!("Validating secret key for URL '{:?}'", url);

            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.validate_sk_for_url(&sk, &url)).unwrap_or_else(|err| { panic!(format!("Failed to vaildate the secret key for the SafeKey URL '{}': {:?}", url, err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            Ok(cx.string(data).upcast())
        }

        // Transfer safecoins from one SafeKey to another, or to a Wallet
        // Binding for: pub async fn keys_transfer(&mut self, amount: &str, from_sk: Option<&str>, to_url: &str, tx_id: Option<u64>) -> sn_api::Result<u64>
        method keys_transfer(mut cx) {
            let amount = cx.argument::<JsString>(0)?.value();
            let from_sk = get_optional_string(&mut cx, 1)?;
            let to_url = cx.argument::<JsString>(2)?.value();
            let tx_id = get_optional_number(&mut cx, 3).map(|r| r.map(|v| v as u64))?;
            debug!("Transferring '{}' from SafeKey", amount);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.keys_transfer(&amount, from_sk.as_deref(), &to_url, tx_id)).unwrap_or_else(|err| { panic!(format!("Failed to transfer from SafeKey: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            Ok(cx.number(data as f64).upcast())
        }


        //**** Wallet ****//

        // Create an empty Wallet and return its XOR-URL
        // Binding for: pub async fn wallet_create(&mut self) -> sn_api::Result<XorUrl>
        method wallet_create(mut cx) {
            debug!("Creating a Wallet...");
            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.wallet_create()).unwrap_or_else(|err| { panic!(format!("Failed to create Wallet: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            Ok(cx.string(data).upcast())
        }

        // Add a SafeKey to a Wallet to make it spendable, and returns the friendly name set for it
        // Binding for: pub async fn wallet_insert(&mut self, url: &str, name: Option<&str>, default: bool, sk: &str) -> sn_api::Result<String>
        method wallet_insert(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            let name = get_optional_string(&mut cx, 1)?;
            let default = cx.argument::<JsBoolean>(2)?.value();
            let sk = cx.argument::<JsString>(3)?.value();
            debug!("Inserting '{:?}' in Wallet at '{}'", name, url);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.wallet_insert(&url, name.as_deref(), default, &sk)).unwrap_or_else(|err| { panic!(format!("Failed to insert in Wallet: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            Ok(cx.string(data).upcast())
        }

        // Check the total balance of a Wallet found at a given XOR-URL
        // Binding for: pub async fn wallet_balance(&mut self, url: &str) -> sn_api::Result<String>
        method wallet_balance(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            debug!("Checking balance of a Wallet at '{}'", url);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.wallet_balance(&url)).unwrap_or_else(|err| { panic!(format!("Failed to check balance of Wallet at '{}': {:?}", url, err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            Ok(cx.string(data).upcast())
        }

        // Binding for: pub async fn wallet_get_default_balance(&self, url: &str) -> sn_api::Result<(WalletSpendableBalance, u64)>
        method wallet_get_default_balance(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            debug!("Fetching default spendable balance from Wallet at '{:?}'", url);

            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.wallet_get_default_balance(&url)).unwrap_or_else(|err| { panic!(format!("Failed to get default spendable balance from Wallet at '{}': {:?}", url, err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Binding for: pub async fn wallet_transfer(&mut self, amount: &str, from_url: Option<&str>, to_url: &str, tx_id: Option<u64>) -> sn_api::Result<u64>
        method wallet_transfer(mut cx) {
            let amount = cx.argument::<JsString>(0)?.value();
            let from_url = get_optional_string(&mut cx, 1)?;
            let to_url = cx.argument::<JsString>(2)?.value();
            let tx_id = match cx.argument_opt(3) {
                Some(arg) => Some(arg.downcast::<JsNumber>().or_throw(&mut cx)?.value() as u64),
                None => None
            };
            debug!("Transferring '{}' from Wallet at '{:?}'", amount, from_url);

            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.wallet_transfer(&amount, from_url.as_deref(), &to_url, tx_id)).unwrap_or_else(|err| { panic!(format!("Failed to transfer from Wallet at: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            Ok(cx.number(data as f64).upcast())
        }

        // Binding for: pub async fn wallet_get(&self, url: &str) -> sn_api::Result<WalletSpendableBalances>
        method wallet_get(mut cx) {
            let url = cx.argument::<JsString>(0)?.value();
            debug!("Fetching Wallet from '{:?}'", url);

            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.wallet_get(&url)).unwrap_or_else(|err| { panic!(format!("Failed to get Wallet from '{}': {:?}", url, err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }
    }

    /// JS class wrapping SafeAuthdClient struct
    pub class JsSafeAuthdClient for SafeAuthdClient {
        // Initialise a new SafeAuthdClient instance
        init(mut cx) {
            let port = match cx.argument_opt(0) {
                Some(arg) => Some(arg.downcast::<JsString>().or_throw(&mut cx)?.value()),
                None => None
            };
            debug!("Creating SafeAuthdClient API instance with port number: '{:?}'", port);
            let sn_authd_client = SafeAuthdClient::new(port);

            Ok(sn_authd_client)
        }

        // Start the Authenticator daemon
        // Binding for: pub fn start(&self, authd_path: Option<&str>) -> sn_api::Result<()>
        method start(mut cx) {
            let authd_path = get_optional_string(&mut cx, 0)?;
            debug!("Starting authd from {:?} ...", authd_path);

            {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.start(authd_path.as_deref()).unwrap_or_else(|err| { panic!(format!("Failed to start authd from '{:?}': {:?}", authd_path, err)) } )
            };

            Ok(cx.undefined().upcast())
        }

        // Stop the Authenticator daemon
        // Binding for: pub fn stop(&self, authd_path: Option<&str>) -> sn_api::Result<()>
        method stop(mut cx) {
            let authd_path = get_optional_string(&mut cx, 0)?;
            debug!("Stopping authd from {:?} ...", authd_path);

            {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.stop(authd_path.as_deref()).unwrap_or_else(|err| { panic!(format!("Failed to stop authd from '{:?}': {:?}", authd_path, err)) } )
            };

            Ok(cx.undefined().upcast())
        }

        // Restart the Authenticator daemon
        // Binding for: pub fn restart(&self, authd_path: Option<&str>) -> sn_api::Result<()>
        method restart(mut cx) {
            let authd_path = get_optional_string(&mut cx, 0)?;
            debug!("Restarting authd from {:?} ...", authd_path);

            {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                user.restart(authd_path.as_deref()).unwrap_or_else(|err| { panic!(format!("Failed to restart authd from '{:?}': {:?}", authd_path, err)) } )
            };

            Ok(cx.undefined().upcast())
        }

        // Send a request to remote authd endpoint to retrieve an status report
        // Binding for: pub async fn status(&self) -> sn_api::Result<AuthdStatus>
        method status(mut cx) {
            debug!("Retrieving authd status report...");
            let data = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                let data = rt.block_on(user.status()).unwrap_or_else(|err| { panic!(format!("Failed to retrieve authd status report: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
                data
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Send a login action request to remote authd endpoint
        // Binding for: pub async fn log_in(&mut self, secret: &str, password: &str) -> sn_api::Result<()>
        method log_in(mut cx) {
            let secret = cx.argument::<JsString>(0)?.value();
            let password = cx.argument::<JsString>(1)?.value();
            debug!("Logging in...");

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                rt.block_on(user.log_in(&secret, &password)).unwrap_or_else(|err| { panic!(format!("Failed to log in: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
            };

            Ok(cx.undefined().upcast())
        }

        // Sends a logout action request to the SAFE Authenticator
        // Binding for: pub async fn log_out(&mut self) -> sn_api::Result<()>
        method log_out(mut cx) {
            debug!("Logging out...");

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                rt.block_on(user.log_out()).unwrap_or_else(|err| { panic!(format!("Failed to log out: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
            };

            Ok(cx.undefined().upcast())
        }

        // Sends an account creation request to the SAFE Authenticator
        // Binding for: pub async fn create_acc(&self, sk: &str, secret: &str, password: &str) -> sn_api::Result<()>
        method create_acc(mut cx) {
            let sk = cx.argument::<JsString>(0)?.value();
            let secret = cx.argument::<JsString>(1)?.value();
            let password = cx.argument::<JsString>(2)?.value();
            debug!("Creating a SAFE account...");

            {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                rt.block_on(user.create_acc(&sk, &secret, &password)).unwrap_or_else(|err| { panic!(format!("Failed to create SAFE account: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
            };

            Ok(cx.undefined().upcast())
        }

        // Get the list of applications authorised from remote authd
        // Binding for: pub async fn authed_apps(&self) -> sn_api::Result<AuthedAppsList>
        method authed_apps(mut cx) {
            debug!("Retrieving list of authorised apps...");

            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                rt.block_on(user.authed_apps()).unwrap_or_else(|err| { panic!(format!("Failed to retrieve list of authorised apps: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Revoke all permissions from an application
        // Binding for: pub async fn revoke_app(&self, app_id: &str) -> sn_api::Result<()>
        method revoke_app(mut cx) {
            let app_id = cx.argument::<JsString>(0)?.value();
            debug!("Revoking app with ID: {}", app_id);

            {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                rt.block_on(user.revoke_app(&app_id)).unwrap_or_else(|err| { panic!(format!("Failed to revoke app ('{}'): {:?}", app_id, err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
            };

            Ok(cx.undefined().upcast())
        }

        // Get the list of pending authorisation requests from remote authd
        // Binding for: pub async fn auth_reqs(&self) -> sn_api::Result<PendingAuthReqs>
        method auth_reqs(mut cx) {
            debug!("Retrieving list of pending authorisation requests...");

            let data = {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                rt.block_on(user.auth_reqs()).unwrap_or_else(|err| { panic!(format!("Failed to retrieve list of pending authorisation requests: {:?}", err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
            };

            let js_value = neon_serde::to_value(&mut cx, &data).map_err(|_| Throw)?;
            Ok(js_value)
        }

        // Allow an authorisation request
        // Binding for: pub async fn allow(&self, req_id: SafeAuthReqId) -> sn_api::Result<()>
        method allow(mut cx) {
            let req_id = cx.argument::<JsNumber>(0)?.value() as u32;
            debug!("Allowing authorisation request with ID: {}", req_id);

            {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                rt.block_on(user.allow(req_id)).unwrap_or_else(|err| { panic!(format!("Failed to allow authorisation request ('{}'): {:?}", req_id, err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
            };

            Ok(cx.undefined().upcast())
        }

        // Deny an authorisation request
        // Binding for: pub async fn deny(&self, req_id: SafeAuthReqId) -> sn_api::Result<()>
        method deny(mut cx) {
            let req_id = cx.argument::<JsNumber>(0)?.value() as u32;
            debug!("Denying authorisation request with ID: {}", req_id);

            {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                rt.block_on(user.deny(req_id)).unwrap_or_else(|err| { panic!(format!("Failed to allow authorisation request ('{}'): {:?}", req_id, err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
            };

            Ok(cx.undefined().upcast())
        }

        // Subscribe a callback to receive notifications to allow/deny authorisation requests
        // Binding for: pub async fn subscribe(&mut self, endpoint_url: &str, app_id: &str, allow_cb: &'static AuthAllowPrompt) -> sn_api::Result<()>
        method subscribe(mut cx) {
            let endpoint_url = cx.argument::<JsString>(0)?.value();
            let app_id = cx.argument::<JsString>(1)?.value();
            let js_callback = cx.argument::<JsFunction>(2)?;
            let this = cx.this();
            let cb = EventHandler::new(&cx, this, js_callback);

            let allow_auth_cb = move |auth_req: AuthReq| -> Option<bool> {
                cb.schedule(move |cx| {
                    let cb_args: Vec<Handle<JsString>> = vec![
                        cx.string(auth_req.app_id),
                        cx.string(auth_req.req_id.to_string())
                    ];
                    cb_args
                });
                // Since we cannot obtain a return value from the JS callback out of the schedule,
                // we return no decision. The JS app will need to call 'allow' API to authorise the request.
                None
            };

            debug!("Subscribing to receive auth req notifs...");

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                rt.block_on(user.subscribe(&endpoint_url, &app_id, allow_auth_cb)).unwrap_or_else(|err| { panic!(format!("Failed to subscribe ('{}'): {:?}", endpoint_url, err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
            };

            Ok(cx.undefined().upcast())
        }

        // Subscribe an endpoint URL where notifications to allow/deny authorisation requests shall be sent
        // Binding for: pub async fn subscribe_url(&self, endpoint_url: &str) -> sn_api::Result<()>
        method subscribe_url(mut cx) {
            let endpoint_url = cx.argument::<JsString>(0)?.value();
            debug!("Subscribing URL for auth req notifs...");

            {
                let this = cx.this();
                let guard = cx.lock();
                let user = this.borrow(&guard);
                let mut rt = Runtime::new().unwrap();
                rt.block_on(user.subscribe_url(&endpoint_url)).unwrap_or_else(|err| { panic!(format!("Failed to subscribe URL ('{}'): {:?}", endpoint_url, err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
            };

            Ok(cx.undefined().upcast())
        }

        // Unsubscribe from notifications to allow/deny authorisation requests
        // Binding for: pub async fn unsubscribe(&self, endpoint_url: &str) -> sn_api::Result<()>
        method unsubscribe(mut cx) {
            let endpoint_url = cx.argument::<JsString>(0)?.value();
            debug!("Unsubscribing URL from auth req notifs...");

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut user = this.borrow_mut(&guard);
                let mut rt = Runtime::new().unwrap();
                rt.block_on(user.unsubscribe(&endpoint_url)).unwrap_or_else(|err| { panic!(format!("Failed to unsubscribe URL ('{}'): {:?}", endpoint_url, err)) } );
                rt.shutdown_timeout(Duration::from_millis(1));
            };

            Ok(cx.undefined().upcast())
        }
    }
}

fn get_opt_str_to_js_undefined(
    cx: &mut CallContext<JsUndefined>,
    arg_index: i32,
) -> Result<Option<String>, neon::result::Throw> {
    let optional_value = match cx.argument_opt(arg_index) {
        Some(arg) => {
            if arg.is_a::<JsNull>() {
                None
            } else {
                match arg.downcast::<JsString>() {
                    Ok(a) => Some(a.value()),
                    Err(err) => panic!(err.to_string()),
                }
            }
        }
        None => None,
    };
    Ok(optional_value)
}

fn get_optional_string<T: neon::object::Class>(
    cx: &mut CallContext<T>,
    arg_index: i32,
) -> Result<Option<String>, neon::result::Throw> {
    let optional_value = match cx.argument_opt(arg_index) {
        Some(arg) => {
            if arg.is_a::<JsNull>() {
                None
            } else {
                match arg.downcast::<JsString>() {
                    Ok(a) => Some(a.value()),
                    Err(err) => panic!(err.to_string()),
                }
            }
        }
        None => None,
    };
    Ok(optional_value)
}

fn get_optional_number<T: neon::object::Class>(
    cx: &mut CallContext<T>,
    arg_index: i32,
) -> Result<Option<f64>, neon::result::Throw> {
    let optional_value = match cx.argument_opt(arg_index) {
        Some(arg) => {
            if arg.is_a::<JsNull>() {
                None
            } else {
                match arg.downcast::<JsNumber>() {
                    Ok(a) => Some(a.value()),
                    Err(err) => panic!(err.to_string()),
                }
            }
        }
        None => None,
    };
    Ok(optional_value)
}

register_module!(mut m, {
    env_logger::init();
    m.export_class::<JsSafe>("Safe")?;
    m.export_class::<JsXorUrlEncoder>("XorUrlEncoder")?;
    m.export_class::<JsSafeAuthdClient>("SafeAuthdClient")?;

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
