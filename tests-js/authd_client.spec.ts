/**
 * @jest-environment node
 */
// Override default environment so it will not connect to a Safe network.

import { SafeAuthdClient } from 'sn_nodejs';
import crypto from 'crypto';

describe('authd_client', () => {
    const phrase = 'passphrase-' + crypto.randomBytes(8).toString('hex');
    const word = 'password-' + crypto.randomBytes(8).toString('hex');

    test('constructor', () => {
        const CliAny = SafeAuthdClient as any;
        const endpoint = 'https://localhost:33000';

        // Correct
        expect(() => new CliAny()).not.toThrow();
        expect(() => new CliAny(undefined)).not.toThrow();
        expect(() => new CliAny(endpoint)).not.toThrow();

        // Incorrect
        expect(() => new CliAny(true)).toThrow();
        expect(() => new CliAny(12345)).toThrow();
    });

    test('getter', () => {
        const cli = new SafeAuthdClient();

        expect(cli).toHaveProperty('authd_endpoint');
        expect(typeof cli.authd_endpoint).toBe('string');
    });

    test('status', async () => {
        const cli = new SafeAuthdClient();
        const status = await cli.status();

        expect(status).toHaveProperty('safe_unlocked');
        expect(typeof status.safe_unlocked).toBe('boolean');

        expect(status).toHaveProperty('num_auth_reqs');
        expect(typeof status.num_auth_reqs).toBe('number');
    });

    test('create, lock and unlock', async () => {
        const cli = new SafeAuthdClient();
        await cli.create(phrase, word);
        await cli.lock();
        await cli.unlock(phrase, word);
        await cli.lock(); // Make sure we don't leave an unlocked Safe.
    });
});
