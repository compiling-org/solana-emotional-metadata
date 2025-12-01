import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
    },
  },
  server: {
    port: 3000,
    host: '0.0.0.0',
  },
  build: {
    outDir: 'dist',
    sourcemap: true,
    rollupOptions: {
      external: [
        '@polkadot/x-globalThis',
        '@polkadot/x-randomvalues/browser',
        '@polkadot/util-crypto',
        '@polkadot/util',
        '@polkadot/keyring',
        '@polkadot/types',
        '@polkadot/api',
        '@polkadot/api-contract',
        '@polkadot/extension-dapp'
      ],
    },
  },
  define: {
    global: 'globalThis',
    'process.env': {},
  },
  optimizeDeps: {
    include: ['near-api-js'],
  },
})