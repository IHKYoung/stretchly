import { expect } from 'vitest'

import { pickBreakPromptEntry, rotateBreakPromptEntries } from '../apps/desktop/src/lib/break-ideas'
import { tBreakIdeaEntries } from '../apps/desktop/src/i18n'

describe('Desktop break prompt rotation', () => {
  it('keeps microbreak prompts to a single stable entry per break', () => {
    const startedAtMs = 1712505600000

    expect(rotateBreakPromptEntries('zh-CN', 'microbreak', startedAtMs)).toEqual([
      pickBreakPromptEntry('zh-CN', 'microbreak', startedAtMs),
    ])
  })

  it('rotates through the full prompt list without dropping entries', () => {
    const startedAtMs = 1712505600000
    const source = tBreakIdeaEntries('en', 'longBreak')
    const rotated = rotateBreakPromptEntries('en', 'longBreak', startedAtMs)

    expect(rotated).toHaveLength(source.length)
    expect(
      new Set(rotated.map((entry) => `${entry.title ?? ''}::${entry.text}`)).size,
    ).toBe(source.length)
  })
})
