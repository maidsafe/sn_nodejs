const assert = require('assert');

const { new_safe } = require('./helpers.js');

describe('fetch/inspect APIs', function() {
  let safe = new_safe();

  test('fetch FilesContainer', () => {
    let filesContainer = safe.files_container_create("test/testfolder/", "", false, false, false);
    let filesContainerXorUrl = filesContainer[0];
    let filesMap = filesContainer[2];

    let data = safe.fetch(`safe://${filesContainerXorUrl}`);
    assert.equal(data.FilesContainer.type_tag, 1100);
    assert.equal(data.FilesContainer.version, 0);
    assert.equal(data.FilesContainer.data_type, 'PublicSequence');
    assert.deepEqual(data.FilesContainer.files_map['/test.md'], filesMap['/test.md']);
    assert.deepEqual(data.FilesContainer.files_map['/test.txt'], filesMap['/test.txt']);
  });

  test('inspect FilesContainer', () => {
    let filesContainer = safe.files_container_create("test/testfolder/", "", false, false, false);
    let filesContainerXorUrl = filesContainer[0];
    let filesMap = filesContainer[2];

    let data = safe.inspect(`safe://${filesContainerXorUrl}`)[0];
    assert.equal(data.FilesContainer.type_tag, 1100);
    assert.equal(data.FilesContainer.version, 0);
    assert.equal(data.FilesContainer.data_type, 'PublicSequence');
    assert.deepEqual(data.FilesContainer.files_map['/test.md'], filesMap['/test.md']);
    assert.deepEqual(data.FilesContainer.files_map['/test.txt'], filesMap['/test.txt']);
  });

  test('fetch PublicImmutableData', () => {
    let filesContainer = safe.files_container_create("test/testfolder/test.txt", "", false, false, false);
    let filesContainerXorUrl = filesContainer[0];

    let data = safe.fetch(`safe://${filesContainerXorUrl}/test.txt`);

    assert.equal(data.PublicImmutableData.media_type, 'text/plain');
    assert(String.fromCharCode.apply(null, new Uint8Array(data.PublicImmutableData.data)).startsWith("hello test.txt!"));
  });

  test('fetch PublicImmutableData range', () => {
    let filesContainer = safe.files_container_create("test/testfolder/test.txt", "", false, false, false);
    let filesContainerXorUrl = filesContainer[0];

    let data = safe.fetch(`safe://${filesContainerXorUrl}/test.txt`, {start: 2, end: 5});

    assert.equal(String.fromCharCode.apply(null, new Uint8Array(data.PublicImmutableData.data)), "llo");
  });

  test('inspect PublicImmutableData', () => {
    let filesContainer = safe.files_container_create("test/testfolder/test.txt", "", false, false, false);
    let filesContainerXorUrl = filesContainer[0];

    let fetched = safe.fetch(`safe://${filesContainerXorUrl}/test.txt`);

    let data = safe.inspect(fetched.PublicImmutableData.xorurl)[0];
    assert.equal(data.PublicImmutableData.media_type, 'text/plain');
    assert.equal(data.PublicImmutableData.data, "");
  });
});
