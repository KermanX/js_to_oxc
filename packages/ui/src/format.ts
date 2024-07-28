const start = `impl _ {\n    fn create_ast(&self) {`
const end = `    }\n}`

export async function formatRust(source: string, signal: AbortSignal) {
  const response = await fetch('https://play.rust-lang.org/format', {
    headers: {
      'accept': '*/*',
      'content-type': 'application/json',
    },
    body: JSON.stringify({ channel: 'stable', edition: '2018', code: start + source + end }),
    method: 'POST',
    mode: 'cors',
    credentials: 'omit',
    signal,
  })
  const data = await response.json()
  if (!data.success) {
    throw data.exitDetail ? new Error(`Formatter error: ${data.exitDetail}\n${data.stderr}`) : new Error(`Formatter error: ${response.status} ${response.statusText}`)
  }
  const code: string = data.code.slice(start.length, -end.length)
  return `${code.replaceAll(/^ {8}( *)/gm, (_, all) => all.slice(all.length / 2)).trim()}\n`
}
