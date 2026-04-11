const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches
const outputEl = document.querySelector('[data-typed-output]')

const lines = Array.isArray(window.PAUZA_SITE_LINES) && window.PAUZA_SITE_LINES.length
  ? window.PAUZA_SITE_LINES
  : ['久坐提醒，也可以更有意思一点。']

const wait = (time) =>
  new Promise((resolve) => {
    window.setTimeout(resolve, time)
  })

const getTypingDelay = (char, deleting) => {
  if (deleting) {
    return /[，。！？：；、,.!?]/.test(char) ? 26 : 18
  }

  if (/[，、]/.test(char)) {
    return 150
  }

  if (/[。！？.!?]/.test(char)) {
    return 240
  }

  if (/[:：]/.test(char)) {
    return 180
  }

  return 78
}

const runTypewriter = async () => {
  let index = 0

  while (true) {
    const line = lines[index]

    for (let length = 1; length <= line.length; length += 1) {
      const char = line[length - 1]
      outputEl.textContent = line.slice(0, length)
      await wait(getTypingDelay(char, false))
    }

    await wait(5000)

    for (let length = line.length; length > 0; length -= 1) {
      const char = line[length - 1]
      outputEl.textContent = line.slice(0, length - 1)
      await wait(getTypingDelay(char, true))
    }

    await wait(460)
    index = (index + 1) % lines.length
  }
}

if (outputEl) {
  if (reduceMotion) {
    outputEl.textContent = lines[0]
  } else {
    runTypewriter()
  }
}
