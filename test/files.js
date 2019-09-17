var assert = require('assert');

const { new_safe } = require('./helpers.js');

describe('Files API', function() {
  let safe = new_safe();

  it('Create a FilesContainer and fetch it', function() {
    let filesContainer = safe.files_container_create("test/testfolder/", "", false, false);
    let processedFiles = filesContainer[1];
    let filesMap = filesContainer[2];
    assert.equal(Object.keys(processedFiles).length, 2);
    assert.equal(processedFiles['test/testfolder/test.txt'][1], filesMap['/test.txt'].link);

    let fetched = safe.fetch(filesContainer[0]);
    assert.equal(fetched.FilesContainer.files_map['/test.txt'].link, filesMap['/test.txt'].link);

    let fetchedFile = safe.fetch(`${filesContainer[0]}/test.txt`);
    assert.equal("hello test.txt!\n", String.fromCharCode.apply(null, new Uint8Array(fetchedFile.PublishedImmutableData.data)));
  });

  it('Get a FilesContainer', function() {
    let filesContainer = safe.files_container_create("test/testfolder/", "", false, false);
    let filesMap = filesContainer[2];

    let filesContainerData = safe.files_container_get(filesContainer[0]);
    assert.equal(0, filesContainerData[0]); //version should be 0;
    assert.equal(filesContainerData[1]['/test.txt'].link, filesMap['/test.txt'].link);
  });

  it('Sync a FilesContainer', function() {
    let filesContainer = safe.files_container_create("test/testfolder/", "", false, false);
    let filesMap = filesContainer[2];

    let filesContainerData = safe.files_container_sync("test/testfolder/test.txt", `${filesContainer[0]}/new-files.js`, false, false, false, false);
    let newFilesMap = filesContainerData[2];
    assert.equal(Object.keys(newFilesMap).length, 3);

    filesContainerData = safe.files_container_get(filesContainer[0]);
    assert.equal(filesContainerData[1]['/new-files.js'].link, newFilesMap['/new-files.js'].link);
  });

  it('Add a file to FilesContainer from Buffer and get it', function() {
    let filesContainer = safe.files_container_create("test/testfolder/", "", false, false);
    let filesMap = filesContainer[2];

    let rawBytes = Buffer.from("bytes-of-file");
    let filesContainerData = safe.files_container_add_from_raw(rawBytes, `${filesContainer[0]}/from-raw.txt`, false, false, false);
    let newFilesMap = filesContainerData[2];
    assert.equal(Object.keys(newFilesMap).length, 3);

    filesContainerData = safe.files_container_get(filesContainer[0]);
    assert.equal(filesContainerData[1]['/from-raw.txt'].link, newFilesMap['/from-raw.txt'].link);

    let fetchedFile = safe.fetch(newFilesMap['/from-raw.txt'].link);
    assert.equal(rawBytes.toString(), String.fromCharCode.apply(null, new Uint8Array(fetchedFile.PublishedImmutableData.data)));
  });

  it('Add a file to FilesContainer from Uint8Array and get it', function() {
    let filesContainer = safe.files_container_create("test/testfolder/", "", false, false);
    let filesMap = filesContainer[2];

    let rawBytes = Uint8Array.from([62, 79, 74, 65, 73]);
    let filesContainerData = safe.files_container_add_from_raw(rawBytes, `${filesContainer[0]}/from-raw.txt`, false, false, false);
    let newFilesMap = filesContainerData[2];
    assert.equal(Object.keys(newFilesMap).length, 3);

    filesContainerData = safe.files_container_get(filesContainer[0]);
    assert.equal(filesContainerData[1]['/from-raw.txt'].link, newFilesMap['/from-raw.txt'].link);

    let fetchedFile = safe.fetch(newFilesMap['/from-raw.txt'].link);
    assert.equal(rawBytes.toString(), new Uint8Array(fetchedFile.PublishedImmutableData.data).toString());
  });

  it('Put a PublishedImmutableData from Buffer and get it', function() {
    let rawBytes = Buffer.from("bytes-of-file");
    let immdUrl = safe.files_put_published_immutable(rawBytes);

    fetchedFile = safe.files_get_published_immutable(immdUrl);
    assert.equal(rawBytes.toString(), String.fromCharCode.apply(null, new Uint8Array(fetchedFile)));
  });

  it('Put a PublishedImmutableData from Uint8Array and get it', function() {
    let rawBytes = Uint8Array.from([62, 79, 74, 65, 73]);
    let immdUrl = safe.files_put_published_immutable(rawBytes);

    fetchedFile = safe.files_get_published_immutable(immdUrl);
    assert.equal(rawBytes.toString(), new Uint8Array(fetchedFile).toString());
  });
});
