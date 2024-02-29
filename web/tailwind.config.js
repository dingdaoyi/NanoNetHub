/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{vue,js,ts,jsx,tsx}", // 用到tailwind的地方
    ],
    theme: {
        container: {
            center: true,
        },
        extend: {},
    },
    plugins: [],
    corePlugins: {
        preflight: false,
    },
}

