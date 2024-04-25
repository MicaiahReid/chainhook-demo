/** @type {import('tailwindcss').Config} */
const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  content: ["./src/**/*.{html,js,ts,jsx,tsx}"],
  theme: {
    colors: {
      white: "#FCFCFD",
      red: "red",
      grey: "rgb(215 212 212)",
      black: "#0C0C0D",
    },
    extend: {
      fontFamily: {
        sans: [
          "'__openSauce_f4c502', '__openSauce_Fallback_f4c502'",
          ...defaultTheme.fontFamily.sans,
        ],
      },
    },
  },
  plugins: [],
};
