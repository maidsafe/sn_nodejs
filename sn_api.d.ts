export class Safe {
    constructor(xor_url_base: 'base32z' | 'base32' | 'base64' | undefined, timeout: Duration);

    connect(app_keypair?: Keypair, config_path?: String, bootstrap_config?: String[]): Promise<void>;

    keys_create_preload_test_coins(a: String): Promise<[String, Keypair]>;
    keys_balance_from_sk(a: SecretKey): Promise<String>;

    files_container_create(location: undefined | String, dest: undefined | String, recursive: boolean, follow_links: boolean, dry_run: boolean): Promise<[String, ProcessedFiles, FilesMap]>;

    nrs_map_container_create(name: String, link: String, def: boolean, hard_link: boolean, dry_run: boolean): Promise<[String, unknown, NrsMap]>;
    nrs_map_container_get(xor: String): Promise<[number, NrsMap]>;
}

type NrsMap = {
    default: DefaultRdf,
    sub_names_map: unknown,
};

// Rust Enum. E.g: DefaultRdf::OtherRdf(..)
type DefaultRdf = NotSet | ExistingRdf | OtherRdf;
type NotSet = 'NotSet'; // TODO: Confirm this is the type resulting from Enum serialization.
type ExistingRdf = { ExistingRdf: string }; // SubName
type OtherRdf = { OtherRdf: BTreeMap<string> };

interface Duration {
    secs: number,
    nanos: number,
}

// Returned by files_container_create().
type ProcessedFiles = BTreeMap<[string, string]>;
interface FilesMap {
    [path: string]: FilesMapDetails;
}
interface FilesMapDetails {
    [property: string]: string;
}

export class Keypair {
    // Keypair should only be constructed from within the addon.
    private constructor();
    static new_ed25519(): Keypair;

    secret_key(): SecretKey;
}
export class SecretKey { }

// Map Rust's BTreeMap to an object. Can only have strings as keys.
type BTreeMap<V> = { [key: string]: V };