const assert = require('assert');

const { Safe, KeyPair } = require('../lib/index');

// const new_safe = () => {
//   let safe = new Safe();
//   const auth_credentials = "";
//   safe.connect("net.maidsafe.sn_nodejs", auth_credentials);
//   return safe;
// };

describe('EXPERIMENT  EXPERIMENT  EXPERIMENT', () => {
//   let safe = new_safe();

  test('TEST ALPHA', () => {
    // let keypair = safe.keypair();
    // expect(keypair.sk.length).toBe(64);
    // expect(keypair.pk.length).toBe(96);
    const kp = new KeyPair();
    const safe = new Safe();
    safe.connect2(kp);
    const balance = safe.keys_balance_from_sk2(kp);
    console.log(balance);
    // const key = safe.keys_create_preload_test_coins('10');
  });

});
