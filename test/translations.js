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
})
