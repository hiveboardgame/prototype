module.exports = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx}',
    './components/**/*.{js,ts,jsx,tsx}'
  ],
  theme: {
    fontFamily: {
      sans: [
        'InterVariable',
        'ui-sans-serif',
        'system-ui',
        '-apple-system',
        'BlinkMacSystemFont',
        '"Segoe UI"',
        'Roboto',
        '"Helvetica Neue"'
      ]
    },
    extend: {
      colors: {
        hive: '#f8a61c',
        hiveblack: '#333',
        hivewhite: 'white',
        ladybug: '#c32033',
        mosquito: '#605946',
        pillbug: '#00b3ad'
      }
    }
  },
  plugins: [require('@tailwindcss/typography')],
  important: true
};
