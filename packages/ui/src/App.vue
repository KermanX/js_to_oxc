<script setup lang="ts">
import { compressToBase64, decompressFromBase64 } from 'lz-string'
import { generate_expression, generate_program } from 'js_to_oxc_wasm'
import { computed, ref, watch, watchEffect } from 'vue'
import Editor from './Editor.vue'
import { formatRust, formatterReady } from './format'

const input = ref('')
const programMode = ref(true)
const astBuilder = ref('')
const span = ref('')
const shouldFormat = ref(true)

const debouncedInput = ref('')
let debounceTimeout = Number.NaN
watch(input, (input) => {
  clearInterval(debounceTimeout)
  debounceTimeout = setTimeout(() => {
    debouncedInput.value = input
  }, 300)
})

function load() {
  let parsed
  if (window.location.hash) {
    try {
      parsed = JSON.parse(decompressFromBase64(window.location.hash.slice(1)) || '{}')
    }
    catch (e) { console.error(e) }
  }
  parsed ||= {}
  debouncedInput.value = input.value = parsed.input ?? `console.log('Hello', $name)\n`
  programMode.value = parsed.programMode ?? true
  astBuilder.value = parsed.astBuilder ?? 'self.ast_builder'
  span.value = parsed.span ?? 'SPAN'
  shouldFormat.value = parsed.shouldFormat ?? true
}

function save() {
  window.location.hash = compressToBase64(JSON.stringify({
    input: input.value,
    programMode: programMode.value,
    astBuilder: astBuilder.value,
    span: span.value,
  }))
}

load()
watchEffect(save)

const formatted = ref('')
const error = ref('')
const loading = ref(true)
const cannotFormat = ref(false)
const formatModelValue = computed({
  get: () => !cannotFormat.value && shouldFormat.value,
  set: value => shouldFormat.value = value,
})

watchEffect(() => {
  const { result: unformatted, errors } = (programMode.value ? generate_program : generate_expression)(debouncedInput.value, astBuilder.value, span.value)
  error.value = errors || ''
  formatted.value = unformatted
  if (unformatted.length > 1500) {
    cannotFormat.value = true
  }
  else {
    cannotFormat.value = false
  }
  if (cannotFormat.value || !shouldFormat.value) {
    loading.value = false
    return
  }
  if (formatterReady.value) {
    loading.value = false
    try {
      formatted.value = formatRust(unformatted)
    }
    catch (err) {
      console.error(err)
      error.value += String(err)
    }
  }
})
</script>

<template>
  <div py-2 md:py-4 fixed inset-0 flex flex-col>
    <div px-4 flex flex-wrap gap-x-2 pb-2>
      <h1 text-xl md:text-3xl font-bold md:pb-2 select-none flex flex-wrap items-center gap-x-2>
        <img src="/favicon.ico" h-1em>
        JS to Oxc
        <div text-sm self-end flex items-center gap-1 op-80>
          Convert JS to Oxc AST builder
          <a i-carbon-logo-github flex-grow inline-block w-1.2em h-1.2em hover:op-80 href="https://github.com/KermanX/js_to_oxc" target="_blank" />
        </div>
      </h1>
      <div flex-grow />
      <div flex w-fit md:flex-col h-min md:h-0 z-10 gap-x-2 gap-y-1 font-mono items-end mr-2>
        <label flex align-center gap-1 select-none>
          <span hidden sm:important:inline-block op-80>
            ast builder:
          </span>
          <input v-model="astBuilder" type="text" placeholder="ast_builder" bg-dark-300 px-1 rounded w-40 focus:outline-none @blur="astBuilder ||= 'self.ast_builder'">
        </label>
        <label flex align-center gap-1 select-none>
          <span hidden sm:important:inline-block op-80>
            empty span:
          </span>
          <input v-model="span" type="text" bg-dark-300 px-1 rounded w-40 focus:outline-none @blur="span ||= 'SPAN'">
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
        </div>
        <Editor v-model="input" lang="javascript" class="flex-grow h-0 max-h-full" />
      </div>
      <div flex-grow h-0 md:h-full md:w-0 flex flex-col>
        <h2 md:text-xl pb-2 pl-4 select-none flex items-end>
          Output RS
          <div flex-grow />
          <div mr-6>
            <label text-sm flex items-center op-80 :title="cannotFormat ? 'Too long to be formatted' : undefined">
              <input v-model="formatModelValue" type="checkbox" class="mr-1" :disabled="cannotFormat">
              Format
            </label>
          </div>
        </h2>
        <div flex-grow relative max-h-full>
          <Editor v-model="formatted" lang="rust" readonly class="w-full h-full max-h-full" />
          <div z-20 absolute left-1 right-2 bottom--2 children:p-2 children:px-3 children:b-2 children:rounded flex flex-col gap-2>
            <div v-if="error" relative text-red-200 bg-red-900 bg-op-80 b-red-500>
              <h3 text-lg pb-1>
                Error
              </h3>
              <div font-mono>
                {{ error }}
              </div>
              <button absolute right-3 top-3 w-6 h-6 b-none i-carbon-close @click="error = ''" />
            </div>
            <div v-if="loading" relative text-gray-300 bg-gray-900 bg-op-80 b-gray-500>
              <h3 text-lg pb-1>
                Running
              </h3>
              <div font-mono>
                Loading formatter
              </div>
              <button absolute right-3 top-3 w-6 h-6 b-none i-carbon-close @click="loading = false" />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
