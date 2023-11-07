import {defineConfig} from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig(async () => ({
    plugins: [vue()],
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
    },
    envPrefix: ["VITE_", "TAURI_"],
    // Support tiny-vue
    define: {
        'process.env': {...process.env}
    },
    // Pass options to CSS-related loaders
    css: {
        preprocessorOptions: {
            less: {
                javascriptEnabled: true,
            }
        }
    }
}))
