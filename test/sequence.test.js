const assert = require('assert');

const { new_safe } = require('./helpers.js');

describe('Sequence API', () => {
  let safe = new_safe();

  test('Store a Sequence from Buffer and get it', () => {
    let rawBytes = Buffer.from("bytes-of-sequence");
    let seqUrl = safe.sequence_create(rawBytes, null, 11000, false);

    fetchedSeq = safe.sequence_get(seqUrl);
    let fetchedVersion = fetchedSeq[0];
    let fetchedBytes = fetchedSeq[1];
    assert.equal(fetchedVersion, 0);
    assert.equal(rawBytes.toString(), String.fromCharCode.apply(null, new Uint8Array(fetchedBytes)));
  });

  test('Store a Private Sequence and get it', () => {
    let rawBytes = Buffer.from("private-bytes-of-sequence");
    let seqUrl = safe.sequence_create(rawBytes, null, 11000, /*private =*/true);

    fetchedSeq = safe.sequence_get(seqUrl);
    let fetchedVersion = fetchedSeq[0];
    let fetchedBytes = fetchedSeq[1];
    assert.equal(fetchedVersion, 0);
    assert.equal(rawBytes.toString(), String.fromCharCode.apply(null, new Uint8Array(fetchedBytes)));
  });

  test('Store a Sequence from Uint8Array and get it', () => {
    let rawBytes = Uint8Array.from([62, 79, 74, 65, 73, 0x2d, 0x6f, 66, 0x2d, 73, 65, 71, 75, 65, 0x6e, 63, 65]);
    let seqUrl = safe.sequence_create(rawBytes, null, 11000, false);

    fetchedSeq = safe.sequence_get(seqUrl);
    let fetchedVersion = fetchedSeq[0];
    let fetchedBytes = fetchedSeq[1];
    assert.equal(fetchedVersion, 0);
    assert.equal(rawBytes.toString(), new Uint8Array(fetchedBytes).toString());
  });

  test('Append to a Sequence', () => {
    let rawBytes = Uint8Array.from([62, 79, 74, 65, 73, 0x2d, 0x6f, 66, 0x2d, 73, 65, 71, 75, 65, 0x6e, 63, 65]);
    let seqUrl = safe.sequence_create(rawBytes, null, 11000, false);

    let appenddedBytes = Uint8Array.from([70, 80, 90, 100]);
    safe.append_to_sequence(seqUrl, appenddedBytes);

    fetchedSeq = safe.sequence_get(seqUrl);
    let fetchedVersion = fetchedSeq[0];
    let fetchedBytes = fetchedSeq[1];
    assert.equal(fetchedVersion, 1);
    assert.equal(appenddedBytes.toString(), new Uint8Array(fetchedBytes).toString());
  });
});
