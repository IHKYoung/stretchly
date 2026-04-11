const targets = window.PAUZA_DOWNLOAD_TARGETS || {}
const body = document.body
const targetKey = body.dataset.target || ''
const targetUrl = String(targets[targetKey] || '').trim()

const titleEl = document.querySelector('[data-title]')
const subtitleEl = document.querySelector('[data-subtitle]')
const noteEl = document.querySelector('[data-note]')
const manualLinkEl = document.querySelector('[data-manual-link]')
const fallbackLinkEl = document.querySelector('[data-fallback-link]')
const routeEl = document.querySelector('[data-route-label]')
const countdownEl = document.querySelector('[data-countdown]')

const labels = {
  macosAppleSilicon: 'Pauza for Apple Silicon',
  macosIntel: 'Pauza for Intel',
  releases: 'Pauza releases',
}

const currentLabel = labels[targetKey] || 'Pauza download'

if (routeEl) {
  routeEl.textContent = `/download/${targetKey.replace(/[A-Z]/g, (match) => `-${match.toLowerCase()}`)}/`
}

if (targetUrl) {
  body.dataset.mode = 'loading'

  if (titleEl) titleEl.textContent = '正在准备下载'
  if (subtitleEl) subtitleEl.textContent = `即将跳转到 ${currentLabel}`
  if (noteEl) noteEl.textContent = '如果浏览器没有自动跳转，你也可以手动打开下载链接。'
  if (manualLinkEl) {
    manualLinkEl.href = targetUrl
    manualLinkEl.hidden = false
  }
  if (fallbackLinkEl && targets.releases) fallbackLinkEl.href = targets.releases

  let seconds = 2
  if (countdownEl) countdownEl.textContent = String(seconds)

  const countdownTimer = window.setInterval(() => {
    seconds -= 1
    if (countdownEl) countdownEl.textContent = String(Math.max(seconds, 0))
    if (seconds <= 0) window.clearInterval(countdownTimer)
  }, 1000)

  window.setTimeout(() => {
    window.location.assign(targetUrl)
  }, 1200)
} else {
  body.dataset.mode = 'missing'
  console.warn(`[Pauza site] Missing download target for "${targetKey}". Update apps/site/download/targets.js.`)

  if (titleEl) titleEl.textContent = '下载地址尚未配置'
  if (subtitleEl) subtitleEl.textContent = `${currentLabel} 的稳定路由已经建好，但真实下载地址还没有填入。`
  if (noteEl) {
    noteEl.innerHTML =
      '请在 <code>apps/site/download/targets.js</code> 中填入真实 URL。后续切换 GitHub Releases 或对象存储时，不需要改首页按钮。'
  }

  if (manualLinkEl) manualLinkEl.hidden = true

  if (fallbackLinkEl) {
    const releaseUrl = String(targets.releases || '').trim()
    fallbackLinkEl.href = releaseUrl || '../../'
    fallbackLinkEl.textContent = releaseUrl ? '暂时打开 Releases 页' : '返回首页'
  }
}
