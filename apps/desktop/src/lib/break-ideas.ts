import { tBreakIdeaEntries, type AppLanguage, type BreakIdeaEntry } from '../i18n'

export type BreakIdeaKind = 'microbreak' | 'longBreak'

export function pickBreakPromptIndex(
  kind: BreakIdeaKind,
  startedAtMs: number,
  sourceLength: number,
) {
  if (sourceLength <= 0) {
    return -1
  }

  // Keep one prompt stable during a break, but mix the timestamp so fixed 10m/30m
  // schedules do not collapse to the same modulo bucket forever.
  let hash = 2166136261
  const seed = `${kind}:${startedAtMs}`

  for (let index = 0; index < seed.length; index += 1) {
    hash ^= seed.charCodeAt(index)
    hash = Math.imul(hash, 16777619)
  }

  return (hash >>> 0) % sourceLength
}

export function pickBreakPrompt(
  language: AppLanguage,
  kind: BreakIdeaKind,
  startedAtMs: number,
): string {
  return pickBreakPromptEntry(language, kind, startedAtMs)?.text ?? ''
}

export function pickBreakPromptEntry(
  language: AppLanguage,
  kind: BreakIdeaKind,
  startedAtMs: number,
): BreakIdeaEntry | null {
  const source = tBreakIdeaEntries(language, kind)

  if (source.length === 0) {
    return null
  }

  return source[pickBreakPromptIndex(kind, startedAtMs, source.length)]
}
