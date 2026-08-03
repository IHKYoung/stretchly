import { readFileSync, readdirSync } from 'node:fs'
import { join } from 'path'
import { expect } from 'vitest'

const localeJsonDirectory = join(__dirname, '../apps/desktop/src/locales/messages')
const breakIdeaJsonDirectory = join(__dirname, '../apps/desktop/src/locales/break-ideas/messages')

function getJsonFiles (dir) {
  return readdirSync(dir).filter(file => file.endsWith('.json'))
}

describe('Translations files', () => {
  const localeJsonFiles = getJsonFiles(localeJsonDirectory)
  const breakIdeaJsonFiles = getJsonFiles(breakIdeaJsonDirectory)

  localeJsonFiles.forEach(file => {
    it(`${file} is valid`, () => {
      const filePath = join(localeJsonDirectory, file)
      const data = readFileSync(filePath, 'utf8')

      expect(() => {
        JSON.parse(data)
      }).not.toThrow()
    })
  })

  breakIdeaJsonFiles.forEach(file => {
    it(`break ideas ${file} is valid`, () => {
      const filePath = join(breakIdeaJsonDirectory, file)
      const data = readFileSync(filePath, 'utf8')

      expect(() => {
        JSON.parse(data)
      }).not.toThrow()
    })
  })

  it('uses paired microbreak and full-break terminology in primary UI locales', () => {
    const zhCN = JSON.parse(readFileSync(join(localeJsonDirectory, 'zh-CN.json'), 'utf8'))
    const zhTW = JSON.parse(readFileSync(join(localeJsonDirectory, 'zh-TW.json'), 'utf8'))
    const en = JSON.parse(readFileSync(join(localeJsonDirectory, 'en.json'), 'utf8'))

    expect(zhCN.ui.longBreaks).toBe('完整休息')
    expect(zhCN.ui.status.longBreak).toBe('完整休息')
    expect(zhCN.runtime.break.long.title).toBe('完整休息')
    expect(zhCN.runtime.tray.skipLongBreak).toBe('完整休息')
    expect(zhTW.ui.longBreaks).toBe('完整休息')
    expect(zhTW.ui.status.longBreak).toBe('完整休息')
    expect(en.ui.longBreaks).toBe('Full breaks')
    expect(en.runtime.break.long.title).toBe('Full break')
  })

  it('describes rhythm values with explicit actions and break-slot units', () => {
    const zhCN = JSON.parse(readFileSync(join(localeJsonDirectory, 'zh-CN.json'), 'utf8'))
    const zhTW = JSON.parse(readFileSync(join(localeJsonDirectory, 'zh-TW.json'), 'utf8'))
    const en = JSON.parse(readFileSync(join(localeJsonDirectory, 'en.json'), 'utf8'))

    expect(zhCN.ui.interval).toBe('每隔')
    expect(zhCN.ui.durationLabel).toBe('持续')
    expect(zhCN.ui.suffix.reminderCycles).toBe('个提醒周期')
    expect(zhCN.ui.fullBreakCadenceHint).toContain('完整休息，其余为微休息')
    expect(zhCN.ui.fullBreakCadenceHint).not.toContain('休息提醒')
    expect(zhTW.ui.suffix.reminderCycles).toBe('個提醒週期')
    expect(en.ui.every).toBe('Every')
    expect(en.ui.suffix.reminderCycles).toBe('reminder cycles')
  })

  it('does not promise an immediate start while smart delivery may still wait', () => {
    const zhCN = JSON.parse(readFileSync(join(localeJsonDirectory, 'zh-CN.json'), 'utf8'))
    const en = JSON.parse(readFileSync(join(localeJsonDirectory, 'en.json'), 'utf8'))

    expect(zhCN.runtime.actions.skipToScheduledVia).toContain('计划提醒')
    expect(zhCN.runtime.actions.skipToScheduledVia).not.toContain('计划休息')
    expect(zhCN.runtime.actions.skipToSpecificVia).toContain('提前到现在')
    expect(zhCN.runtime.actions.skipToSpecificVia).not.toContain('直接开始')
    expect(en.runtime.actions.skipToSpecificVia).not.toContain('immediately')
  })
})
