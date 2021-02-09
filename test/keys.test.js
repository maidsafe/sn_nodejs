const assert = require('assert');

const { new_safe } = require('./helpers.js');

describe('Keys API', () => {
  let safe = new_safe();

  test('Create a key pair', () => {
    let keypair = safe.keypair();
    expect(keypair.sk.length).toBe(64);
    expect(keypair.pk.length).toBe(96);
  });

  test('Create a SafeKey with testcoins', () => {
    let safeKeyData = safe.keys_create_preload_test_coins("5");
    expect(safeKeyData[1].sk.length).toBe(64);
    expect(safeKeyData[1].pk.length).toBe(96);
    let xorurl = safeKeyData[0];
    let fetched = safe.fetch(xorurl);
    expect(fetched.SafeKey.xorname.length).toBe(32);
  });

  test('Create a SafeKey', () => {
    let safeKeyData = safe.keys_create_preload_test_coins("5");
    let newSafeKeyData = safe.keys_create_and_preload_from_sk_string(safeKeyData[1].sk, "2");
    let xorurl = newSafeKeyData[0];
    let fetched = safe.fetch(xorurl);
    expect(fetched.SafeKey.xorname.length).toBe(32);
  });

  test('Check coins balance of SafeKey from secret key', () => {
    let safeKeyData = safe.keys_create_preload_test_coins("9.8");
    let balance = safe.keys_balance_from_sk(safeKeyData[1].sk);
    expect(balance).toMatch("9.800000000");
  });

  test('Check coins balance of SafeKey from URL', () => {
    let safeKeyData = safe.keys_create_preload_test_coins("754.658");
    let balance = safe.keys_balance_from_url(safeKeyData[0], safeKeyData[1].sk);
    expect(balance).toMatch("754.658000000");
  });

  test('Validate sk for SafeKey URL', () => {
    let safeKeyData = safe.keys_create_preload_test_coins("343.455");
    let balance = safe.validate_sk_for_url(safeKeyData[1].sk, safeKeyData[0]);
    expect(balance).toMatch(safeKeyData[1].pk);
  });

  test('Transfer from a SafeKey', () => {
    let fromSafeKeyData = safe.keys_create_preload_test_coins("200");
    let toSafeKeyData = safe.keys_create_preload_test_coins("1");
    let txId = safe.keys_transfer("64", fromSafeKeyData[1].sk, toSafeKeyData[0], 123456789);
    expect(txId).toBe(123456789);
    let from_balance = safe.keys_balance_from_sk(fromSafeKeyData[1].sk);
    expect(from_balance).toMatch("136.000000000");
    let to_balance = safe.keys_balance_from_sk(toSafeKeyData[1].sk);
    expect(to_balance).toMatch("65.000000000");
  });
});
