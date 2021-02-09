const assert = require('assert');

const { new_safe } = require('./helpers.js');

describe('Files API', () => {
  let safe = new_safe();

  test('Create a FilesContainer and fetch it', () => {
    let filesContainer = safe.files_container_create("test/testfolder/", "", false, false, false);
    let processedFiles = filesContainer[1];
    let filesMap = filesContainer[2];
    assert.equal(Object.keys(processedFiles).length, 2);
    assert.equal(processedFiles['test/testfolder/test.txt'][1], filesMap['/test.txt'].link);

    let fetched = safe.fetch(filesContainer[0]);
    assert.equal(fetched.FilesContainer.files_map['/test.txt'].link, filesMap['/test.txt'].link);

    let fetchedFile = safe.fetch(`${filesContainer[0]}/test.txt`);
    assert(String.fromCharCode.apply(null, new Uint8Array(fetchedFile.PublicBlob.data)).startsWith("hello test.txt!"));
  });

  test('Get a FilesContainer', () => {
    let filesContainer = safe.files_container_create("test/testfolder/", "", false, false, false);
    let filesMap = filesContainer[2];

    let filesContainerData = safe.files_container_get(filesContainer[0]);
    assert.equal(0, filesContainerData[0]); //version should be 0;
    assert.equal(filesContainerData[1]['/test.txt'].link, filesMap['/test.txt'].link);
  });

  test('Sync a FilesContainer', () => {
    let filesContainer = safe.files_container_create("test/testfolder/", "", false, false, false);
    let filesMap = filesContainer[2];

    let filesContainerData = safe.files_container_sync("test/testfolder/test.txt", `${filesContainer[0]}/new-files.js`, false, false, false, false, false);
    let newFilesMap = filesContainerData[2];
    assert.equal(Object.keys(newFilesMap).length, 3);

    filesContainerData = safe.files_container_get(filesContainer[0]);
    assert.equal(filesContainerData[1]['/new-files.js'].link, newFilesMap['/new-files.js'].link);
  });

  test('Add a file to FilesContainer from Buffer and get it', () => {
    let filesContainer = safe.files_container_create("test/testfolder/", "", false, false, false);
    let filesMap = filesContainer[2];

    let rawBytes = Buffer.from("bytes-of-file");
    let filesContainerData = safe.files_container_add_from_raw(rawBytes, `${filesContainer[0]}/from-raw.txt`, false, false, false);
    let newFilesMap = filesContainerData[2];
    assert.equal(Object.keys(newFilesMap).length, 3);

    filesContainerData = safe.files_container_get(filesContainer[0]);
    assert.equal(filesContainerData[1]['/from-raw.txt'].link, newFilesMap['/from-raw.txt'].link);

    let fetchedFile = safe.fetch(newFilesMap['/from-raw.txt'].link);
    assert.equal(rawBytes.toString(), String.fromCharCode.apply(null, new Uint8Array(fetchedFile.PublicBlob.data)));
  });

  test('Add a file to FilesContainer from Uint8Array and get it', () => {
    let filesContainer = safe.files_container_create("test/testfolder/", "", false, false, false);
    let filesMap = filesContainer[2];

    let rawBytes = Uint8Array.from([62, 79, 74, 65, 73]);
    let filesContainerData = safe.files_container_add_from_raw(rawBytes, `${filesContainer[0]}/from-raw.txt`, false, false, false);
    let newFilesMap = filesContainerData[2];
    assert.equal(Object.keys(newFilesMap).length, 3);

    filesContainerData = safe.files_container_get(filesContainer[0]);
    assert.equal(filesContainerData[1]['/from-raw.txt'].link, newFilesMap['/from-raw.txt'].link);

    let fetchedFile = safe.fetch(newFilesMap['/from-raw.txt'].link);
    assert.equal(rawBytes.toString(), new Uint8Array(fetchedFile.PublicBlob.data).toString());
  });

  test('Remove a file from a FilesContainer', () => {
    let filesContainer = safe.files_container_create("test/testfolder/", "", false, false, false);
    let filesMap = filesContainer[2];
    assert.equal(Object.keys(filesMap).length, 2);
    let file_removed = `${filesContainer[0]}/test.txt`;

    let filesContainerData = safe.files_container_remove_path(file_removed, false, false, false);
    let newFilesMap = filesContainerData[2];
    assert.equal(Object.keys(newFilesMap).length, 1);

    filesContainerData = safe.files_container_get(filesContainer[0]);
    assert.equal(filesContainerData[1]['/test.md'].link, newFilesMap['/test.md'].link);

    try {
      let fetchedFile = safe.fetch(file_removed);
    } catch(err) {
      assert(err.message.includes("No data found for path \\\"/test.txt/\\\""));
    }
  });

  test('Put a PublicBlob from Buffer and get it', () => {
    let rawBytes = Buffer.from("bytes-of-file");
    let immdUrl = safe.files_store_public_blob(rawBytes, null, false);

    fetchedFile = safe.files_get_public_blob(immdUrl);
    assert.equal(rawBytes.toString(), String.fromCharCode.apply(null, new Uint8Array(fetchedFile)));
  });

  test('Put a PublicBlob from Uint8Array and get it', () => {
    let rawBytes = Uint8Array.from([62, 79, 74, 65, 73]);
    let immdUrl = safe.files_store_public_blob(rawBytes, null, false);

    fetchedFile = safe.files_get_public_blob(immdUrl);
    assert.equal(rawBytes.toString(), new Uint8Array(fetchedFile).toString());
  });
});
