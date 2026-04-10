import localeRegistryJson from './locales/registry.generated.json'

export type AppLanguage = string
export type TranslationTree = string | TranslationTree[] | { [key: string]: TranslationTree }
export type BreakIdeaEntry = {
  title: string | null
  text: string
}

export type AppLanguageConfig = {
  code: string
  label: string
  nativeLabel: string
  fallback: string
  direction: 'ltr' | 'rtl'
  desktopReady: boolean
}

type LocaleRegistry = {
  defaultLanguage: string
  languages: AppLanguageConfig[]
  bundles: Record<string, TranslationTree>
}

const localeRegistry = localeRegistryJson as LocaleRegistry
const languageConfigs = new Map(
  localeRegistry.languages.map((config) => [config.code, config] as const),
)
const defaultLanguage = localeRegistry.defaultLanguage
const defaultConfig = languageConfigs.get(defaultLanguage) ?? localeRegistry.languages[0]

export const AVAILABLE_LANGUAGE_CONFIGS = [...localeRegistry.languages]
export const DESKTOP_LANGUAGE_CONFIGS = [...AVAILABLE_LANGUAGE_CONFIGS].sort((left, right) => {
  if (left.code === defaultLanguage) {
    return -1
  }
  if (right.code === defaultLanguage) {
    return 1
  }
  return left.label.localeCompare(right.label, 'en')
})

function canonicalLanguage(language?: string): string | undefined {
  switch (language) {
    case 'zh':
    case 'zh_CN':
    case 'zh-Hans':
    case 'zh-Hans-CN':
      return 'zh-CN'
    default:
      return language
  }
}

function rawConfig(language?: string): AppLanguageConfig {
  const canonical = canonicalLanguage(language)
  return (canonical ? languageConfigs.get(canonical) : undefined) ?? defaultConfig
}

function resolvedConfig(language?: string): AppLanguageConfig {
  return rawConfig(language)
}

function bundleChain(language?: string): TranslationTree[] {
  const chain: TranslationTree[] = []
  const seen = new Set<string>()
  let current: AppLanguageConfig | undefined = rawConfig(language)

  while (current && !seen.has(current.code)) {
    seen.add(current.code)
    const bundle = localeRegistry.bundles[current.code]
    if (bundle) {
      chain.push(bundle)
    }
    current = current.fallback ? languageConfigs.get(current.fallback) : undefined
  }

  const resolved = resolvedConfig(language)
  if (!seen.has(resolved.code)) {
    const bundle = localeRegistry.bundles[resolved.code]
    if (bundle) {
      chain.push(bundle)
    }
  }

  return chain.length > 0 ? chain : [localeRegistry.bundles[defaultLanguage]]
}

export function getLanguageConfig(language?: string): AppLanguageConfig {
  return resolvedConfig(language)
}

export function normalizeLanguage(language?: string): AppLanguage {
  return resolvedConfig(language).code
}

export function resolveUiLanguage(
  draftLanguage?: AppLanguage,
  persistedLanguage?: AppLanguage,
): AppLanguage {
  return normalizeLanguage(persistedLanguage ?? draftLanguage)
}

function lookup(bundle: TranslationTree, key: string): TranslationTree | null {
  let current: TranslationTree | undefined = bundle

  for (const segment of key.split('.')) {
    if (!current || typeof current === 'string' || Array.isArray(current)) {
      return null
    }
    current = current[segment]
  }

  return current ?? null
}

export function t(
  language: AppLanguage,
  key: string,
  vars: Record<string, string | number> = {},
): string {
  for (const bundle of bundleChain(language)) {
    const template = lookup(bundle, key)
    if (typeof template !== 'string') {
      continue
    }

    return Object.entries(vars).reduce((result, [name, value]) => {
      return result.split(`{{${name}}}`).join(String(value))
    }, template)
  }

  return key
}

export function tList(language: AppLanguage, key: string): string[] {
  for (const bundle of bundleChain(language)) {
    const value = lookup(bundle, key)
    if (Array.isArray(value) && value.every((entry) => typeof entry === 'string')) {
      return [...value]
    }
  }

  return []
}

export function tBreakIdeaList(
  language: AppLanguage,
  kind: 'microbreak' | 'longBreak',
): string[] {
  return tBreakIdeaEntries(language, kind).map((entry) => entry.text)
}

export function tBreakIdeaEntries(
  language: AppLanguage,
  kind: 'microbreak' | 'longBreak',
): BreakIdeaEntry[] {
  const key = kind === 'microbreak' ? 'miniBreakIdeas' : 'longBreakIdeas'

  for (const bundle of bundleChain(language)) {
    const value = lookup(bundle, key)
    if (!value || typeof value === 'string' || Array.isArray(value)) {
      continue
    }

    const prompts = Object.values(value)
      .map((entry) => {
        if (!entry || typeof entry === 'string' || Array.isArray(entry)) {
          return null
        }

        const text = entry.text
        if (typeof text !== 'string') {
          return null
        }

        return {
          title: typeof entry.title === 'string' ? entry.title : null,
          text,
        }
      })
      .filter((entry): entry is BreakIdeaEntry => Boolean(entry))

    if (prompts.length > 0) {
      return prompts
    }
  }

  return []
}

export function formatDuration(ms: number | null, language: AppLanguage): string {
  const normalizedLanguage = normalizeLanguage(language)

  if (!ms || ms <= 0) {
    return normalizedLanguage === 'zh-CN' ? '0分' : '0m'
  }

  const totalSeconds = Math.ceil(ms / 1000)
  const minutes = Math.floor(totalSeconds / 60)
  const seconds = totalSeconds % 60

  if (minutes === 0) {
    return t(normalizedLanguage, 'duration.seconds', { value: seconds })
  }

  if (seconds === 0) {
    return t(normalizedLanguage, 'duration.minutes', { value: minutes })
  }

  const paddedSeconds =
    normalizedLanguage === 'zh-CN'
      ? seconds.toString().padStart(2, '0')
      : `${seconds}s`

  return t(normalizedLanguage, 'duration.minutesSeconds', {

    minutes,
    seconds: paddedSeconds,
  })
}

export function formatCountdown(ms: number | null): string {
  const totalSeconds = Math.max(0, Math.ceil((ms ?? 0) / 1000))
  const hours = Math.floor(totalSeconds / 3600)
  const minutes = Math.floor((totalSeconds % 3600) / 60)
  const seconds = totalSeconds % 60

  if (hours > 0) {
    return [hours, minutes, seconds].map((value) => value.toString().padStart(2, '0')).join(':')
  }

  return [minutes, seconds].map((value) => value.toString().padStart(2, '0')).join(':')
}
