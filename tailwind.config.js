/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./ui/**/*.{rs,html,css}", "./web/**/*.{rs,html,css}", '!./**/tailwind.css',],
  theme: {
    extend: {},
  },
  plugins: [
    require('daisyui'),
  ],
};
