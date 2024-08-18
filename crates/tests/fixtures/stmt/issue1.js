async function test() {
  let { b } = await __vitePreload(async () => { const { b } = await import('./lib-!~{002}~.js');return { b }},true?__VITE_PRELOAD__:void 0);
}
