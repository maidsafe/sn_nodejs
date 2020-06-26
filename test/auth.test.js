const assert = require('assert');
const { SafeAuthdClient } = require('../lib/index');
const { Safe } = require('../lib/index');

jest.setTimeout(120000);
const WAIT_TIME = 1000;
const delay = async ( time ) =>
    new Promise(
        ( resolve ) => setTimeout( resolve, time )
    );


describe('Authd Client API', () => {
  let safe_authd_client = new SafeAuthdClient(); // use default port number

  let safe = new Safe();
  let passphrase;
  let password;
  let sk;

  beforeAll( async () => {
    safe_authd_client.start();
    await delay(WAIT_TIME);
    passphrase = `random-passphrase-${Math.floor(Math.random() * Math.floor(1000))}`;
    password = `random-password-${Math.floor(Math.random() * Math.floor(1000))}`;
    sk = safe.keys_create_preload_test_coins("10")[1].sk;
    await delay(WAIT_TIME);
  })



  afterEach( async () => {
    await delay(WAIT_TIME);
  })

  afterAll( async () => {
    await safe_authd_client.unsubscribe(`https://localhost:${randomPort}`);
    await delay(WAIT_TIME);
    await safe_authd_client.stop();
    await delay(WAIT_TIME);

  })

  test('Create account', () => {
    expect( () =>  safe_authd_client.create_acc(sk, passphrase, password) ).not.toThrow() ;
  });

  test('Log out', () => {
    expect( () => safe_authd_client.log_out() ).not.toThrow() ;
  });

  test('Log in', () => {
    expect( () => safe_authd_client.log_in( passphrase, password) ).not.toThrow() ;
  });

  test('Status', () => {
    let status = safe_authd_client.status();
    assert.equal(status.logged_in, true);
    assert.equal(status.num_auth_reqs, 0);
    assert.equal(status.num_notif_subs, 0);
  });

  // test('Subscribe', async () => {
  //   let x = new Promise ( async (resolve, reject) => {
  //     const appId = "Jest.test.app.id";
  //     const randomPort = 33001 + Math.floor(Math.random() * Math.floor(1000));
  //     safe_authd_client.subscribe(`https://localhost:${randomPort}`, appId, (auppId, reqId) => {
  //       console.log("New auth req received:", appId);
  //       console.log("Allowing safe-nodejs TEST app:", reqId);
  //       let allow_out = safe_authd_client.allow(parseInt(reqId));
  //       console.log("Allowed? ", allow_out);
  //       resolve('fin');
  //     });

  //     let credentials = safe.auth_app(appId, "safe-nodejs Jest Test", "Maidsafe.net Ltd.");
  //     safe.connect(appId, credentials);

  //     await delay(30000);
  //     reject();

  //   } );

  //   await x;
  //   expect( x ).toEqual('fin');
  // })
});
