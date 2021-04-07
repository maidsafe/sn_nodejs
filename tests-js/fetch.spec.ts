import safe from './util';

import fs from 'fs/promises';
import os from 'os';
import path from 'path';

describe('fetch', () => {
    let container_xor: string;

    beforeAll(async () => {
        // Create temporary directory containing file(s).
        const dir = await fs.mkdtemp(path.join(os.tmpdir(), 'sn_nodejs-'));
        await fs.writeFile(path.join(dir, 'index.txt'), 'index.txt content');

        [container_xor, ,] = await safe.files_container_create(dir + path.sep, undefined, true, true, false);
    });

    test('fetch FilesContainer', async () => {
        const r = await safe.fetch(container_xor, undefined);

        // Expect FilesContainer. TypeScript will know it's a FilesContainer after this statement.
        if (!('FilesContainer' in r)) {
            throw new Error('fetched XoR URL should be a FilesContainer');
        }

        expect('/index.txt' in r.FilesContainer.files_map).toBe(true);
    });

    test('fetch PublicBlob', async () => {
        const r = await safe.fetch(container_xor + '/index.txt', undefined);

        if (!('PublicBlob' in r)) {
            throw new Error('fetched XoR URL should be a PublicBlob');
        }

        expect(r.PublicBlob.media_type).toBe('text/plain');
    });
});
