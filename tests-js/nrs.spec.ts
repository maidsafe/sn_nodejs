import safe from './util';

import crypto from 'crypto';

describe('nrs', () => {
    test('map container create', async () => {
        const nrs_rand_str = 'random_nrs-' + crypto.randomBytes(8).toString('hex');

        const [files_map_xor,,] = await safe.files_container_create(undefined, undefined, true, true, false);
        const [nrs_map_xor,,] = await safe.nrs_map_container_create(nrs_rand_str, files_map_xor + '?v=0', true, false, false);
        const [version, b] = await safe.nrs_map_container_get(nrs_map_xor);

        expect(nrs_map_xor).not.toBeUndefined();
    });
});
