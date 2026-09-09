import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// Static single-page build. No backend, no runtime data fetching: every figure
// on the page comes from the typed modules under src/data.
export default defineConfig({
  plugins: [react()],
  base: './',
  build: { outDir: 'dist', assetsInlineLimit: 8192, sourcemap: false },
});
