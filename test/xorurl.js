const assert = require('assert');

const { new_safe } = require('./helpers.js');
const { SafeDataType, SafeContentType, XorUrlEncoder } = require('../lib/index');

describe('XorUrlEncoder API', function() {
  let safe = new_safe();

  it('Instantiate a XorUrlEncoder', function() {
    let random = Math.floor(Math.random() * Math.floor(1000));
    let xorname = Uint8Array.from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1]);
    let subNames = ['sub1', 'sub2'];
    let xorUrlEncoder = new XorUrlEncoder(xorname, 10000, SafeDataType.PublishedSeqAppendOnlyData,
                                          SafeContentType.FilesContainer, "/folder", subNames, 5);
    assert.equal(xorUrlEncoder.encoding_version(), 1);
    assert.equal(xorUrlEncoder.xorname().toString(), xorname.toString());
    assert.equal(xorUrlEncoder.type_tag(), 10000);
    assert.equal(xorUrlEncoder.data_type(), SafeDataType.PublishedSeqAppendOnlyData);
    assert.equal(xorUrlEncoder.content_type(), SafeContentType.FilesContainer);
    assert.equal(xorUrlEncoder.path(), "/folder");
    assert.deepEqual(xorUrlEncoder.sub_names(), subNames);
    assert.equal(xorUrlEncoder.content_version(), 5);
    assert(xorUrlEncoder.to_string().startsWith("safe://sub1.sub2."));
    assert(xorUrlEncoder.to_string().endsWith("/folder?v=5"));
    assert(xorUrlEncoder.to_base("base64").startsWith("safe://sub1.sub2."));
    assert(xorUrlEncoder.to_base("base64").endsWith("/folder?v=5"));

    xorUrlEncoder.set_path("newpath");
    assert.equal(xorUrlEncoder.path(), "/newpath");
    xorUrlEncoder.set_content_version(888);
    assert.equal(xorUrlEncoder.content_version(), 888);
    assert(xorUrlEncoder.to_string().startsWith("safe://sub1.sub2."));
    assert(xorUrlEncoder.to_string().endsWith("/newpath?v=888"));
  });

  it('Encode XorUrl with MediaType', function() {
    let random = Math.floor(Math.random() * Math.floor(1000));
    let xorname = Uint8Array.from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1]);
    let subNames = ['sub1', 'sub2'];
    let xorUrlEncoder = new XorUrlEncoder(xorname, 10000, SafeDataType.PublishedImmutableData,
                                          "text/html", "/folder", subNames, 5);
    assert.equal(xorUrlEncoder.data_type(), SafeDataType.PublishedImmutableData);
    assert.equal(xorUrlEncoder.content_type(), "text/html");
  });
});
