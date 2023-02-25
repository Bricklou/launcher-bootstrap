/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,ts,tsx,css}", "./index.html"],
  theme: {
    extend: {},
  },
  /** @type {import('daisyui').config} */
  daisyui: {
    themes: ["winter"],
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
};
