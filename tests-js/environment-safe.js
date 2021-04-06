const Env = require('jest-environment-node');
const { Safe } = require('../sn_nodejs');
const dns = require('dns').promises;
const net = require('net');

class SafeEnv extends Env {
    async setup() {
        await super.setup();
        this.global.safe = new Safe(undefined, { secs: 120, nanos: 0 });

        // Allow a hardcoded bootstrap node to be passed to tests.
        let contact = process.env.SN_CONTACT;
        if (contact !== undefined) {
            // Check if the address provided contains a hostname. If so, resolve it.
            {
                const addr = contact.split(':');
                if (net.isIP(addr[0]) === 0) {
                    const ip = await dns.lookup(addr[0]); // DNS lookup.
                    contact = ip.address + ':' + addr[1];
                }
            }

            console.log('Connecting to node "%s"', contact);
            contact = [contact];
        }
        await this.global.safe.connect(undefined, undefined, contact);
    }

    async teardown() {
        await super.teardown();
    }

    runScript(script) {
        return super.runScript(script);
    }
}

module.exports = SafeEnv;