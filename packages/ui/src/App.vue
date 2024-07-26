<script setup lang="ts">
import { debouncedWatch, useLocalStorage } from '@vueuse/core'
import { run as jsToOxc } from 'js_to_oxc_wasm'
import { ref, shallowRef } from 'vue'
import Editor from './Editor.vue'
import { formatRust } from './format'

const js = ref('console.log("Hello, World!")')
const autoRun = useLocalStorage('js_to_oxc:autoRun', true)
const shouldFormat = useLocalStorage('js_to_oxc:shouldFormat', true)

const formatted = ref('')
const loading = ref(true)
const error = ref('')
const controller = shallowRef<AbortController>()

async function run() {
  const unformatted = `impl _ {
    fn create_ast(&self) {
      ${jsToOxc(js.value)}
    }
  }`
  controller.value?.abort()
  const { signal } = controller.value = new AbortController()
  try {
    error.value = ''
    loading.value = true
    formatted.value = shouldFormat.value ? await formatRust(unformatted, signal) : unformatted
    loading.value = false
  }
  catch (err: any) {
    if (err instanceof DOMException && err.name === 'AbortError')
      return
    error.value = String(err)
    formatted.value = unformatted
    loading.value = false
  }
}

debouncedWatch(() => [autoRun.value && js.value, shouldFormat.value], ([source]) => {
  if (source)
    run()
}, { immediate: true, debounce: 300 })
</script>

<template>
  <div p-4 fixed inset-0 flex flex-col>
    <div flex>
      <h1 text-3xl font-bold pb-4>
        JS to Oxc
      </h1>
      <div flex-grow />
      <div flex flex-col h-0>
        <div>
          Options
        </div>
        <div flex-grow text-sm>
          <label flex align-center gap-1>
            <input v-model="autoRun" type="checkbox">
            Auto Run
          </label>
          <label flex align-center gap-1>
            <input v-model="shouldFormat" type="checkbox">
            Format Rust
          </label>
        </div>
      </div>
    </div>
    <div flex-grow grid grid-cols-2 gap-6>
      <div flex flex-col>
        <div flex items-center>
          <h2 text-2xl pb-2 pl-2>
            Input JS
          </h2>
          <div flex-grow />
          <div>
            <button bg-gray-400 bg-op-40 px-3 rounded hover:bg-op-50 @click="run">
              Run
            </button>
          </div>
        </div>
        <Editor v-model="js" lang="javascript" class="w-full h-full" />
      </div>
      <div flex flex-col>
        <h2 text-2xl pb-2 pl-2>
          Output RS
        </h2>
        <div flex-grow relative>
          <Editor v-model="formatted" lang="rust" readonly class="w-full h-full" />
          <div z-20 absolute w-full top-0 children:p-2 children:px-3 children:b-2 children:rounded>
            <div v-if="error" text-red-200 bg-red-900 bg-op-80 b-red-500>
              <h3 text-lg pb-1>
                Error
              </h3>
              <div font-mono>
                {{ error }}
              </div>
            </div>
            <div v-if="loading" text-gray-300 bg-gray-900 bg-op-80 b-gray-500>
              <h3 text-lg pb-1>
                Running
              </h3>
              <div font-mono>
                Formatting Rust code...
              </div>
            </div>
            <button v-if="error || loading" absolute right-4 top-1 w-4 h-4 b-none i-carbon-close @click="error = ''; loading = false" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
