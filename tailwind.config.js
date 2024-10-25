/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
      "./shared/js/*.js",
      "./themes/**/**/*.{js,html}"
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('daisyui'),
  ],
}

