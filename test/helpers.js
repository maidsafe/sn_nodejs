const { Safe } = require('../lib/index');

const new_safe = () => {
  // This requires the safe-nodejs lib to be built with scl-mock feature
  let safe = new Safe();
  safe.connect("net.maidsafe.safe-nodejs", "auth_credentials");
  return safe;
};

module.exports = {
  new_safe
};
