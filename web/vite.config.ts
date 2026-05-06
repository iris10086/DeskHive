import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  base: './',
  resolve: {
    alias: {
      '@': '/src'
    }
  },
  build: {
    outDir: '../deskhive_server/static',
    emptyOutDir: true
  },
  server: {
    port: 5174,
    proxy: {
      '/api': { target: 'http://127.0.0.1:8080', ws: true },
      '/health': { target: 'http://127.0.0.1:8080', ws: true },
      '/ws': { target: 'ws://127.0.0.1:8080', ws: true },
    }
  }
})
