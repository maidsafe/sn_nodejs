const assert = require('assert');

const { new_safe } = require('./helpers.js');
const { SafeDataType, SafeContentType, XorUrlEncoder } = require('../lib/index');

describe('XorUrlEncoder API', () => {
  let safe = new_safe();

  test('Instantiate a XorUrlEncoder', () => {
    let random = Math.floor(Math.random() * Math.floor(1000));
    let xorname = Uint8Array.from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1]);
    let subNames = ['sub1', 'sub2'];
    let xorUrlEncoder = new XorUrlEncoder(xorname, null, 10000, SafeDataType.PublicSequence,
                                          SafeContentType.FilesContainer, "/folder", subNames, "q=0", "fragment", 5);
    assert.equal(xorUrlEncoder.encoding_version(), 1);
    assert.equal(xorUrlEncoder.xorname().toString(), xorname.toString());
    assert.equal(xorUrlEncoder.type_tag(), 10000);
    assert.equal(xorUrlEncoder.data_type(), SafeDataType.PublicSequence);
    assert.equal(xorUrlEncoder.content_type(), SafeContentType.FilesContainer);
    assert.equal(xorUrlEncoder.path(), "/folder");
    assert.deepEqual(xorUrlEncoder.sub_names(), subNames);
    assert.equal(xorUrlEncoder.content_version(), 5);
    assert(xorUrlEncoder.to_string().startsWith("safe://sub1.sub2."));
    assert(xorUrlEncoder.to_string().endsWith("/folder?q=0&v=5#fragment"));
    assert(xorUrlEncoder.to_base("base64").startsWith("safe://sub1.sub2."));
    assert(xorUrlEncoder.to_base("base64").endsWith("/folder?q=0&v=5#fragment"));

    xorUrlEncoder.set_path("newpath");
    assert.equal(xorUrlEncoder.path(), "/newpath");
    xorUrlEncoder.set_content_version(888);
    assert.equal(xorUrlEncoder.content_version(), 888);
    assert(xorUrlEncoder.to_string().startsWith("safe://sub1.sub2."));
    assert(xorUrlEncoder.to_string().endsWith("/newpath?q=0&v=888#fragment"));
  });

  test('Encode XorUrl with MediaType', () => {
    let random = Math.floor(Math.random() * Math.floor(1000));
    let xorname = Uint8Array.from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1]);
    let subNames = ['sub1', 'sub2'];
    let xorUrlEncoder = new XorUrlEncoder(xorname, null, 10000, SafeDataType.PublicBlob,
                                          "text/html", "/folder", subNames, null, null, 5);
    assert.equal(xorUrlEncoder.data_type(), SafeDataType.PublicBlob);
    assert.equal(xorUrlEncoder.content_type(), "text/html");
  });
});
