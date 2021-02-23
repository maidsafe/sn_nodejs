export class Safe {
    constructor();

    connect(a?: Keypair, b?: String, c?: String[]): Promise<void>;
    keys_create_preload_test_coins(a: String): Promise<[String, Keypair]>;
    keys_balance_from_sk(a: SecretKey): Promise<String>;
}

export class Keypair {
    static new_ed25519(): Keypair;

    secret_key(): SecretKey;
}
export class SecretKey { }