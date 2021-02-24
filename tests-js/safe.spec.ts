/**
 * @jest-environment node
 */
// Override default environment so it will not connect to a Safe network.

import { Safe } from 'sn_api';

describe('safe', () => {
    test('safe constructor', () => {
        const SafeAny = Safe as any;
        const timeout = { secs: 120, nanos: 0 };

        // Correct
        expect(() => new SafeAny(undefined, timeout)).not.toThrow();
        expect(() => new SafeAny('base32z', timeout)).not.toThrow();
        expect(() => new SafeAny('base32', timeout)).not.toThrow();

        // Incorrect
        expect(() => new SafeAny()).toThrow();
        expect(() => new SafeAny(undefined)).toThrow();
        expect(() => new SafeAny('<wrong>', timeout)).toThrow();
        expect(() => new SafeAny(undefined, { secs: 120 })).toThrow();
        expect(() => new SafeAny(123, timeout)).toThrow();
    });
});