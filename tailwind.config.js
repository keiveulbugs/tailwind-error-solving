/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
  "./static/**/*.rs"],
  theme: {
    screens : {
      sm: '480px',
      md: '768px',
      lg: '976px',
      xl: '1440px'
    },


    extend: {
      colors: {
        transparent: 'transparent',
        current: 'currentColor',
        'beer-yellow': '#f8a90c',
        'beer-yellow-hover': '#b87e08',
      },
    },
  },
  plugins: [],
}
