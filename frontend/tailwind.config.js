/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    screens: {
      sm: "320px",
      // => @media (min-width: 320px) { ... }

      md: "1024px",
      // => @media (min-width: 768px) { ... }

      lg: "1124px",
      // => @media (min-width: 1024px) { ... }

      xl: "1280px",
      // => @media (min-width: 1280px) { ... }

      "2xl": "1536px",
      // => @media (min-width: 1536px) { ... }
    },
    // screens: {
    //   sm: "320px",
    //   // => @media (min-width: 320px) { ... }

    //   md: "768px",
    //   // => @media (min-width: 768px) { ... }

    //   lg: "1024px",
    //   // => @media (min-width: 1024px) { ... }

    //   xl: "1280px",
    //   // => @media (min-width: 1280px) { ... }

    //   "2xl": "1536px",
    //   // => @media (min-width: 1536px) { ... }
    // },
    extend: {
      fontFamily: {
        oswald: ["Oswald", " sans-serif"],
        poppins: ["Poppins", "sans-serif"],
        Noto: ["Noto Sans Mono", " monospace"],
      },
      animation: {
        tilt: "tilt 10s infinite linear",
      },
      keyframes: {
        tilt: {
          "0%, 50%, 100%": {
            transform: "rotate(0deg)",
          },
          "25%": {
            transform: "rotate(0deg)",
          },
          "75%": {
            transform: "rotate(0 eg)",
          },
        },
        pulse: {
          "0%,  100%": {
            opacity: ".7",
          },
          "50%": {
            opacity: "1",
          },
        },
      },
    },
  },
  plugins: [],
};
