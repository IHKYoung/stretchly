import en from './locales/en.json'
import zhCn from './locales/zh-CN.json'

export type AppLanguage = 'zh-CN' | 'en'

const bundles = {
  'zh-CN': zhCn,
  en,
} as const

type TranslationTree = string | { [key: string]: TranslationTree }

export function normalizeLanguage(language?: string): AppLanguage {
  return language === 'en' ? 'en' : 'zh-CN'
}

function lookup(bundle: TranslationTree, key: string): string | null {
  let current: TranslationTree | undefined = bundle

  for (const segment of key.split('.')) {
    if (!current || typeof current === 'string') {
      return null
    }
    current = current[segment]
  }

  return typeof current === 'string' ? current : null
}

export function t(
  language: AppLanguage,
  key: string,
  vars: Record<string, string | number> = {},
): string {
  const template = lookup(bundles[normalizeLanguage(language)], key) ?? key
  return Object.entries(vars).reduce((result, [name, value]) => {
    return result.split(`{{${name}}}`).join(String(value))
  }, template)
}

export function formatDuration(ms: number | null, language: AppLanguage): string {
  if (!ms || ms <= 0) {
    return normalizeLanguage(language) === 'zh-CN' ? '0分' : '0m'
  }

  const totalSeconds = Math.ceil(ms / 1000)
  const minutes = Math.floor(totalSeconds / 60)
  const seconds = totalSeconds % 60

  if (minutes === 0) {
    return t(language, 'duration.seconds', { value: seconds })
  }

  if (seconds === 0) {
    return t(language, 'duration.minutes', { value: minutes })
  }

  const paddedSeconds = normalizeLanguage(language) === 'zh-CN'
    ? seconds.toString().padStart(2, '0')
    : `${seconds}s`

  return t(language, 'duration.minutesSeconds', {
    minutes,
    seconds: paddedSeconds,
  })
}
