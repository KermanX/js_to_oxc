import { ref } from 'vue'
import iframeSrc from './formatter.html?url'

const start = `impl _ {\n    fn create_ast(&self) {`
const end = `    }\n}`

const formatter: HTMLIFrameElement = document.createElement('iframe')
formatter.src = iframeSrc
formatter.style.display = 'none'
document.body.appendChild(formatter)

export const formatterReady = ref(false)

formatter.onload = () => {
  if (!formatter.contentWindow)
    throw new Error('Failed to load formatter')
  // @ts-expect-error missing types
  formatter.contentWindow.wasmLoader.then(() => {
    formatterReady.value = true
  }).catch(() => {
    throw new Error('Failed to load formatter')
  })
}

export function formatRust(source: string) {
  // @ts-expect-error missing types
  const result = formatter.contentWindow.formatRustCode(start + source + end)
  if (result.error)
    throw new Error(result.error)
  const code: string = result.success.slice(start.length, -end.length)
  return `${code.replaceAll(/^ {8}( *)/gm, (_, all) => all.slice(all.length / 2)).trim()}\n`
}
