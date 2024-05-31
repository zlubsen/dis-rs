/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./**/*.{html,js}"],
    plugins: [require('daisyui')],
    // plugins: [require("@tailwindcss/forms"), require("@tailwindcss/typography")],
    daisyui: {
        themes: ["light", "dark", "cupcake"],
        darkTheme: "dark",
        base: true
    },
};