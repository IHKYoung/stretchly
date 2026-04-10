// Microbreak intervals: 20-20-20 rule (20 min), Pomodoro (25 min), common productivity windows
export const MICROBREAK_INTERVAL_PRESETS = [10, 20, 25, 30, 45] as const
// Microbreak durations: 20-20-20 rule (20 s), typical eye-rest and stretch windows
export const MICROBREAK_DURATION_PRESETS = [20, 30, 45, 60, 120] as const
// Long break every N microbreaks: Pomodoro classic = 4 cycles
export const LONGBREAK_EVERY_PRESETS = [2, 3, 4, 5, 6] as const
// Long break durations: Pomodoro short long-break (5–15 min), deep-rest windows
export const LONGBREAK_DURATION_PRESETS = [5, 10, 15, 20, 30] as const

export function commitDraftNumber(
  draft: string,
  currentValue: number,
  min: number,
  max: number,
): number {
  const normalized = draft.trim()
  if (!normalized) {
    return currentValue
  }

  const parsed = Number(normalized)
  if (!Number.isFinite(parsed)) {
    return currentValue
  }

  return Math.min(max, Math.max(min, Math.round(parsed)))
}

export function isNumericDraft(value: string): boolean {
  return /^\d*$/.test(value)
}
