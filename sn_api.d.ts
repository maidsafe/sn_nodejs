export class Safe {
    constructor(xor_url_base: 'base32z' | 'base32' | 'base64' | undefined, timeout: Duration);

    connect(app_keypair?: Keypair, config_path?: string, bootstrap_config?: string[]): Promise<void>;

    keys_create_preload_test_coins(a: string): Promise<[string, Keypair]>;
    keys_balance_from_sk(a: SecretKey): Promise<string>;

    files_container_create(location: undefined | string, dest: undefined | string, recursive: boolean, follow_links: boolean, dry_run: boolean): Promise<[string, ProcessedFiles, FilesMap]>;

    nrs_map_container_create(name: string, link: string, def: boolean, hard_link: boolean, dry_run: boolean): Promise<[string, ProcessedEntries, NrsMap]>;
    nrs_map_container_add(name: string, link: string, def: boolean, hard_link: boolean, dry_run: boolean): Promise<[number | BigInt, string, ProcessedEntries, NrsMap]>;
    nrs_map_container_get(xor: string): Promise<[number | BigInt, NrsMap]>;
}

type NrsMap = {
    default: DefaultRdf,
    sub_names_map: SubNamesMap,
};

// Rust Enum. E.g: DefaultRdf::OtherRdf(..)
type DefaultRdf = NotSet | ExistingRdf | OtherRdf;
type NotSet = 'NotSet'; // TODO: Confirm this is the type resulting from Enum serialization.
type ExistingRdf = { ExistingRdf: string }; // SubName
type OtherRdf = { OtherRdf: BTreeMap<string> };

type SubNamesMap = BTreeMap<SubNameRdf>;
type SubNameRdf = unknown; // TODO: implement real type.

type ProcessedEntries = BTreeMap<[string, string]>;

// Returned by files_container_create().
type ProcessedFiles = BTreeMap<[string, string]>;
type FilesMap = BTreeMap<[string, string]>;

interface Duration {
    secs: number,
    nanos: number,
}

export class Keypair {
    // Keypair should only be constructed from within the addon.
    private constructor();
    static new_ed25519(): Keypair;

    secret_key(): SecretKey;
}
export class SecretKey { }


export class SafeAuthdClient {
    constructor(endpoint?: string);

    status(): Promise<AuthdStatus>;
}
type AuthdStatus = {
    safe_unlocked: boolean,
    num_auth_reqs: number,
    num_notif_subs: number,
    authd_version: string,
}


// Map Rust's BTreeMap to an object. Can only have strings as keys.
type BTreeMap<V> = { [key: string]: V };