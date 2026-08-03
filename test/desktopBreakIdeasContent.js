import { readFileSync, readdirSync } from 'node:fs'
import { join } from 'node:path'
import { expect } from 'vitest'

import { splitBreakPromptLines } from '../apps/desktop/src/lib/break-copy-layout'

const repoRoot = join(__dirname, '..')
const breakIdeasRoot = join(repoRoot, 'apps/desktop/src/locales/break-ideas')
const messagesDirectory = join(breakIdeasRoot, 'messages')
const batchesDirectory = join(breakIdeasRoot, 'batches')
const registrySource = readJson(join(breakIdeasRoot, 'registry.json'))
const officialLanguages = registrySource.officialLanguages
const ideaKinds = ['miniBreakIdeas', 'longBreakIdeas']
const expectedFields = {
  miniBreakIdeas: ['text'],
  longBreakIdeas: ['text', 'title']
}

function readJson (path) {
  return JSON.parse(readFileSync(path, 'utf8'))
}

function ideaIdNumber (ideaId) {
  return [...ideaId].reduce(
    (value, character) => value * 26 + character.charCodeAt(0) - 'a'.charCodeAt(0),
    0
  )
}

function ideaIdFromNumber (value) {
  const characters = ['a', 'a', 'a']

  for (let index = 2; index >= 0; index -= 1) {
    characters[index] = String.fromCharCode('a'.charCodeAt(0) + (value % 26))
    value = Math.floor(value / 26)
  }

  return characters.join('')
}

function normalizedCopy (value) {
  return value
    .normalize('NFKC')
    .toLocaleLowerCase()
    .replace(/[\p{P}\p{S}\s]/gu, '')
}

function copyLength (value) {
  return [...value].length
}

const bundles = Object.fromEntries(
  officialLanguages.map((code) => [code, readJson(join(messagesDirectory, `${code}.json`))])
)
const batches = readdirSync(batchesDirectory)
  .filter((file) => file.endsWith('.json'))
  .sort()
  .map((file) => ({ file, data: readJson(join(batchesDirectory, file)) }))

describe('Desktop break idea content governance', () => {
  it('keeps official locale IDs, order, and strict entry shapes aligned', () => {
    const reference = bundles[officialLanguages[0]]

    for (const kind of ideaKinds) {
      const referenceIds = Object.keys(reference[kind])
      expect(referenceIds).toEqual(
        Array.from({ length: referenceIds.length }, (_, index) => ideaIdFromNumber(index))
      )

      for (const code of officialLanguages) {
        expect(Object.keys(bundles[code][kind])).toEqual(referenceIds)

        for (const entry of Object.values(bundles[code][kind])) {
          expect(Object.keys(entry).sort()).toEqual(expectedFields[kind])
          for (const field of expectedFields[kind]) {
            expect(entry[field].trim()).toBe(entry[field])
            expect(entry[field].length).toBeGreaterThan(0)
          }
        }
      }
    }
  })

  it('keeps batch manifests metadata-only, continuous, balanced, and interleaved', () => {
    expect(batches.length).toBeGreaterThan(0)
    const priorAfter = {}

    for (const { file, data: batch } of batches) {
      expect(batch.schemaVersion).toBe(1)
      expect(batch.batchId).toBe(file.replace(/\.json$/, ''))
      expect(batch.officialLanguages).toEqual(officialLanguages)
      expect(officialLanguages).toContain(batch.sourceLanguage)

      for (const kind of ideaKinds) {
        const target = batch.targets[kind]
        const entries = batch.entries[kind]
        const ids = entries.map((entry) => entry.id)
        const expectedIds = Array.from(
          { length: target.added },
          (_, index) => ideaIdFromNumber(target.before + index)
        )

        expect(target.before + target.added).toBe(target.after)
        expect(ideaIdNumber(target.firstId)).toBe(target.before)
        expect(ideaIdNumber(target.lastId) + 1).toBe(target.after)
        expect(ids).toEqual(expectedIds)
        expect(new Set(ids).size).toBe(ids.length)
        if (priorAfter[kind] !== undefined) expect(target.before).toBe(priorAfter[kind])
        priorAfter[kind] = target.after

        const counts = new Map(batch.categories.map((category) => [category, 0]))
        let priorCategory = ''
        let runLength = 0

        for (const entry of entries) {
          expect(Object.keys(entry).sort()).toEqual(['category', 'id'])
          expect(batch.categories).toContain(entry.category)
          counts.set(entry.category, counts.get(entry.category) + 1)

          runLength = entry.category === priorCategory ? runLength + 1 : 1
          priorCategory = entry.category
          expect(runLength).toBeLessThanOrEqual(target.maxConsecutiveCategory)

          for (const code of officialLanguages) {
            expect(bundles[code][kind]).toHaveProperty(entry.id)
          }
        }

        const categoryCounts = [...counts.values()]
        expect(Math.max(...categoryCounts) - Math.min(...categoryCounts))
          .toBeLessThanOrEqual(target.maxCategoryImbalance)
      }
    }

    for (const kind of ideaKinds) {
      expect(Object.keys(bundles[officialLanguages[0]][kind])).toHaveLength(priorAfter[kind])
    }
  })

  it('enforces batch length budgets and synthetic-copy phrase guards', () => {
    for (const { data: batch } of batches) {
      for (const code of officialLanguages) {
        const forbiddenPhrases = batch.copyRules[code].forbiddenPhrases

        for (const kind of ideaKinds) {
          const limits = batch.limits[code][kind]

          for (const { id } of batch.entries[kind]) {
            const entry = bundles[code][kind][id]
            const visibleCopy = expectedFields[kind].map((field) => entry[field]).join('\n')

            for (const phrase of forbiddenPhrases) {
              expect(visibleCopy.toLocaleLowerCase()).not.toContain(phrase.toLocaleLowerCase())
            }

            for (const [field, [minimum, maximum]] of Object.entries(limits)) {
              expect(copyLength(entry[field])).toBeGreaterThanOrEqual(minimum)
              expect(copyLength(entry[field])).toBeLessThanOrEqual(maximum)
            }
          }
        }
      }
    }
  })

  it('rejects normalized duplicate visible fields inside every official pool', () => {
    for (const code of officialLanguages) {
      for (const kind of ideaKinds) {
        const seen = Object.fromEntries(expectedFields[kind].map((field) => [field, new Map()]))

        for (const [id, entry] of Object.entries(bundles[code][kind])) {
          for (const field of expectedFields[kind]) {
            const normalized = normalizedCopy(entry[field])
            expect(
              seen[field].get(normalized),
              `${code}.${kind}.${id}.${field} duplicates ${seen[field].get(normalized)}`
            ).toBeUndefined()
            seen[field].set(normalized, id)
          }
        }
      }
    }
  })

  it('keeps every managed prompt intact through the runtime line splitter', () => {
    for (const { data: batch } of batches) {
      for (const code of officialLanguages) {
        for (const { id } of batch.entries.miniBreakIdeas) {
          const text = bundles[code].miniBreakIdeas[id].text
          const lines = splitBreakPromptLines(text, code, 'hero')

          expect(lines.length).toBeGreaterThan(0)
          expect(lines.length).toBeLessThanOrEqual(4)
          expect(normalizedCopy(lines.join(' '))).toBe(normalizedCopy(text))
        }

        for (const { id } of batch.entries.longBreakIdeas) {
          const entry = bundles[code].longBreakIdeas[id]
          const titleLines = splitBreakPromptLines(entry.title, code, 'hero')
          const textLines = splitBreakPromptLines(entry.text, code, 'detail')

          expect(titleLines.length).toBeGreaterThan(0)
          expect(titleLines.length).toBeLessThanOrEqual(4)
          expect(textLines.length).toBeGreaterThan(0)
          expect(textLines.length).toBeLessThanOrEqual(10)
          expect(normalizedCopy(titleLines.join(' '))).toBe(normalizedCopy(entry.title))
          expect(normalizedCopy(textLines.join(' '))).toBe(normalizedCopy(entry.text))
        }
      }
    }
  })
})
