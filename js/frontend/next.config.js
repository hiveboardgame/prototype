const withTM = require('next-transpile-modules')(['hive-db']);

module.exports = withTM({
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: 'http://127.0.0.1:8080/api/:path*',
      }
    ]
  },
  reactStrictMode: true,
});
