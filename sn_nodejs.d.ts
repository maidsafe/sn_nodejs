export class Safe {
    constructor(xor_url_base: 'base32z' | 'base32' | 'base64' | undefined, timeout: Duration);

    connect(app_keypair?: Keypair, config_path?: string, bootstrap_config?: string[]): Promise<void>;

    keys_create_preload_test_coins(a: string): Promise<[string, Keypair]>;
    keys_balance_from_sk(a: SecretKey): Promise<string>;

    files_container_create(location: undefined | string, dest: undefined | string, recursive: boolean, follow_links: boolean, dry_run: boolean): Promise<[string, ProcessedFiles, FilesMap]>;

    nrs_map_container_create(name: string, link: string, def: boolean, hard_link: boolean, dry_run: boolean): Promise<[string, ProcessedEntries, NrsMap]>;
    nrs_map_container_add(name: string, link: string, def: boolean, hard_link: boolean, dry_run: boolean): Promise<[number | BigInt, string, ProcessedEntries, NrsMap]>;
    nrs_map_container_get(xor: string): Promise<[number | BigInt, NrsMap]>;

    fetch(url: string, range?: Range): Promise<SafeData>;
}

// TODO: Verify (Option<u64>, Option<u64>) does indeed resolve into:
type Range = [BigInt | undefined, BigInt | undefined];
// Enum.
type SafeData = SafeKey | Wallet | FilesContainer | PublicBlob | NrsMapContainer | PublicSequence | PrivateSequence;

type XorName = Array<number>; // Not ideal, but this is what napi-rs generates for [u8].

type WalletSpendableBalance = { xorurl: XorUrl, sk: string };
type XorUrl = string;
type WalletSpendableBalances = BTreeMap<[boolean, WalletSpendableBalance]>;

type SafeDataType = 'SafeKey' | 'PublicBlob' | 'PrivateBlob' | 'PublicSequence' | 'PrivateSequence' | 'SeqMap' | 'UnseqMap';
type FileItem = BTreeMap<string>;

type SafeKey = {
    'SafeKey': {
        xorurl: string,
        xorname: XorName,
        resolved_from: string,
    }
};
type Wallet = {
    'Wallet': {
        xorurl: string,
        xorname: XorName,
        type_tag: BigInt,
        balances: WalletSpendableBalances,
        data_type: SafeDataType,
        resolved_from: string,
    }
};
type FilesContainer = {
    'FilesContainer': {
        xorurl: string,
        xorname: XorName,
        type_tag: BigInt,
        version: BigInt,
        files_map: FilesMap,
        data_type: SafeDataType,
        resolved_from: string,
    }
};
type PublicBlob = {
    'PublicBlob': {
        xorurl: string,
        xorname: XorName,
        data: Array<number>, // Vec<u8>
        media_type: string | undefined,
        metadata: FileItem | undefined,
        resolved_from: string,
    }
};
type NrsMapContainer = {
    'NrsMapContainer': {
        public_name: string | undefined,
        xorurl: string,
        xorname: XorName,
        type_tag: BigInt,
        version: BigInt,
        nrs_map: NrsMap,
        data_type: SafeDataType,
        resolved_from: string,
    }
};
type PublicSequence = {
    'PublicSequence': {
        xorurl: string,
        xorname: XorName,
        type_tag: BigInt,
        version: BigInt,
        data: Array<number>, // Vec<u8>
        resolved_from: string,
    }
};
type PrivateSequence = {
    'PrivateSequence': {
        xorurl: string,
        xorname: XorName,
        type_tag: BigInt,
        version: BigInt,
        data: Array<number>, // Vec<u8>
        resolved_from: string,
    }
};


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
    authd_endpoint: string;

    constructor(endpoint?: string);

    status(): Promise<AuthdStatus>;
    unlock(passphrase: string, password: string): Promise<void>;
    lock(): Promise<void>;
    create(passphrase: string, password: string): Promise<void>;
    authed_apps(): Promise<unknown>;
    revoke_app(app_id: string): Promise<void>;
    auth_reqs(): Promise<unknown>;
    allow(req_id: SafeAuthReqId): Promise<void>;
    deny(req_id: SafeAuthReqId): Promise<void>;
}
type AuthdStatus = {
    safe_unlocked: boolean,
    num_auth_reqs: number,
    num_notif_subs: number,
    authd_version: string,
}
type SafeAuthReqId = string;


// Map Rust's BTreeMap to an object. Can only have strings as keys.
type BTreeMap<V> = { [key: string]: V };