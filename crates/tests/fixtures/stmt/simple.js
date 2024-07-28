do {
  f(1)
} while (a < b)
for (let i = 0; i < 10; i++) {
  f(i)
}
for (const x of [1, 2, 3]) {
  f(x)
}
for (const x in { a: 1, b: 2 }) {
  f(x)
}