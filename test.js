const { Safe } = require('./');

(async () => {
    const safe = new Safe();
    // Blocking
    safe.connect(undefined, undefined, ['127.0.0.1:42828']);

    // Async
    const [str, kp] = await safe.keys_create_preload_test_coins_async('9');
    console.log(str); // 'safe://<..>'
    console.log(kp); // 'Keypair::Ed25519(..)'
})().catch(r => console.dir(r));
