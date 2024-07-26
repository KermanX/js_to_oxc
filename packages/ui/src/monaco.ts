import EditorWorker from 'monaco-editor-core/esm/vs/editor/editor.worker.js?worker'
import * as monaco from 'monaco-editor-core'

// @ts-expect-error missing types
window.MonacoEnvironment = {
  getWorker() {
    return new EditorWorker()
  },
}

monaco.languages.register({ id: 'javascript' })
monaco.languages.register({ id: 'rust' })
