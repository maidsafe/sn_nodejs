var assert = require('assert');

const { new_safe } = require('./helpers.js');

describe.skip('fetch and parse URL APIs', function() {
  let safe = new_safe();

  it('Parse URL', function() {
    let filesContainer = safe.files_container_create("test/testfolder/", "", false, false);
    let filesContainerXorUrl = filesContainer[0];

    xorUrlEncoder = safe.parse_url(`safe://subname.${filesContainerXorUrl}/some/path?v=4`);
    console.log("AA ", xorUrlEncoder)
    //assert.equal(xorUrlEncoder.subnames, ['subname']);
  });

  it('Parse and resolve URL', function() {
    let filesContainer = safe.files_container_create("test/testfolder/", "", false, false);
    let filesContainerXorUrl = filesContainer[0];
    let random = Math.floor(Math.random() * Math.floor(1000));
    let nrsName = `safe://subname.pubname-${random}`;
    safe.nrs_map_container_create(nrsName, `${filesContainerXorUrl}?v=0`, true, false, false);

    xorUrlEncoder = safe.parse_and_resolve_url(`safe://subname.pubname-${random}/some/path?v=744`);
    //assert.equal(xorUrlEncoder.subnames, ['subname']);
  });
});
