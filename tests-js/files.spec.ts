import safe from './util';

describe('files', () => {
    test('container create', async () => {
        const [a, b, c] = await safe.files_container_create(undefined, undefined, true, true, true);

        expect(a).not.toBeUndefined();
    });
});
