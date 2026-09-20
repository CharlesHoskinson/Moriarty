import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// Static build with two HTML entries and no backend. Every figure on either
// page comes from the typed modules under src/data. The relative base keeps
// assets resolvable when the site is served under a project subpath such as
// /Moriarty/, and it applies to kernel.html exactly as it does to index.html.
export default defineConfig({
  plugins: [react()],
  base: './',
  build: {
    outDir: 'dist',
    assetsInlineLimit: 8192,
    sourcemap: false,
    rollupOptions: {
      input: {
        index: 'index.html',
        kernel: 'kernel.html',
      },
    },
  },
});
