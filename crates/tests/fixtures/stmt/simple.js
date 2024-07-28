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
if (a < b) {
  f(1)
}
if (a < b) {
  f(1)
} else {
  f(2)
}
label: 1
switch (a) {
  case 1:
    f(1)
    break
  default:
    f(2)
}
throw x + y
try {
  f(1)
} catch ([e]) {
  f(2)
} finally {
  f(3)
}

