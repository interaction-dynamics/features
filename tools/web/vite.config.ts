import path from 'node:path'
import tailwindcss from '@tailwindcss/vite'
import react from '@vitejs/plugin-react'
import { configDefaults, defineConfig } from 'vitest/config'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  base: process.env.BASE_URL ?? './',
  build: {
    rolldownOptions: {
      // it seems this property doesn't work yet
      external: ['**/features.json'],
    },
  },
  test: {
    exclude: [...configDefaults.exclude, 'e2e/*'],
  },
})
