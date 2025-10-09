/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.rs", "./src/*.rs"],
  theme: {
    extend: {
      colors: {
        'storm-black': '#24283b',
        'storm-night': '#1a1b26',
        'storm-white': '#a9b1d6',
        'terminal-red': '#f7768e',
        'terminal-orange': '#ff9e64',
        'terminal-yellow': '#e0af68',
        'terminal-green': '#9ece6a',
        'terminal-azure': '#2ac3de',
        'terminal-purple': '#bb9af7',
      }
    }
  },
  darkMode: 'class',
  plugins: [],
}
