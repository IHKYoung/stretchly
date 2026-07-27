import { existsSync, readFileSync } from 'node:fs'
import { join } from 'path'
import { expect } from 'vitest'

const repoRoot = join(__dirname, '..')
const messagesDirectory = join(repoRoot, 'apps/desktop/src/locales/messages')
const breakIdeasDirectory = join(repoRoot, 'apps/desktop/src/locales/break-ideas/messages')
const breakIdeasRegistryPath = join(repoRoot, 'apps/desktop/src/locales/break-ideas/registry.generated.json')
const appSource = readFileSync(join(repoRoot, 'apps/desktop/src/App.tsx'), 'utf8')

function readMessages (file) {
  const filePath = join(messagesDirectory, file)
  return JSON.parse(readFileSync(filePath, 'utf8'))
}

function readBreakIdeas (file) {
  const filePath = join(breakIdeasDirectory, file)
  return JSON.parse(readFileSync(filePath, 'utf8'))
}

describe('Desktop break copy source of truth', () => {
  it('starts the long-break hold only after the complete prompt is typed', () => {
    const bodyTypedIndex = appSource.indexOf("await typeField('body', nextPrompt.body)")
    const holdIndex = appSource.indexOf(
      'await pause(LONG_BREAK_PROMPT_HOLD_MS)',
      bodyTypedIndex,
    )
    const switchIndex = appSource.indexOf(
      'await pause(LONG_BREAK_PROMPT_SWITCH_GAP_MS)',
      holdIndex,
    )

    expect(bodyTypedIndex).toBeGreaterThan(-1)
    expect(holdIndex).toBeGreaterThan(bodyTypedIndex)
    expect(switchIndex).toBeGreaterThan(holdIndex)
  })

  it('stores break prompt copy inside messages for desktop-ready languages', () => {
    for (const file of ['en.json', 'zh-CN.json']) {
      const messages = readMessages(file)
      const breakCopy = messages.ui.breakCopy

      expect(typeof breakCopy.clearedDetail).toBe('string')
      expect(typeof breakCopy.manualAwaiting).toBe('string')
      expect(typeof breakCopy.awaitingFinish).toBe('string')
      expect(typeof breakCopy.actions.resumeWork).toBe('string')
      expect(typeof breakCopy.actions.done).toBe('string')
      expect(typeof breakCopy.actions.later).toBe('string')
      expect(typeof breakCopy.actions.skip).toBe('string')
      expect(typeof breakCopy.defaultPrompt.microbreak).toBe('string')
      expect(typeof breakCopy.defaultPrompt.longBreak).toBe('string')
      expect('prompts' in breakCopy).toBe(false)
      expect('miniBreakIdeas' in messages).toBe(false)
      expect('longBreakIdeas' in messages).toBe(false)
    }
  })

  it('stores break ideas in dedicated locale assets and marks official languages in registry', () => {
    const registry = JSON.parse(readFileSync(breakIdeasRegistryPath, 'utf8'))

    expect(registry.officialLanguages).toEqual(['en', 'zh-CN', 'zh-TW'])
    expect(registry.languages.find((language) => language.code === 'zh-CN').tier).toBe('official')
    expect(registry.languages.find((language) => language.code === 'zh-TW').tier).toBe('official')
    expect(registry.languages.find((language) => language.code === 'en').tier).toBe('official')
    expect(registry.languages.find((language) => language.code === 'tr').tier).toBe('legacy')

    for (const file of ['en.json', 'zh-CN.json', 'zh-TW.json']) {
      const breakIdeas = readBreakIdeas(file)
      expect(typeof breakIdeas.miniBreakIdeas.aaa.text).toBe('string')
      expect(typeof breakIdeas.longBreakIdeas.aaa.text).toBe('string')
    }
  })

  it('does not keep the removed break-message-copy transition files', () => {
    expect(existsSync(join(repoRoot, 'apps/desktop/src/locales/break-message-copy.json'))).toBe(
      false,
    )
    expect(existsSync(join(repoRoot, 'apps/desktop/src/locales/break-message-copy.ts'))).toBe(
      false,
    )
  })
})
