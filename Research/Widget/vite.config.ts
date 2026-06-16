import { devtools } from '@tanstack/devtools-vite'
import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [devtools({ removeDevtoolsOnBuild: false }), react()],
})
