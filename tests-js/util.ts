import { Safe } from 'sn_api';

// The environment sets a Safe instance as a global object.
declare var safe: Safe;

// (Re)export it so tests can import it from this module file.
export default safe;