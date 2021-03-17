const Env = require('jest-environment-node');
const { Safe } = require('../sn_nodejs');

class SafeEnv extends Env {
    async setup() {
        await super.setup();
        this.global.safe = new Safe(undefined, { secs: 120, nanos: 0 });

        const contact = process.env.SN_CONTACT || '127.0.0.1:12000';
        await this.global.safe.connect(undefined, undefined, [contact]);
    }

    async teardown() {
        await super.teardown();
    }

    runScript(script) {
        return super.runScript(script);
    }
}

module.exports = SafeEnv;