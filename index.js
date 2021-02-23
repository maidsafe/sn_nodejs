const { Safe } = require('./sn_api');

(async () => {
    const safe = new Safe();
    await safe.connect(undefined, undefined, ['127.0.0.1:12000']);
    const [a, b, c] = await safe.files_container_create("haha", undefined, true, true, true);

    console.log("end.");
})().catch(r => console.dir(r));
