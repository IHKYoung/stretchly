import localeRegistryJson from './locales/registry.generated.json'

export type AppLanguage = string
export type TranslationTree = string | TranslationTree[] | { [key: string]: TranslationTree }

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
export const DESKTOP_LANGUAGE_CONFIGS = AVAILABLE_LANGUAGE_CONFIGS.filter(
  (config) => config.desktopReady,
).sort((left, right) => {
  if (left.code === defaultLanguage) {
    return -1
  }
  if (right.code === defaultLanguage) {
    return 1
  }
  return left.code.localeCompare(right.code)
})

function rawConfig(language?: string): AppLanguageConfig {
  return (language ? languageConfigs.get(language) : undefined) ?? defaultConfig
}

function resolvedConfig(language?: string): AppLanguageConfig {
  const seen = new Set<string>()
  let current = rawConfig(language)

  while (!current.desktopReady && !seen.has(current.code)) {
    seen.add(current.code)
    const next = languageConfigs.get(current.fallback)
    if (!next) {
      break
    }
    current = next
  }

  return current.desktopReady ? current : defaultConfig
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
