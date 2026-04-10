import { existsSync, readFileSync } from 'node:fs'
import { join } from 'path'
import { expect } from 'vitest'

const repoRoot = join(__dirname, '..')
const messagesDirectory = join(repoRoot, 'apps/desktop/src/locales/messages')

function readMessages (file) {
  const filePath = join(messagesDirectory, file)
  return JSON.parse(readFileSync(filePath, 'utf8'))
}

describe('Desktop break copy source of truth', () => {
  it('stores break prompt copy inside messages for desktop-ready languages', () => {
    for (const file of ['en.json', 'zh-CN.json']) {
      const breakCopy = readMessages(file).ui.breakCopy

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
