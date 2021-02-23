const Env = require('jest-environment-node');
const { Safe } = require('../sn_api');

class SafeEnv extends Env {
    async setup() {
        await super.setup();
        this.global.safe = new Safe();
        await this.global.safe.connect(undefined, undefined, ['127.0.0.1:12000']);
    }

    async teardown() {
        await super.teardown();
    }

    runScript(script) {
        return super.runScript(script);
    }
}

module.exports = SafeEnv;