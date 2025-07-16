import { defineConfig } from 'vitest/config';
import path from 'path';

export default defineConfig({
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './test/setup.js',
    include: ['test/**/*.test.js'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      exclude: [
        'node_modules/',
        'test/',
        '**/*.config.js',
        'dist/',
        'scripts/'
      ]
    }
  },
  resolve: {
    alias: {
      '@host': path.resolve(__dirname, './host/src'),
      '@remote': path.resolve(__dirname, './remote/src'),
      '@test': path.resolve(__dirname, './test')
    }
  }
});