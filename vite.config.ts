import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
import compression from 'vite-plugin-compression'

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => ({
  plugins: [
    vue(),
    // Gzip 压缩 (阈值 10KB)
    compression({ algorithm: 'gzip', threshold: 10240 }),
    // Brotli 压缩 (阈值 10KB)
    compression({ algorithm: 'brotliCompress', threshold: 10240 }),
  ],
  resolve: {
    alias: {
      '@': resolve(__dirname, './src')
    }
  },
  // 鸿蒙构建模式使用相对路径（加载本地 rawfile 资源）
  base: mode === 'harmony' ? './' : '/',
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**']
    }
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
    // 鸿蒙构建模式输出到 harmony/dist 目录
    outDir: mode === 'harmony' ? 'harmony/dist' : 'dist',
    rollupOptions: {
      output: {
        manualChunks: {
          'three': ['three'],
          'monaco': ['monaco-editor'],
          'tiptap': ['@tiptap/vue-3', '@tiptap/starter-kit', '@tiptap/extension-placeholder'],
          'katex': ['katex'],
          'vendor': ['vue', 'vue-router', 'pinia'],
        }
      }
    }
  },
  optimizeDeps: {
    include: ['monaco-editor']
  }
}))
