// @ts-check
import antfu from '@antfu/eslint-config'

export default antfu({
  ignores: [
    'crates/**/*',
  ],
  vue: true,
  toml: false,
})
