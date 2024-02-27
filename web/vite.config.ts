import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react'
import {resolve} from "path"
// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react()],
    server: {
        proxy: {
            "/api": {
                target: 'http://127.0.0.1:3121',
                changeOrigin: true
            }
        }
    },
    resolve: {
        alias: {
            "@": resolve(__dirname, "./src"),
        }
    },
})

