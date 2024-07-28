export async function formatRust(source: string, signal: AbortSignal) {
  const response = await fetch('https://play.rust-lang.org/format', {
    headers: {
      'accept': '*/*',
      'content-type': 'application/json',
    },
    body: JSON.stringify({ channel: 'stable', edition: '2018', code: source }),
    method: 'POST',
    mode: 'cors',
    credentials: 'omit',
    signal,
  })
  const data = await response.json()
  if (!data.success) {
    throw data.exitDetail ? new Error(`${data.exitDetail}\n${data.stderr}`) : new Error(`${response.status} ${response.statusText}`)
  }
  const code: string = data.code
  return code.replaceAll(/^ {2,}/gm, all => all.slice(all.length / 2))
}
