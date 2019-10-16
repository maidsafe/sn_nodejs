const assert = require('assert');
const { new_safe } = require('./helpers.js');
const { SafeAuthdClient } = require('../lib/index');

describe.only('Authd Client API', () => {
  let safe_authd_client = new SafeAuthdClient(); // use default port number
  const secret = `random-secret-${Math.floor(Math.random() * Math.floor(1000))}`;
  const password = `random-password-${Math.floor(Math.random() * Math.floor(1000))}`;
  let safe = new_safe();
  const sk = safe.keys_create_preload_test_coins("10")[1].sk;
  safe_authd_client.start("/opt/safe/safe-authd");

  test('Create account', () => {
    safe_authd_client.create_acc(sk, secret, password);
  });

  test('Log in', () => {
    safe_authd_client.log_in(secret, password);
  });

  test.skip('Log out', () => {
    safe_authd_client.log_out();
  });

  test('Subscribe', () => {
    safe_authd_client.subscribe("https://localhost:33001", (appId) => {
      console.log("Allow safe-nodejs TEST app?:", appId); return true;
    });
  });
});
