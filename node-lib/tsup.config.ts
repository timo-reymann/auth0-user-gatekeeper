import { defineConfig } from 'tsup';

export default defineConfig({
    entry: ['src/index.ts'],
    format: ['esm', 'cjs'], // Output formats
    dts: true, // Generate type declarations
    sourcemap: true, // Generate source maps
    clean: true, // Clean dist folder before build
    minify: true, // Minify output
    outDir: 'dist', // Output directory
    target: 'node18', // Node.js version target
    external: ['node-fetch'],
    minifySyntax: false,
    minifyWhitespace: false,
    minifyIdentifiers: false,
    cjsInterop: true,
});
