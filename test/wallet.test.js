const assert = require('assert');

const { new_safe } = require('./helpers.js');

describe('Wallet API', () => {
  let safe = new_safe();

  test('Create a Wallet', () => {
    let walletXorUrl = safe.wallet_create();
    let fetched = safe.fetch(walletXorUrl);
    expect(fetched.Wallet.xorname.length).toBe(32);
  });

  test('Insert a SafeKey in a Wallet', () => {
    let safeKeyData = safe.keys_create_preload_test_coins("576");
    let walletXorUrl = safe.wallet_create();
    let name = safe.wallet_insert(walletXorUrl, "my-spendable-balance", true, safeKeyData[1].sk);
    expect(name).toMatch("my-spendable-balance");
  });

  test('Check balance of a Wallet', () => {
    let safeKeyData = safe.keys_create_preload_test_coins("941.706");
    let walletXorUrl = safe.wallet_create();
    let name = safe.wallet_insert(walletXorUrl, "my-spendable-balance", true, safeKeyData[1].sk);
    expect(name).toMatch("my-spendable-balance");
    let balance = safe.wallet_balance(walletXorUrl);
    expect(balance).toMatch("941.706000000");
  });

  test('Get default spendable balance from a Wallet', () => {
    let safeKeyData = safe.keys_create_preload_test_coins("941.706");
    let walletXorUrl = safe.wallet_create();
    let name = safe.wallet_insert(walletXorUrl, "my-default-spendable-balance", true, safeKeyData[1].sk);
    let defaultBalance = safe.wallet_get_default_balance(walletXorUrl);
    expect(defaultBalance[0].sk).toMatch(safeKeyData[1].sk);
    expect(defaultBalance[0].xorurl).toMatch(safeKeyData[0]);
  });

  test('Transfer from a Wallet', () => {
    let fromSafeKeyData = safe.keys_create_preload_test_coins("333");
    let walletXorUrl = safe.wallet_create();
    let name = safe.wallet_insert(walletXorUrl, "my-spendable-balance", true, fromSafeKeyData[1].sk);
    let toSafeKeyData = safe.keys_create_preload_test_coins("3");
    let txId = safe.wallet_transfer("30.000000001", walletXorUrl, toSafeKeyData[0], 987654321);
    expect(txId).toBe(987654321);
    let fromBalance = safe.wallet_balance(walletXorUrl);
    expect(fromBalance).toMatch("302.999999999");
    let toBalance = safe.keys_balance_from_sk(toSafeKeyData[1].sk);
    expect(toBalance).toMatch("33.000000001");
  });

  test('Get a Wallet', () => {
    let safeKeyData1 = safe.keys_create_preload_test_coins("1000.300");
    let safeKeyData2 = safe.keys_create_preload_test_coins("3000.500");
    let walletXorUrl = safe.wallet_create();
    safe.wallet_insert(walletXorUrl, "first-spendable-balance", false, safeKeyData1[1].sk);
    safe.wallet_insert(walletXorUrl, "second-spendable-balance", true, safeKeyData2[1].sk);
    let spendableBalances = safe.wallet_get(walletXorUrl);
    expect(spendableBalances['first-spendable-balance'][0]).toBe(false);
    expect(spendableBalances['first-spendable-balance'][1].sk).toMatch(safeKeyData1[1].sk);
    expect(spendableBalances['first-spendable-balance'][1].xorurl).toMatch(safeKeyData1[0]);
    expect(spendableBalances['second-spendable-balance'][0]).toBe(true);
    expect(spendableBalances['second-spendable-balance'][1].sk).toMatch(safeKeyData2[1].sk);
    expect(spendableBalances['second-spendable-balance'][1].xorurl).toMatch(safeKeyData2[0]);
    let totalBalance = safe.wallet_balance(walletXorUrl);
    expect(totalBalance).toMatch("4000.800000000");
  });
});
