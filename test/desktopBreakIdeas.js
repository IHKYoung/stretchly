import { expect } from 'vitest'

import { tBreakIdeaList } from '../apps/desktop/src/i18n'
import { pickBreakPrompt, pickBreakPromptIndex } from '../apps/desktop/src/lib/break-ideas'

describe('Desktop break ideas source and selection', () => {
  it('reads break prompts from locale idea sources', () => {
    const zhMicrobreakIdeas = tBreakIdeaList('zh-CN', 'microbreak')
    const enLongBreakIdeas = tBreakIdeaList('en', 'longBreak')

    expect(zhMicrobreakIdeas.length).toBeGreaterThan(20)
    expect(typeof zhMicrobreakIdeas[0]).toBe('string')
    expect(zhMicrobreakIdeas[0].length).toBeGreaterThan(0)
    expect(enLongBreakIdeas.length).toBeGreaterThan(20)
    expect(enLongBreakIdeas[0]).toContain('Find it hard to take breaks alone?')
  })

  it('does not collapse fixed 10-minute and 30-minute schedules into one repeated prompt', () => {
    const sourceLength = 50
    const startedAtMs = 1_700_000_000_000

    expect(pickBreakPromptIndex('microbreak', startedAtMs, sourceLength)).not.toBe(
      pickBreakPromptIndex('microbreak', startedAtMs + 10 * 60_000, sourceLength),
    )
    expect(pickBreakPromptIndex('longBreak', startedAtMs, sourceLength)).not.toBe(
      pickBreakPromptIndex('longBreak', startedAtMs + 30 * 60_000, sourceLength),
    )
  })

  it('keeps one prompt stable for the same break instance', () => {
    const startedAtMs = 1_700_000_000_000

    expect(pickBreakPrompt('zh-CN', 'microbreak', startedAtMs)).toBe(
      pickBreakPrompt('zh-CN', 'microbreak', startedAtMs),
    )
    expect(pickBreakPrompt('en', 'longBreak', startedAtMs)).toBe(
      pickBreakPrompt('en', 'longBreak', startedAtMs),
    )
  })
})
