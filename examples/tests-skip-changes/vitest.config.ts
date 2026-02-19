import { defineConfig } from 'vite'

export default defineConfig({
  test: {
    // ... other Vitest options
    coverage: {
      reporter: ['lcov', 'html'], // Other reporters can be included here
    },
  },
})
