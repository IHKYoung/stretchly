import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { expect } from 'vitest'

const appSource = readFileSync(join(__dirname, '../apps/desktop/src/App.tsx'), 'utf8')

describe('Desktop settings navigation', () => {
  it('opens every settings topic directly from the overview', () => {
    const overviewStart = appSource.indexOf("case 'overview'")
    const overviewEnd = appSource.indexOf("case 'appearance'", overviewStart)
    const overviewSource = appSource.slice(overviewStart, overviewEnd)

    expect(overviewStart).toBeGreaterThan(-1)
    expect(overviewEnd).toBeGreaterThan(overviewStart)

    for (const route of ['rhythm', 'reminders', 'appearance', 'automation', 'system']) {
      expect(overviewSource).toContain(`setSettingsRoute('${route}')`)
    }
  })

  it('returns from every topic directly to the overview without a details route', () => {
    expect(appSource).toContain("const goBackInSettings = () => {\n    setSettingsRoute('overview')\n  }")
    expect(appSource).not.toContain("| 'details'")
    expect(appSource).not.toContain("case 'details'")
  })

  it('shows a custom-value affordance instead of echoing an active preset', () => {
    expect(appSource).toContain("value={isPreset && !customEditing ? '' : draft}")
    expect(appSource).toContain('placeholder={customLabel}')
    expect(appSource).toContain("t(language, 'ui.fullBreakCadenceHint'")
  })
})
