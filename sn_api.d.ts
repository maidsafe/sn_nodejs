export class Safe {
    constructor(xor_url_base: 'base32z' | 'base32' | 'base64' | undefined, timeout: Duration);

    connect(app_keypair?: Keypair, config_path?: String, bootstrap_config?: String[]): Promise<void>;

    keys_create_preload_test_coins(a: String): Promise<[String, Keypair]>;
    keys_balance_from_sk(a: SecretKey): Promise<String>;

    files_container_create(location: undefined | String, dest: undefined | String, recursive: boolean, follow_links: boolean, dry_run: boolean): Promise<[String, ProcessedFiles, FilesMap]>;
}

interface Duration {
    secs: number,
    nanos: number,
}

// Returned by files_container_create().
interface ProcessedFiles {
    [path: string]: [String, String];
}
interface FilesMap {
    [path: string]: FilesMapDetails;
}
interface FilesMapDetails {
    [property: string]: string;
}

export class Keypair {
    static new_ed25519(): Keypair;

    secret_key(): SecretKey;
}
export class SecretKey { }