import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// Static build with seven HTML entries (home, kernel, and the documentation hub plus its four Diátaxis pages) and no backend. Tutorial illustrations
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
        documentation: 'documentation.html',
        howto: 'how-to.html',
        reference: 'reference.html',
        explanation: 'explanation.html',
      },
    },
  },
});
