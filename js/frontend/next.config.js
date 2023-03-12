const withTM = require('next-transpile-modules')(['hive-db']);

module.exports = withTM({
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: 'http://localhost:8080/api/:path*',
      }
    ]
  },
  reactStrictMode: true,
});
