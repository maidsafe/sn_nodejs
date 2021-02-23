import safe from './util';

import { Safe, Keypair } from 'sn_api';

describe('keys', () => {
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
