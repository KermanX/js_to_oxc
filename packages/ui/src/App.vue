<script setup lang="ts">
import { debouncedWatch, useLocalStorage } from '@vueuse/core'
import { generate_expression, generate_program } from 'js_to_oxc_wasm'
import { ref, shallowRef } from 'vue'
import Editor from './Editor.vue'
import { formatRust } from './format'

const js = ref('console.log("Hello, World!")')
const autoRun = useLocalStorage('js_to_oxc:autoRun', true)
const shouldFormat = useLocalStorage('js_to_oxc:shouldFormat', true)
const programMode = useLocalStorage('js_to_oxc:programMode', true)

const formatted = ref('')
const formatting = ref(false)
const error = ref('')
const controller = shallowRef<AbortController>()

async function run() {
  const unformatted = `impl _ {
    fn create_ast(&self) {
      ${(programMode.value ? generate_program : generate_expression)(js.value)}
    }
  }`
  controller.value?.abort()
  const { signal } = controller.value = new AbortController()
  try {
    error.value = ''
    formatted.value = unformatted
    if (shouldFormat.value) {
      formatting.value = true
      formatRust(unformatted, signal).then((result) => {
        if (!signal.aborted) {
          formatted.value = result
          formatting.value = false
        }
      }).catch((err) => {
        if (!signal.aborted) {
          error.value = String(err)
          formatted.value = unformatted
          formatting.value = false
        }
      })
    }
  }
  catch (err: any) {
    if (err instanceof DOMException && err.name === 'AbortError')
      return
    error.value = String(err)
    formatted.value = unformatted
    formatting.value = false
  }
}

debouncedWatch(
  () => autoRun.value && [js.value, shouldFormat.value, programMode.value],
  shouldRun => shouldRun && run(),
  { immediate: true, debounce: 300 },
)
</script>

<template>
  <div py-2 md:py-4 fixed inset-0 flex flex-col>
    <div px-4 flex>
      <h1 text-xl md:text-3xl font-bold pb-2 md:pb-4 select-none flex flex-wrap items-start gap-x-2>
        JS to Oxc
        <div text-sm self-end flex items-center gap-1 op-80>
          Convert JS to Oxc AST builder
          <a i-carbon-logo-github flex-grow inline-block w-1.2em h-1.2em hover:op-80 href="https://github.com/KermanX/js_to_oxc" target="_blank" />
        </div>
      </h1>
      <div flex-grow />
      <div flex md:flex-col h-min md:h-0 z-10 gap-x-2>
        <label flex align-center gap-1 select-none>
          <input v-model="autoRun" type="checkbox">
          Auto Run
        </label>
        <label flex align-center gap-1 select-none>
          <input v-model="shouldFormat" type="checkbox">
          Format Rust
        </label>
      </div>
    </div>
    <div flex-grow flex flex-col md:flex-row gap-x-6 gap-y-2>
      <div flex-grow h-0 md:h-full md:w-0 flex flex-col>
        <div flex items-center>
          <h2 md:text-xl pb-2 pl-4 select-none>
            Input JS
            <button md:text-lg underline underline-dotted underline-white underline-offset-3 underline-op60 bg-gray-700 hover:bg-gray-600 px-2 py-1 my--1 rounded-lg @click="programMode = !programMode">
              {{ programMode ? 'Program' : 'Expression' }}
            </button>
          </h2>
          <div flex-grow />
          <div>
            <button v-if="!autoRun" bg-gray-400 bg-op-40 py-.5 px-2.5 mr-2 md:mr-0 rounded hover:bg-op-50 select-none @click="run">
              Run
            </button>
          </div>
        </div>
        <Editor v-model="js" lang="javascript" class="flex-grow h-0 max-h-full" />
      </div>
      <div flex-grow h-0 md:h-full md:w-0 flex flex-col>
        <h2 md:text-xl pb-2 pl-4 select-none>
          Output RS
        </h2>
        <div flex-grow relative max-h-full>
          <Editor v-model="formatted" lang="rust" readonly class="w-full h-full max-h-full" />
          <div z-20 absolute left-1 right-1 bottom-0 children:p-2 children:px-3 children:b-2 children:rounded>
            <div v-if="error" text-red-200 bg-red-900 bg-op-80 b-red-500>
              <h3 text-lg pb-1>
                Error
              </h3>
              <div font-mono>
                {{ error }}
              </div>
            </div>
            <div v-if="formatting" text-gray-300 bg-gray-900 bg-op-80 b-gray-500>
              <h3 text-lg pb-1>
                Running
              </h3>
              <div font-mono>
                Formatting Rust code...
              </div>
            </div>
            <button v-if="error || formatting" absolute right-3 top-3 w-6 h-6 b-none i-carbon-close @click="error = ''; formatting = false" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
