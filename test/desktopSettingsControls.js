import { expect } from 'vitest'

import {
  commitDraftNumber,
  LONGBREAK_DURATION_PRESETS,
  LONGBREAK_EVERY_PRESETS,
  MICROBREAK_DURATION_PRESETS,
  MICROBREAK_INTERVAL_PRESETS,
} from '../apps/desktop/src/lib/settings-controls'
import { DESKTOP_LANGUAGE_CONFIGS, normalizeLanguage, resolveUiLanguage } from '../apps/desktop/src/i18n'

describe('Desktop settings controls', () => {
  it('exposes the full desktop language list instead of only zh-CN and en', () => {
    expect(DESKTOP_LANGUAGE_CONFIGS.length).toBeGreaterThan(2)
    expect(DESKTOP_LANGUAGE_CONFIGS.some((config) => config.code === 'tr')).toBe(true)
    expect(DESKTOP_LANGUAGE_CONFIGS.some((config) => config.code === 'ja')).toBe(true)
    expect(normalizeLanguage('tr')).toBe('tr')
    expect(normalizeLanguage('ja')).toBe('ja')
    expect(normalizeLanguage('zh')).toBe('zh-CN')
  })

  it('keeps the current UI locale until the persisted language save completes', () => {
    expect(resolveUiLanguage('tr', 'zh-CN')).toBe('zh-CN')
    expect(resolveUiLanguage('tr', 'tr')).toBe('tr')
    expect(resolveUiLanguage('zh', undefined)).toBe('zh-CN')
  })

  it('keeps five preset options for each break rhythm control', () => {
    expect(MICROBREAK_INTERVAL_PRESETS).toEqual([10, 20, 25, 30, 45])
    expect(MICROBREAK_DURATION_PRESETS).toEqual([20, 30, 45, 60, 120])
    expect(LONGBREAK_EVERY_PRESETS).toEqual([2, 3, 4, 5, 6])
    expect(LONGBREAK_DURATION_PRESETS).toEqual([5, 10, 15, 20, 30])
  })

  it('allows a custom numeric draft to be cleared temporarily before commit', () => {
    expect(commitDraftNumber('', 20, 5, 300)).toBe(20)
    expect(commitDraftNumber('0', 20, 5, 300)).toBe(5)
    expect(commitDraftNumber('137', 20, 5, 300)).toBe(137)
    expect(commitDraftNumber('999', 20, 5, 300)).toBe(300)
  })
})
