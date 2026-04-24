import breakIdeasRegistryJson from '../locales/break-ideas/registry.generated.json'
import { normalizeLanguage, type AppLanguage } from '../i18n'

export type BreakIdeaKind = 'microbreak' | 'longBreak'
export type BreakIdeaEntry = {
  title: string | null
  text: string
}
type BreakIdeaRawEntry = {
  title?: string
  text: string
}

type BreakIdeaSourceKey = 'miniBreakIdeas' | 'longBreakIdeas'
type BreakIdeaBundle = Partial<Record<BreakIdeaSourceKey, Record<string, BreakIdeaRawEntry>>>
type BreakIdeaRegistryLanguage = {
  code: string
  fallback: string
  tier: 'official' | 'legacy'
  available: boolean
}
type BreakIdeaRegistry = {
  defaultLanguage: string
  officialLanguages: string[]
  languages: BreakIdeaRegistryLanguage[]
  bundles: Record<string, BreakIdeaBundle>
}

const breakIdeasRegistry = breakIdeasRegistryJson as BreakIdeaRegistry
const breakIdeaConfigMap = new Map(
  breakIdeasRegistry.languages.map((config) => [config.code, config] as const),
)
const defaultBreakIdeaConfig =
  breakIdeaConfigMap.get(breakIdeasRegistry.defaultLanguage) ?? breakIdeasRegistry.languages[0] ?? null

function breakIdeaBundleKey(kind: BreakIdeaKind): BreakIdeaSourceKey {
  return kind === 'microbreak' ? 'miniBreakIdeas' : 'longBreakIdeas'
}

function breakIdeaBundleChain(language: AppLanguage): BreakIdeaBundle[] {
  if (!defaultBreakIdeaConfig) {
    return []
  }

  const chain: BreakIdeaBundle[] = []
  const seen = new Set<string>()
  let current: BreakIdeaRegistryLanguage | undefined =
    breakIdeaConfigMap.get(normalizeLanguage(language)) ??
    breakIdeaConfigMap.get(breakIdeasRegistry.defaultLanguage) ??
    defaultBreakIdeaConfig

  while (current && !seen.has(current.code)) {
    seen.add(current.code)

    const bundle = breakIdeasRegistry.bundles[current.code]
    if (bundle) {
      chain.push(bundle)
    }

    current = current.fallback ? breakIdeaConfigMap.get(current.fallback) : undefined
  }

  if (!seen.has(defaultBreakIdeaConfig.code)) {
    const bundle = breakIdeasRegistry.bundles[defaultBreakIdeaConfig.code]
    if (bundle) {
      chain.push(bundle)
    }
  }

  return chain
}

export function breakIdeaEntries(
  language: AppLanguage,
  kind: BreakIdeaKind,
): BreakIdeaEntry[] {
  const sourceKey = breakIdeaBundleKey(kind)

  for (const bundle of breakIdeaBundleChain(language)) {
    const source = bundle[sourceKey]
    if (!source || typeof source !== 'object') {
      continue
    }

    const prompts = Object.values(source)
      .map((entry) => {
        if (!entry || typeof entry.text !== 'string') {
          return null
        }

        return {
          title: typeof entry.title === 'string' ? entry.title : null,
          text: entry.text,
        }
      })
      .filter((entry): entry is BreakIdeaEntry => Boolean(entry))

    if (prompts.length > 0) {
      return prompts
    }
  }

  return []
}

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

export function rotateBreakPromptEntries(
  language: AppLanguage,
  kind: BreakIdeaKind,
  startedAtMs: number,
): BreakIdeaEntry[] {
  const source = breakIdeaEntries(language, kind)

  if (source.length <= 1) {
    return source
  }

  const startIndex = pickBreakPromptIndex(kind, startedAtMs, source.length)

  if (kind === 'microbreak') {
    return [source[startIndex]]
  }

  return source.map((_, index) => source[(startIndex + index) % source.length])
}

export function pickBreakPromptEntry(
  language: AppLanguage,
  kind: BreakIdeaKind,
  startedAtMs: number,
): BreakIdeaEntry | null {
  return rotateBreakPromptEntries(language, kind, startedAtMs)[0] ?? null
}
