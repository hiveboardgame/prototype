const withTM = require('next-transpile-modules')(['hive-db']);

module.exports = withTM({
  reactStrictMode: true,
});
