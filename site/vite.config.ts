import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// Static build with three HTML entries and no backend. Tutorial illustrations
// are static assets; interactive examples use the typed data modules. The relative base keeps
// assets resolvable when the site is served under a project subpath such as
// /Moriarty/, and it applies to kernel.html and tutorial.html exactly as it does to index.html.
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
        tutorial: 'tutorial.html',
      },
    },
  },
});
