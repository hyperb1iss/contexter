import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import webExtension from 'vite-plugin-web-extension'
import path from 'path'

export default defineConfig({
  plugins: [
    react(),
    webExtension({
      browser: 'chrome',
      manifest: 'src/manifest.json'
    }),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  build: {
    outDir: 'dist',
    rollupOptions: {
      output: {
        chunkFileNames: 'chunks/[name].[hash].js',
        assetFileNames: 'assets/[name].[ext]'
      }
    }
  }
}) 