const { Safe, Keypair } = require('sn_api');

describe('keys', () => {
    const safe = new Safe();

    beforeAll(async () => {
        await safe.connect(undefined, undefined, ['127.0.0.1:12000']);
    });

    test('keypair instantiation has correct type', async () => {
        const kp = Keypair.new_ed25519();
        expect(kp.constructor).toBe(Keypair);
    });

    test('key preload and balance', async () => {
        const [str, kp] = await safe.keys_create_preload_test_coins('9');
        const balance = await safe.keys_balance_from_sk(kp.secret_key());

        expect(balance).toBe('9.000000000');
    });
});
