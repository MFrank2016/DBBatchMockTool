import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vite.dev/config/
export default defineConfig({
   // prevent vite from obscuring rust errors
   clearScreen: false,
   // Tauri expects a fixed port, fail if that port is not available
   server: {
     strictPort: true,
     port: 5173,
   },
   // to access the Tauri environment variables set by the CLI with information about the current target
   envPrefix: ['VITE_', 'TAURI_PLATFORM', 'TAURI_ARCH', 'TAURI_FAMILY', 'TAURI_PLATFORM_VERSION', 'TAURI_PLATFORM_TYPE', 'TAURI_DEBUG'],
   build: {
     // Tauri uses Chromium on Windows and WebKit on macOS and Linux
     target: ['es2021', 'chrome100', 'safari13'],
     // don't minify for debug builds
     minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
     // produce sourcemaps for debug builds
     sourcemap: !!process.env.TAURI_DEBUG,
   },
  plugins: [vue()],
})
