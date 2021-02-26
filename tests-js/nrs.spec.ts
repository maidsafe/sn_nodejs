import safe from './util';

import crypto from 'crypto';

describe('nrs', () => {
    let xor_files: string;

    beforeAll(async () => {
        [xor_files, ,] = await safe.files_container_create(undefined, undefined, true, true, false);
    });

    test('create and get', async () => {
        const nrs_rand_str = 'nrs-rand-' + crypto.randomBytes(8).toString('hex');

        const [xor_nrs, entries, map_nrs] = await safe.nrs_map_container_create(nrs_rand_str, xor_files + '?v=0', true, false, false);
        const [version, map_nrs_get] = await safe.nrs_map_container_get(xor_nrs);

        expect(version == 0).toBe(true);
        expect(map_nrs).toEqual(map_nrs_get);

        // Type should be (Rust Enum) DefaultRdf::OtherRdf(..)
        if (typeof map_nrs_get.default !== 'object' || !('OtherRdf' in map_nrs_get.default)) {
            throw new Error('type should be OtherRdf');
        }

    });

    test('add subname', async () => {
        const nrs_rand_str = 'nrs-rand-' + crypto.randomBytes(8).toString('hex');

        const [xor_nrs, entries, map_nrs] = await safe.nrs_map_container_create(nrs_rand_str, xor_files + '?v=0', true, false, false);

        const subname = 'sub-name';
        const nrs_subname_str = subname + '.' + nrs_rand_str;
        const [sub_version, sub_xor, , sub_map] = await safe.nrs_map_container_add(nrs_subname_str, xor_files + '?v=1', false, false, false);
        const [version, map_nrs_get] = await safe.nrs_map_container_get(xor_nrs);

        expect(subname in map_nrs_get.sub_names_map).toBe(true);
    });
});
