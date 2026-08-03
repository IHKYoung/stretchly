import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { expect } from 'vitest'

const repoRoot = join(__dirname, '..')
const appSource = readFileSync(join(repoRoot, 'apps/desktop/src/App.tsx'), 'utf8')
const stylesSource = readFileSync(join(repoRoot, 'apps/desktop/src/styles.css'), 'utf8')

function cssRule (selector) {
  const start = stylesSource.indexOf(`${selector} {`)
  const end = stylesSource.indexOf('}', start)

  expect(start).toBeGreaterThan(-1)
  expect(end).toBeGreaterThan(start)
  return stylesSource.slice(start, end)
}

describe('Desktop settings typography', () => {
  it('keeps the LXGW typeface across settings and break surfaces', () => {
    expect(appSource).not.toContain('settings-shell')
    expect(stylesSource).not.toContain('--app-font-settings')
    expect(stylesSource).toContain('--app-font-sans: "LXGW WenKai Screen"')
    expect(cssRule('.type-break')).toContain('font-family: "LXGW WenKai Screen"')
  })

  it('keeps page, navigation, row, and detail text visually distinct', () => {
    expect(cssRule('.settings-page-title')).toContain('font-size: 16px')
    expect(cssRule('.settings-page-title')).toContain('font-weight: 600')
    expect(cssRule('.settings-link-title')).toContain('font-size: 14px')
    expect(cssRule('.settings-link-title')).toContain('font-weight: 600')
    expect(cssRule('.settings-row-title')).toContain('font-size: 14px')
    expect(cssRule('.settings-row-title')).toContain('font-weight: 500')
    expect(cssRule('.settings-detail')).toContain('font-size: 12px')
    expect(cssRule('.settings-detail')).toContain('font-weight: 400')
  })
})
