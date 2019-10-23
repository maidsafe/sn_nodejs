const assert = require('assert');
const { new_safe } = require('./helpers.js');
const { SafeAuthdClient } = require('../lib/index');

describe.skip('Authd Client API', () => {
  let safe_authd_client = new SafeAuthdClient(); // use default port number
  const secret = `random-secret-${Math.floor(Math.random() * Math.floor(1000))}`;
  const password = `random-password-${Math.floor(Math.random() * Math.floor(1000))}`;
  let safe = new_safe();
  const sk = safe.keys_create_preload_test_coins("10")[1].sk;
  safe_authd_client.stop("/home/bochaco/safe/bochaco-safe-cli/target/debug/safe-authd");
  safe_authd_client.start("/home/bochaco/safe/bochaco-safe-cli/target/debug/safe-authd");

  test('Create account', () => {
    safe_authd_client.create_acc(sk, secret, password);
  });

  test('Log in', () => {
    safe_authd_client.log_in(secret, password);
  });

  test('Subscribe', (done) => {
    safe_authd_client.subscribe("https://localhost:33001", (appId, reqId) => {
      console.log("New auth req with ID:", reqId);
      console.log("Allow safe-nodejs TEST app?:", appId);
    });

    console.log("AUTHORISING...");
    const appId = "Jest.test.app.id";
    let credentials = safe.auth_app(appId, "safe-nodejs Jest Test", "Maidsafe.net Ltd.");
    console.log("AUTHORISED:", credentials);
    return new Promise(resolve => {
      safe.connect(appId, credentials);
      console.log("CONNECTED!");
      done();
    });
  });

  test.skip('Log out', () => {
    safe_authd_client.log_out();
  });
});
