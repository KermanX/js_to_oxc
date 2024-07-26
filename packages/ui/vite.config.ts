import Unocss from 'unocss/vite'
import { defineConfig } from 'vite'
import Vue from '@vitejs/plugin-vue'
import Wasm from 'vite-plugin-wasm'

export default defineConfig({
  plugins: [Vue(), Unocss(), Wasm()],
  optimizeDeps: {
    exclude: [
      'js_to_oxc_wasm',
    ],
    include: [
      'monaco-editor-core/esm/vs/editor/editor.worker',
    ],
  },
  build: {
    target: 'esnext',
  },
})
