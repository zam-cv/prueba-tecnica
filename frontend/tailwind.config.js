/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'background': '#0b121f',
        'background-light': '#293447',
        'background-dark': '#182232',
        'primary': '#d2292b',
        'secondary': '#8899b6',
        'opacity': '#ffffff14',
      },
    },
  },
  plugins: [],
}