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
})
