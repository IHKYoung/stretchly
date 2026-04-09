import breakMessageCopyJson from './break-message-copy.json'
import { normalizeLanguage, type AppLanguage } from '@/i18n'

type BreakKind = 'microbreak' | 'longBreak'

type BreakMessageCopy = {
  clearedDetail: string
  manualAwaiting: string
  awaitingFinish: string
  actions: {
    resumeWork: string
    done: string
    later: string
    skip: string
  }
  defaultPrompt: Record<BreakKind, string>
  prompts: Record<BreakKind, string[]>
}

const DEFAULT_LANGUAGE = 'zh-CN'

const BREAK_MESSAGE_COPY = breakMessageCopyJson as Record<string, BreakMessageCopy>

function resolveBreakMessageCopy(language: AppLanguage): BreakMessageCopy {
  const normalized = normalizeLanguage(language)
  return BREAK_MESSAGE_COPY[normalized] ?? BREAK_MESSAGE_COPY[DEFAULT_LANGUAGE]
}

export function getBreakMessageCopy(language: AppLanguage): BreakMessageCopy {
  return resolveBreakMessageCopy(language)
}

export function getBreakPrompts(language: AppLanguage, kind: BreakKind): string[] {
  return [...resolveBreakMessageCopy(language).prompts[kind]]
}
