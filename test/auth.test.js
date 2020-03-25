const assert = require('assert');
// const { new_safe } = require('./helpers.js');
const { SafeAuthdClient } = require('../lib/index');
const { Safe } = require('../lib/index');


describe('Authd Client API', async () => {
  let safe_authd_client = new SafeAuthdClient(); // use default port number
  
  let safe = new Safe();
  // safe.connect("net.maidsafe.safe-nodejs", auth_credentials);
  // return safe;
  // let safe = new_safe();

  const passphrase = `random-passphrase-${Math.floor(Math.random() * Math.floor(1000))}`;
  const password = `random-password-${Math.floor(Math.random() * Math.floor(1000))}`;
  const sk = safe.keys_create_preload_test_coins("10")[1].sk;

    //safe_authd_client.install("~/.safe/authd");
  await safe_authd_client.start();

  test('Create account', () => {
    safe_authd_client.create_acc(sk, passphrase, password);
  });

  test('Log out', () => {
    safe_authd_client.log_out();
  });

  test('Log in', () => {
    safe_authd_client.log_in(passphrase, password);
  });

  test('Status', () => {
    let status = safe_authd_client.status();
    assert.equal(status.logged_in, true);
    assert.equal(status.num_auth_reqs, 0);
    assert.equal(status.num_notif_subs, 0);
  });

  test('Subscribe', (done) => {
    const appId = "Jest.test.app.id";
    const randomPort = 33001 + Math.floor(Math.random() * Math.floor(1000));
    safe_authd_client.subscribe(`https://localhost:${randomPort}`, appId, (auppId, reqId) => {
      console.log("New auth req received:", appId);
      console.log("Allowing safe-nodejs TEST app:", reqId);
      let allow_out = safe_authd_client.allow(parseInt(reqId));
      console.log("Allowed? ", allow_out);
      done();
    });

    let credentials = safe.auth_app(appId, "safe-nodejs Jest Test", "Maidsafe.net Ltd.");
    safe.connect(appId, credentials);
  });
});
