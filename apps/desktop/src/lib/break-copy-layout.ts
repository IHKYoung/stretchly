import type { AppLanguage } from '../i18n'

export type BreakPromptLineVariant = 'hero' | 'detail'

const SENTENCE_PATTERN = /[^.!?。！？；;]+(?:[.!?。！？；;]+|$)/g
const CJK_CLAUSE_PATTERN = /[^，、：；,;:]+(?:[，、：；,;:]+|$)/g
const LATIN_CLAUSE_PATTERN = /[^,:;]+(?:[:;,]+|$)/g

function normalizeBreakCopy(text: string) {
  return text.replace(/\s+/g, ' ').trim()
}

function cleanLine(text: string) {
  return text.replace(/\s+/g, ' ').trim()
}

function isCjkLanguage(language: AppLanguage) {
  return language.startsWith('zh') || language.startsWith('ja') || language.startsWith('ko')
}

function estimateLineUnits(text: string) {
  let units = 0

  for (const char of text) {
    if (/\s/.test(char)) {
      units += 0.3
    } else if (/[\u3040-\u30ff\u3400-\u4dbf\u4e00-\u9fff\uf900-\ufaff\uac00-\ud7af]/.test(char)) {
      units += 1
    } else if (/[.,!?;:，。！？；：、()[\]{}"'"“”‘’\-]/.test(char)) {
      units += 0.45
    } else {
      units += 0.58
    }
  }

  return units
}

function splitSegments(text: string, pattern: RegExp) {
  return text.match(pattern)?.map(cleanLine).filter(Boolean) ?? []
}

function joinSegments(left: string, right: string) {
  if (!left) {
    return right
  }

  if (
    /[A-Za-z0-9)"'”’.,!?;:]$/.test(left) &&
    /^[A-Za-z0-9("“‘]/.test(right)
  ) {
    return `${left} ${right}`
  }

  return `${left}${right}`
}

function maxUnitsFor(language: AppLanguage, variant: BreakPromptLineVariant) {
  if (isCjkLanguage(language)) {
    return variant === 'hero' ? 10 : 16
  }

  return variant === 'hero' ? 28 : 40
}

function packClauses(clauses: string[], maxUnits: number) {
  const lines: string[] = []
  let current = ''

  for (const clause of clauses) {
    const next = joinSegments(current, clause)
    if (!current || estimateLineUnits(next) <= maxUnits) {
      current = next
      continue
    }

    lines.push(cleanLine(current))
    current = clause
  }

  if (current) {
    lines.push(cleanLine(current))
  }

  return lines
}

export function splitBreakPromptLines(
  text: string,
  language: AppLanguage,
  variant: BreakPromptLineVariant,
) {
  const normalized = normalizeBreakCopy(text)
  if (!normalized) {
    return []
  }

  const maxUnits = maxUnitsFor(language, variant)
  const sentences = splitSegments(normalized, SENTENCE_PATTERN)
  const clausePattern = isCjkLanguage(language) ? CJK_CLAUSE_PATTERN : LATIN_CLAUSE_PATTERN

  return sentences.flatMap((sentence) => {
    if (estimateLineUnits(sentence) <= maxUnits) {
      return [sentence]
    }

    const clauses = splitSegments(sentence, clausePattern)
    if (clauses.length <= 1) {
      return [sentence]
    }

    return packClauses(clauses, maxUnits)
  })
}
