const reduceMotionQuery = window.matchMedia('(prefers-reduced-motion: reduce)')
let reduceMotion = reduceMotionQuery.matches
const outputEl = document.querySelector('[data-typed-output]')
const stageEl = document.querySelector('[data-typewriter-stage]')
const layerEl = document.querySelector('[data-interaction-layer]')
const rootEl = document.documentElement
const downloadButtonEl = document.querySelector('[data-download-button]')

const HOLD_DELAY = 10000
const MAX_ACTIVE_BUBBLES = 4
const PARTICLE_SYMBOLS = Object.freeze({
  default: Object.freeze(['0', '1', '#', '0', '1', '#', '@', '！', '¥', '$', '0', '1', '#']),
  trail: Object.freeze(['0', '1', '#', '0', '1', '#', '@']),
  sweep: Object.freeze(['#', '#', '0', '1', '@', '¥', '$', '！']),
  burst: Object.freeze(['0', '1', '#', '0', '1', '#', '@', '！', '¥', '$', '0', '1', '#', '@']),
})

const activeBubbles = []
const pointerState = {
  x: window.innerWidth * 0.5,
  y: window.innerHeight * 0.5,
}

let nudgeIndex = 0
let lastTrailAt = 0
let lastWheelAt = 0

const lines = Array.isArray(window.PAUZA_SITE_LINES) && window.PAUZA_SITE_LINES.length
  ? window.PAUZA_SITE_LINES
  : ['久坐提醒，也可以更有意思一点。']

const nudges = Array.isArray(window.PAUZA_SITE_NUDGES) && window.PAUZA_SITE_NUDGES.length
  ? window.PAUZA_SITE_NUDGES
  : ['先起来活动一下，别让椅子以为你住这儿。']

const wait = (time) =>
  new Promise((resolve) => {
    window.setTimeout(resolve, time)
  })

const clamp = (value, min, max) => Math.min(max, Math.max(min, value))
const randomBetween = (min, max) => min + Math.random() * (max - min)
const pickOne = (items) => items[Math.floor(Math.random() * items.length)]

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

    await wait(HOLD_DELAY)

    for (let length = line.length; length > 0; length -= 1) {
      const char = line[length - 1]
      outputEl.textContent = line.slice(0, length - 1)
      await wait(getTypingDelay(char, true))
    }

    await wait(460)
    index = (index + 1) % lines.length
  }
}

const removeBubble = (bubble) => {
  const index = activeBubbles.indexOf(bubble)
  if (index >= 0) {
    activeBubbles.splice(index, 1)
  }

  bubble.remove()
}

const animateNode = (node, keyframes, timing, onFinish) => {
  if (!node) {
    return
  }

  if (reduceMotion) {
    Object.assign(node.style, keyframes.at(-1))
    window.setTimeout(() => {
      onFinish?.()
    }, 900)
    return
  }

  const animation = node.animate(keyframes, timing)
  animation.addEventListener('finish', () => {
    onFinish?.()
  })
}

const pickParticleSymbol = (variant) => {
  const bucket = PARTICLE_SYMBOLS[variant] ?? PARTICLE_SYMBOLS.default
  return pickOne(bucket)
}

const spawnParticle = (x, y, options = {}) => {
  if (!layerEl) {
    return
  }

  const particle = document.createElement('span')
  particle.className = `particle${options.variant ? ` particle--${options.variant}` : ''}`
  particle.textContent = options.symbol ?? pickParticleSymbol(options.variant)

  const size = options.size ?? randomBetween(16, 28)
  particle.style.fontSize = `${size}px`
  particle.style.left = `${x}px`
  particle.style.top = `${y}px`
  particle.style.opacity = String(options.opacity ?? randomBetween(0.35, 0.9))
  layerEl.append(particle)

  const dx = options.dx ?? randomBetween(-90, 90)
  const dy = options.dy ?? randomBetween(-90, 90)
  const rotate = randomBetween(-60, 60)
  const duration = options.duration ?? randomBetween(760, 1120)

  animateNode(
    particle,
    [
      {
        transform: 'translate(-50%, -50%) translate(0px, 0px) scale(0.24) rotate(0deg)',
        opacity: Number(particle.style.opacity),
      },
      {
        transform: `translate(-50%, -50%) translate(${dx}px, ${dy}px) scale(1) rotate(${rotate}deg)`,
        opacity: 0,
      },
    ],
    {
      duration,
      easing: options.easing ?? 'cubic-bezier(0.16, 1, 0.3, 1)',
      fill: 'forwards',
    },
    () => particle.remove(),
  )
}

const spawnRipple = (x, y, scale = 1) => {
  if (!layerEl) {
    return
  }

  const ripple = document.createElement('span')
  ripple.className = 'interaction-ripple'
  ripple.style.left = `${x}px`
  ripple.style.top = `${y}px`
  layerEl.append(ripple)

  const targetScale = 1.8 * scale
  animateNode(
    ripple,
    [
      {
        transform: 'translate(-50%, -50%) scale(0.25)',
        opacity: 0.62,
      },
      {
        transform: `translate(-50%, -50%) scale(${targetScale})`,
        opacity: 0,
      },
    ],
    {
      duration: reduceMotion ? 680 : 920,
      easing: 'cubic-bezier(0.2, 0.9, 0.25, 1)',
      fill: 'forwards',
    },
    () => ripple.remove(),
  )
}

const spawnTrail = (x, y) => {
  if (reduceMotion) {
    return
  }

  const now = Date.now()
  if (now - lastTrailAt < 80) {
    return
  }

  lastTrailAt = now
  spawnParticle(x, y, {
    variant: 'trail',
    size: randomBetween(12, 18),
    dx: randomBetween(-32, 32),
    dy: randomBetween(-28, 28),
    opacity: randomBetween(0.24, 0.46),
    duration: randomBetween(560, 920),
    easing: 'ease-out',
  })
}

const spawnBurst = (x, y, pointerType = 'mouse') => {
  const count = pointerType === 'touch' ? 14 : 21
  const burstSymbols = PARTICLE_SYMBOLS.burst

  for (let index = 0; index < count; index += 1) {
    const angle = (Math.PI * 2 * index) / count + randomBetween(-0.22, 0.22)
    const distance = randomBetween(34, pointerType === 'touch' ? 82 : 112)
    spawnParticle(x, y, {
      variant: 'burst',
      symbol: burstSymbols[index % burstSymbols.length],
      size: randomBetween(16, 28),
      dx: Math.cos(angle) * distance,
      dy: Math.sin(angle) * distance,
      opacity: randomBetween(0.42, 0.88),
      duration: randomBetween(820, 1280),
    })
  }

  spawnRipple(x, y, pointerType === 'touch' ? 0.9 : 1)
}

const spawnWheelSweep = (deltaY) => {
  if (reduceMotion) {
    return
  }

  const now = Date.now()
  if (now - lastWheelAt < 140) {
    return
  }

  lastWheelAt = now

  const direction = deltaY >= 0 ? 1 : -1
  for (let index = 0; index < 7; index += 1) {
    spawnParticle(pointerState.x + randomBetween(-26, 26), pointerState.y, {
      variant: 'sweep',
      size: randomBetween(13, 20),
      dx: randomBetween(-22, 22),
      dy: direction * randomBetween(30, 72),
      opacity: randomBetween(0.24, 0.44),
      duration: randomBetween(580, 880),
      easing: 'ease-out',
    })
  }
}

const updatePointerState = (x, y) => {
  pointerState.x = x
  pointerState.y = y
  rootEl.style.setProperty('--pointer-x', `${x}px`)
  rootEl.style.setProperty('--pointer-y', `${y}px`)
  rootEl.style.setProperty('--aura-opacity', reduceMotion ? '0.18' : '0.88')

  if (!stageEl) {
    return
  }

  const rect = stageEl.getBoundingClientRect()
  const offsetX = clamp((x - rect.left) / rect.width - 0.5, -0.5, 0.5)
  const offsetY = clamp((y - rect.top) / rect.height - 0.5, -0.5, 0.5)

  if (reduceMotion) {
    stageEl.style.setProperty('--tilt-x', '0deg')
    stageEl.style.setProperty('--tilt-y', '0deg')
    stageEl.style.setProperty('--stage-lift', '0px')
    return
  }

  stageEl.style.setProperty('--tilt-x', `${(-offsetY * 1.45).toFixed(2)}deg`)
  stageEl.style.setProperty('--tilt-y', `${(offsetX * 1.75).toFixed(2)}deg`)
  stageEl.style.setProperty('--stage-lift', `${(-Math.abs(offsetX) - Math.abs(offsetY)) * 6}px`)
}

const resetPointerState = () => {
  rootEl.style.setProperty('--aura-opacity', '0')

  if (!stageEl) {
    return
  }

  stageEl.style.setProperty('--tilt-x', '0deg')
  stageEl.style.setProperty('--tilt-y', '0deg')
  stageEl.style.setProperty('--stage-lift', '0px')
}

const nextNudge = () => {
  const text = nudges[nudgeIndex % nudges.length]
  nudgeIndex += 1
  return text
}

const getDownloadConfig = () => {
  const targets = window.PAUZA_DOWNLOAD_TARGETS || {}
  const macosTarget = targets.macosAppleSilicon

  if (!macosTarget || typeof macosTarget !== 'object') {
    return {
      fallbackUrl: downloadButtonEl?.href || '',
      latestReleaseApi: '',
      assetNameSuffix: '',
    }
  }

  return {
    fallbackUrl: String(macosTarget.fallbackUrl || downloadButtonEl?.href || '').trim(),
    latestReleaseApi: String(macosTarget.latestReleaseApi || '').trim(),
    assetNameSuffix: String(macosTarget.assetNameSuffix || '').trim(),
  }
}

const getAssetNameFromUrl = (value) => {
  if (!value) {
    return ''
  }

  try {
    const url = new URL(value, window.location.href)
    const segments = url.pathname.split('/')
    return String(segments[segments.length - 1] || '').trim()
  } catch {
    const sanitized = String(value).split('#')[0].split('?')[0]
    const segments = sanitized.split('/')
    return String(segments[segments.length - 1] || '').trim()
  }
}

let latestDownloadUrlPromise = null

const resolveLatestDownloadUrl = async () => {
  const config = getDownloadConfig()
  if (!config.latestReleaseApi || !config.assetNameSuffix) {
    return config.fallbackUrl
  }

  if (!latestDownloadUrlPromise) {
    latestDownloadUrlPromise = window
      .fetch(config.latestReleaseApi, {
        headers: {
          Accept: 'application/vnd.github+json',
        },
      })
      .then(async (response) => {
        if (!response.ok) {
          throw new Error(`GitHub API responded with ${response.status}`)
        }

        const release = await response.json()
        const assets = Array.isArray(release.assets) ? release.assets : []
        const matchedAsset = assets.find((asset) =>
          String(asset && asset.name ? asset.name : '').endsWith(config.assetNameSuffix)
        )
        const matchedAssetName = String(matchedAsset && matchedAsset.name ? matchedAsset.name : '').trim()
        const matchedAssetUrl = String(
          matchedAsset && matchedAsset.browser_download_url ? matchedAsset.browser_download_url : ''
        ).trim()
        const pinnedAssetName = getAssetNameFromUrl(config.fallbackUrl)

        // Keep the pinned release authoritative when GitHub "latest" still points to an older asset.
        if (pinnedAssetName && matchedAssetName && matchedAssetName !== pinnedAssetName) {
          return config.fallbackUrl
        }

        return matchedAssetUrl
      })
      .catch((error) => {
        console.warn('[Pauza site] Failed to resolve latest GitHub release asset.', error)
        return ''
      })
  }

  const latestUrl = await latestDownloadUrlPromise
  return latestUrl || config.fallbackUrl
}

const spawnNudge = (x, y) => {
  if (!layerEl) {
    return
  }

  const bubble = document.createElement('p')
  bubble.className = 'nudge-bubble'
  bubble.textContent = nextNudge()

  const bubbleX = clamp(x, 128, window.innerWidth - 128)
  const placeAbove = y > window.innerHeight * 0.56
  const bubbleY = clamp(placeAbove ? y - 42 : y + 24, 42, window.innerHeight - 42)

  bubble.style.left = `${bubbleX}px`
  bubble.style.top = `${bubbleY}px`
  layerEl.append(bubble)

  activeBubbles.push(bubble)
  if (activeBubbles.length > MAX_ACTIVE_BUBBLES) {
    const oldest = activeBubbles.shift()
    oldest?.remove()
  }

  const fromY = placeAbove ? 18 : -18
  const toY = placeAbove ? -18 : 18

  animateNode(
    bubble,
    [
      {
        transform: `translate(-50%, 0) translateY(${fromY}px) scale(0.92)`,
        opacity: 0,
      },
      {
        transform: 'translate(-50%, 0) translateY(0px) scale(1)',
        opacity: 1,
        offset: 0.18,
      },
      {
        transform: `translate(-50%, 0) translateY(${toY}px) scale(1)`,
        opacity: 0,
      },
    ],
    {
      duration: reduceMotion ? 1600 : 2600,
      easing: 'cubic-bezier(0.22, 1, 0.36, 1)',
      fill: 'forwards',
    },
    () => removeBubble(bubble),
  )
}

const triggerNudge = (x, y, pointerType = 'mouse') => {
  updatePointerState(x, y)
  spawnBurst(x, y, pointerType)
  spawnNudge(x, y)
}

if (outputEl) {
  if (reduceMotion) {
    outputEl.textContent = lines[0]
  } else {
    runTypewriter()
  }
}

if (stageEl && layerEl) {
  window.addEventListener('pointermove', (event) => {
    if (event.pointerType === 'touch') {
      return
    }

    updatePointerState(event.clientX, event.clientY)
    spawnTrail(event.clientX, event.clientY)
  })

  window.addEventListener(
    'wheel',
    (event) => {
      spawnWheelSweep(event.deltaY)
    },
    { passive: true },
  )

  window.addEventListener('pointerdown', (event) => {
    if (event.button !== 0) {
      return
    }

    if (event.target.closest('.download-button')) {
      return
    }

    triggerNudge(event.clientX, event.clientY, event.pointerType)
  })

  stageEl.addEventListener('keydown', (event) => {
    if (event.key !== 'Enter' && event.key !== ' ') {
      return
    }

    event.preventDefault()
    const rect = stageEl.getBoundingClientRect()
    triggerNudge(rect.left + rect.width / 2, rect.top + rect.height * 0.72, 'keyboard')
  })

  document.body.addEventListener('mouseleave', resetPointerState)
  window.addEventListener('blur', resetPointerState)

  if (typeof reduceMotionQuery.addEventListener === 'function') {
    reduceMotionQuery.addEventListener('change', (event) => {
      reduceMotion = event.matches
      if (reduceMotion) {
        resetPointerState()
      }
    })
  }
}

if (downloadButtonEl) {
  const config = getDownloadConfig()
  if (config.fallbackUrl) {
    downloadButtonEl.href = config.fallbackUrl
  }

  void resolveLatestDownloadUrl().then((latestUrl) => {
    if (latestUrl) {
      downloadButtonEl.href = latestUrl
    }
  })

  downloadButtonEl.addEventListener('click', async (event) => {
    event.preventDefault()
    const targetUrl = await resolveLatestDownloadUrl()
    if (!targetUrl) {
      return
    }

    downloadButtonEl.href = targetUrl
    window.location.assign(targetUrl)
  })
}
