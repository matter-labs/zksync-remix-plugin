import { defineConfig } from 'vite'
import path from 'path'
import react from '@vitejs/plugin-react'
import svgr from 'vite-plugin-svgr'
import checker from 'vite-plugin-checker'
import { execSync } from 'child_process'

export default defineConfig(() => {
  const commitHash = execSync('git rev-parse --short HEAD').toString().trimEnd();
  process.env.VITE_COMMIT_HASH = commitHash;

  return {
    define: {
      'process.env': {}
    },
    server: {
      port: 3000,
      hmr: true
    },
    build: {
      outDir: 'build'
    },
    plugins: [
      react(),
      svgr({ svgrOptions: { icon: true } }),
      checker({
        typescript: true,
        eslint: {
          lintCommand: 'eslint "./src/**/*.{ts,tsx}"' // for example, lint .ts & .tsx
        }
      })
    ],
    resolve: {
      alias: {
        '@/': `${path.resolve(__dirname, 'src')}/`
      }
    }
  }
})
