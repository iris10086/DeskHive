import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue({
      template: {
        compilerOptions: {
          // 解决Vite 6.x的HTML inline proxy问题
          isCustomElement: (tag) => tag.includes('-')
        }
      }
    })
  ],
  // 使用相对路径，确保在 Tauri 中正常工作
  base: './',
  build: {
    // 禁用CSS代码分割以避免inline proxy问题
    cssCodeSplit: false,
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'index.html'),
        settings: resolve(__dirname, 'settings.html')
      },
      output: {
        entryFileNames: 'assets/[name]-[hash].js',
        chunkFileNames: 'assets/[name]-[hash].js',
        assetFileNames: 'assets/[name]-[hash].[ext]'
      }
    }
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  },
  // 禁用HMR以更好地支持离线环境
  server: {
    hmr: false
  }
})