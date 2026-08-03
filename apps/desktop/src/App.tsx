import { invoke } from '@tauri-apps/api/core'
import { ArrowLeft, ChevronRight } from 'lucide-react'
import { useEffect, useEffectEvent, useRef, useState, type ChangeEvent, type ReactNode } from 'react'

import { Button } from '@/components/ui/button'
import {
  LONG_BREAK_PROMPT_HOLD_MS,
  LONG_BREAK_PROMPT_SWITCH_GAP_MS,
  rotateBreakPromptEntries,
} from '@/lib/break-ideas'
import { splitBreakPromptLines } from '@/lib/break-copy-layout'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { SegmentedControl } from '@/components/ui/segmented-control'
import { Switch } from '@/components/ui/switch'
import { Textarea } from '@/components/ui/textarea'
import {
  BREAK_BACKDROP_OPTIONS,
  BREAK_SOUND_OPTIONS,
  getBreakScene,
  getBreakSoundUrl,
  prepareCustomBackdrop,
  resolveBreakContrastMode,
  type BreakBackdrop,
  type BreakContrastMode,
  type BreakKind,
  type BreakSound,
} from '@/lib/break-prompt'
import {
  commitDraftNumber,
  isNumericDraft,
  LONGBREAK_DURATION_PRESETS,
  LONGBREAK_EVERY_PRESETS,
  matchRhythmProfile,
  MICROBREAK_DURATION_PRESETS,
  MICROBREAK_INTERVAL_PRESETS,
  rhythmProfilePatch,
  type RhythmProfileId,
} from '@/lib/settings-controls'
import { cn } from '@/lib/utils'
import {
  DESKTOP_LANGUAGE_CONFIGS,
  formatCountdown,
  formatDuration,
  getLanguageConfig,
  normalizeLanguage,
  resolveUiLanguage,
  t,
  type AppLanguage,
} from './i18n'

type AppExclusionRule = 'pause' | 'resume'
type ReminderMode = 'smart' | 'forced'
type TargetScreen = 'primary' | 'cursor'
type SettingsRoute =
  | 'overview'
  | 'rhythm'
  | 'reminders'
  | 'appearance'
  | 'automation'
  | 'system'
type PreviewRuntimeMode = 'default' | 'paused' | 'focus'

const PREVIEW_BREAK_OFFSET_MS = 6_000
const PREVIEW_BREAK_STARTED_AT_MS = Date.now() - PREVIEW_BREAK_OFFSET_MS

type PauzaSettings = {
  language: AppLanguage
  microbreakEnabled: boolean
  microbreakIntervalMinutes: number
  microbreakDurationSeconds: number
  microbreakNotificationEnabled: boolean
  microbreakNotificationSeconds: number
  microbreakAllowPostpone: boolean
  microbreakPostponeMinutes: number
  microbreakPostponesLimit: number
  reminderMode: ReminderMode
  microbreakManualFinish: boolean
  longBreakEnabled: boolean
  longBreakEvery: number
  longBreakDurationMinutes: number
  longBreakNotificationEnabled: boolean
  longBreakNotificationSeconds: number
  longBreakAllowPostpone: boolean
  longBreakPostponeMinutes: number
  longBreakPostponesLimit: number
  longBreakManualFinish: boolean
  naturalBreaks: boolean
  naturalBreakResetMinutes: number
  monitorDnd: boolean
  appExclusionsEnabled: boolean
  appExclusionRule: AppExclusionRule
  appExclusionCommands: string
  fullscreen: boolean
  breakBackdrop: BreakBackdrop
  breakCustomBackdropLabel: string | null
  breakCustomBackdropDataUrl: string | null
  breakIdeasEnabled: boolean
  showBreaksOnAllScreens: boolean
  targetScreen: TargetScreen
  currentTimeInBreaks: boolean
  showTimeToBreakInTray: boolean
  microbreakStartSound: BreakSound
  microbreakEndSound: BreakSound
  longBreakStartSound: BreakSound
  longBreakEndSound: BreakSound
  breakSoundVolume: number
  showTrayMenuInStrictMode: boolean
  revealSettingsShortcut: string
  focus45Shortcut: string
  pauseToggleShortcut: string
  pause30Shortcut: string
  pause60Shortcut: string
  pause120Shortcut: string
  pause300Shortcut: string
  skipNextScheduledShortcut: string
  skipNextMicrobreakShortcut: string
  skipNextLongBreakShortcut: string
  resetBreaksShortcut: string
}

type CurrentBreakSnapshot = {
  kind: BreakKind
  title: string
  detail: string
  startedAtMs: number
  endsAtMs: number
  durationMs: number
  strictMode: boolean
  manualAwaiting: boolean
  canPostpone: boolean
  canSkip: boolean
  showClock: boolean
}

type DesktopSnapshot = {
  productName: string
  runtime: string
  platform: string
  appVersion: string
  autostartEnabled: boolean
  settings: PauzaSettings
  status: string
  statusDetail: string
  nextBreakKind: BreakKind | null
  nextBreakDueMs: number | null
  nextBreakInMs: number | null
  nextBreakWaitRemainingMs: number | null
  currentBreak: CurrentBreakSnapshot | null
  pauseUntilMs: number | null
  pausedIndefinitely: boolean
  focusUntilMs: number | null
  idleMs: number
  dndActive: boolean
  appExclusionActive: boolean
  appExclusionMatch: string | null
  lastAction: string
}

type CommandFn = (
  action: string,
  command: string,
  args?: Record<string, unknown>,
  previewTransform?: (current: DesktopSnapshot) => DesktopSnapshot,
) => Promise<void>

function defaultSettings(): PauzaSettings {
  return {
    language: 'zh-CN',
    microbreakEnabled: true,
    microbreakIntervalMinutes: 20,
    microbreakDurationSeconds: 20,
    microbreakNotificationEnabled: true,
    microbreakNotificationSeconds: 10,
    microbreakAllowPostpone: true,
    microbreakPostponeMinutes: 2,
    microbreakPostponesLimit: 1,
    reminderMode: 'smart',
    microbreakManualFinish: false,
    longBreakEnabled: true,
    longBreakEvery: 3,
    longBreakDurationMinutes: 5,
    longBreakNotificationEnabled: true,
    longBreakNotificationSeconds: 30,
    longBreakAllowPostpone: true,
    longBreakPostponeMinutes: 5,
    longBreakPostponesLimit: 1,
    longBreakManualFinish: false,
    naturalBreaks: true,
    naturalBreakResetMinutes: 5,
    monitorDnd: true,
    appExclusionsEnabled: false,
    appExclusionRule: 'pause',
    appExclusionCommands: '',
    fullscreen: false,
    breakBackdrop: 'paper',
    breakCustomBackdropLabel: null,
    breakCustomBackdropDataUrl: null,
    breakIdeasEnabled: true,
    showBreaksOnAllScreens: true,
    targetScreen: 'primary',
    currentTimeInBreaks: false,
    showTimeToBreakInTray: true,
    microbreakStartSound: 'silence',
    microbreakEndSound: 'crystal-glass',
    longBreakStartSound: 'silence',
    longBreakEndSound: 'crystal-glass',
    breakSoundVolume: 100,
    showTrayMenuInStrictMode: false,
    revealSettingsShortcut: 'CmdOrCtrl+Shift+P',
    focus45Shortcut: 'CmdOrCtrl+Shift+F',
    pauseToggleShortcut: '',
    pause30Shortcut: '',
    pause60Shortcut: '',
    pause120Shortcut: '',
    pause300Shortcut: '',
    skipNextScheduledShortcut: '',
    skipNextMicrobreakShortcut: '',
    skipNextLongBreakShortcut: '',
    resetBreaksShortcut: '',
  }
}

function previewSnapshot(): DesktopSnapshot {
  const settings = defaultSettings()
  const language = settings.language
  const breakMode = isBreakWindow()
  const previewMode = breakMode ? 'default' : previewRuntimeMode()
  const previewNext = previewNextBreak(settings)
  const currentBreakDurationMs = settings.microbreakDurationSeconds * 1_000
  const previewBreakStartedAtMs = PREVIEW_BREAK_STARTED_AT_MS
  const previewBreakEndsAtMs = previewBreakStartedAtMs + currentBreakDurationMs
  const pauseDurationMs = 30 * 60_000
  const focusDurationMs = 45 * 60_000
  const pauseUntilMs = previewMode === 'paused' ? Date.now() + pauseDurationMs : null
  const focusUntilMs = previewMode === 'focus' ? Date.now() + focusDurationMs : null
  const previewStatus =
    previewMode === 'paused'
      ? {
          status: t(language, 'runtime.break.status.pausedTitle'),
          statusDetail: t(language, 'runtime.break.status.pausedDetail', {
            duration: formatDuration(pauseDurationMs, language),
          }),
          lastAction: t(language, 'ui.previewAction.pause'),
        }
      : previewMode === 'focus'
        ? {
            status: t(language, 'runtime.break.status.focusTitle'),
            statusDetail: t(language, 'runtime.break.status.focusDetail', {
              duration: formatDuration(focusDurationMs, language),
            }),
            lastAction: t(language, 'ui.previewAction.focus'),
          }
        : {
            status: t(language, 'ui.preview.status'),
            statusDetail: t(language, 'ui.preview.detail'),
            lastAction: t(language, 'ui.preview.action'),
          }

  return {
    productName: 'Pauza',
    runtime: 'Browser preview',
    platform: 'preview',
    appVersion: '0.1.0',
    autostartEnabled: false,
    settings,
    status: previewStatus.status,
    statusDetail: previewStatus.statusDetail,
    nextBreakKind: previewNext.nextBreakKind,
    nextBreakDueMs: previewNext.nextBreakDueMs,
    nextBreakInMs: previewNext.nextBreakInMs,
    nextBreakWaitRemainingMs: null,
    currentBreak: breakMode
      ? {
          kind: 'microbreak',
          title: t(language, 'runtime.break.microbreak.title'),
          detail: t(language, 'runtime.break.microbreak.detail'),
          startedAtMs: previewBreakStartedAtMs,
          endsAtMs: previewBreakEndsAtMs,
          durationMs: currentBreakDurationMs,
          strictMode: settings.reminderMode === 'forced',
          manualAwaiting: false,
          canPostpone: settings.reminderMode !== 'forced',
          canSkip: settings.reminderMode !== 'forced',
          showClock: true,
        }
      : null,
    pauseUntilMs,
    pausedIndefinitely: false,
    focusUntilMs,
    idleMs: 0,
    dndActive: false,
    appExclusionActive: false,
    appExclusionMatch: null,
    lastAction: previewStatus.lastAction,
  }
}

function hasTauriRuntime() {
  return Boolean((window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__)
}

function isBreakWindow() {
  return new URLSearchParams(window.location.search).get('window') === 'break'
}

function previewRuntimeMode(): PreviewRuntimeMode {
  const preview = new URLSearchParams(window.location.search).get('preview')

  if (preview === 'paused' || preview === 'focus') {
    return preview
  }

  return 'default'
}

function previewNextBreak(settings: PauzaSettings) {
  const now = Date.now()

  if (settings.microbreakEnabled) {
    const nextBreakInMs = Math.max(settings.microbreakIntervalMinutes * 60_000, 60_000)
    return {
      nextBreakKind: 'microbreak' as BreakKind,
      nextBreakDueMs: now + nextBreakInMs,
      nextBreakInMs,
    }
  }

  if (settings.longBreakEnabled) {
    const nextBreakInMs = Math.max(
      settings.microbreakIntervalMinutes * settings.longBreakEvery * 60_000,
      60_000,
    )
    return {
      nextBreakKind: 'longBreak' as BreakKind,
      nextBreakDueMs: now + nextBreakInMs,
      nextBreakInMs,
    }
  }

  return {
    nextBreakKind: null,
    nextBreakDueMs: null,
    nextBreakInMs: null,
  }
}

function clockLabel(language: AppLanguage, value = Date.now()) {
  const normalizedLanguage = normalizeLanguage(language)

  return new Intl.DateTimeFormat(normalizedLanguage === 'zh-CN' ? 'zh-CN' : 'en-US', {
    hour: '2-digit',
    minute: '2-digit',
  }).format(value)
}

function playBreakSound(sound: BreakSound, volume: number) {
  const soundUrl = getBreakSoundUrl(sound)
  if (!soundUrl || volume <= 0) {
    return
  }

  const audio = new Audio(soundUrl)
  audio.volume = volume / 100
  void audio.play().catch(() => {})
}

/* ------------------------------------------------------------------ */
/*  Settings UI primitives                                            */
/* ------------------------------------------------------------------ */

function CompactNumber({
  value,
  min,
  max,
  suffix,
  step = 1,
  onChange,
}: {
  value: number
  min: number
  max: number
  suffix: string
  step?: number
  onChange: (next: number) => void
}) {
  const [draft, setDraft] = useState(String(value))
  const draftValue = commitDraftNumber(draft, value, min, max)

  useEffect(() => {
    setDraft(String(value))
  }, [value])

  const commitDraft = () => {
    const nextValue = commitDraftNumber(draft, value, min, max)
    setDraft(String(nextValue))
    if (nextValue !== value) {
      onChange(nextValue)
    }
  }

  const stepDraft = (delta: number) => {
    const nextValue = Math.min(max, Math.max(min, draftValue + delta))
    setDraft(String(nextValue))
    if (nextValue !== value) {
      onChange(nextValue)
    }
  }

  return (
    <div className="flex items-center gap-1.5">
      <div className="flex h-7 items-center overflow-hidden rounded-md bg-black/[0.04]">
        <button
          type="button"
          onMouseDown={(event) => event.preventDefault()}
          onClick={() => stepDraft(-step)}
          disabled={draftValue <= min}
          className="flex h-full w-6 items-center justify-center border-r border-black/[0.06] text-[13px] text-muted-foreground transition-colors hover:bg-black/[0.04] hover:text-foreground disabled:opacity-30"
          aria-label="decrease"
        >
          −
        </button>
        <input
          type="text"
          inputMode="numeric"
          pattern="[0-9]*"
          value={draft}
          onChange={(event) => {
            const nextDraft = event.target.value
            if (isNumericDraft(nextDraft)) {
              setDraft(nextDraft)
            }
          }}
          onBlur={commitDraft}
          onKeyDown={(event) => {
            if (event.key === 'Enter') {
              event.preventDefault()
              commitDraft()
              event.currentTarget.blur()
            }
            if (event.key === 'Escape') {
              event.preventDefault()
              setDraft(String(value))
              event.currentTarget.blur()
            }
          }}
          className="w-12 bg-transparent px-1 text-center text-[13px] font-medium text-foreground outline-none"
        />
        <button
          type="button"
          onMouseDown={(event) => event.preventDefault()}
          onClick={() => stepDraft(step)}
          disabled={draftValue >= max}
          className="flex h-full w-6 items-center justify-center border-l border-black/[0.06] text-[13px] text-muted-foreground transition-colors hover:bg-black/[0.04] hover:text-foreground disabled:opacity-30"
          aria-label="increase"
        >
          +
        </button>
      </div>
      <span className="settings-caption whitespace-nowrap text-muted-foreground">{suffix}</span>
    </div>
  )
}

function PresetChipGroup({
  ariaLabel,
  customLabel,
  value,
  options,
  min,
  max,
  onChange,
}: {
  ariaLabel: string
  customLabel: string
  value: number
  options: readonly number[]
  min: number
  max: number
  onChange: (next: number) => void
}) {
  const isPreset = options.includes(value)
  const [draft, setDraft] = useState(String(value))
  const [customEditing, setCustomEditing] = useState(false)

  useEffect(() => {
    setDraft(String(value))
  }, [value])

  const commitDraft = () => {
    const nextValue = commitDraftNumber(draft, value, min, max)
    setDraft(String(nextValue))
    if (nextValue !== value) {
      onChange(nextValue)
    }
  }

  return (
    <div role="group" aria-label={ariaLabel} className="flex flex-wrap items-center gap-1.5">
      {options.map((option) => {
        const active = option === value

        return (
          <button
            key={option}
            type="button"
            aria-pressed={active}
            className={cn(
              'min-w-[34px] rounded-lg px-2.5 py-1 text-center text-[13px] font-medium tabular-nums transition-all duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
              active
                ? 'bg-foreground text-background'
                : 'bg-black/[0.06] text-muted-foreground hover:bg-black/[0.08] hover:text-foreground',
            )}
            onClick={() => onChange(option)}
          >
            {option}
          </button>
        )
      })}
      <input
        type="text"
        inputMode="numeric"
        pattern="[0-9]*"
        value={isPreset && !customEditing ? '' : draft}
        placeholder={customLabel}
        onFocus={() => {
          setCustomEditing(true)
          if (isPreset) {
            setDraft('')
          }
        }}
        onChange={(event) => {
          const nextDraft = event.target.value
          if (isNumericDraft(nextDraft)) {
            setDraft(nextDraft)
          }
        }}
        onBlur={() => {
          commitDraft()
          setCustomEditing(false)
        }}
        onKeyDown={(event) => {
          if (event.key === 'Enter') {
            event.preventDefault()
            commitDraft()
            event.currentTarget.blur()
          }
          if (event.key === 'Escape') {
            event.preventDefault()
            setDraft(String(value))
            event.currentTarget.blur()
          }
        }}
        aria-label={`${ariaLabel}: ${customLabel}`}
        className={cn(
          'w-14 rounded-lg border py-1 text-center text-[13px] font-medium tabular-nums outline-none transition-all placeholder:text-muted-foreground/55',
          isPreset && !customEditing
            ? 'border-transparent bg-transparent text-muted-foreground hover:border-black/[0.06]'
            : 'border-black/[0.08] bg-white text-foreground shadow-sm',
          'focus:border-ring focus:bg-white focus:text-foreground focus:shadow-sm',
        )}
      />
    </div>
  )
}

function PresetChipRow({
  label,
  unit,
  ariaLabel,
  customLabel,
  value,
  options,
  min,
  max,
  onChange,
}: {
  label: string
  unit: string
  ariaLabel: string
  customLabel: string
  value: number
  options: readonly number[]
  min: number
  max: number
  onChange: (next: number) => void
}) {
  return (
    <div className="flex items-center gap-3">
      <span className="settings-control-label w-14 shrink-0 text-right text-muted-foreground">
        {label}
      </span>
      <PresetChipGroup
        ariaLabel={ariaLabel}
        customLabel={customLabel}
        value={value}
        options={options}
        min={min}
        max={max}
        onChange={onChange}
      />
      <span className="settings-caption shrink-0 text-muted-foreground">{unit}</span>
    </div>
  )
}

function SchedulePresetCard({
  label,
  switchLabel,
  checked,
  onCheckedChange,
  children,
}: {
  label: string
  switchLabel: string
  checked: boolean
  onCheckedChange: (next: boolean) => void
  children: ReactNode
}) {
  return (
    <div className="overflow-hidden rounded-xl bg-white shadow-[0_0_0_0.5px_rgba(0,0,0,0.06),0_1px_2px_rgba(0,0,0,0.04)]">
      <div className="flex items-center justify-between gap-4 border-b border-black/[0.06] px-4 py-3">
        <p className="settings-group-title text-foreground">{label}</p>
        <Switch checked={checked} onCheckedChange={onCheckedChange} aria-label={switchLabel} />
      </div>
      <div
        className={cn(
          'space-y-3 px-4 py-3 transition-opacity duration-200',
          !checked && 'pointer-events-none opacity-40',
        )}
      >
        {children}
      </div>
    </div>
  )
}

function SectionLabel({ children }: { children: ReactNode }) {
  return (
    <h3 className="settings-section-title mb-2 px-1 text-muted-foreground">{children}</h3>
  )
}

function SettingsCard({ children }: { children: ReactNode }) {
  return (
    <div className="overflow-hidden rounded-xl bg-white shadow-[0_0_0_0.5px_rgba(0,0,0,0.06),0_1px_2px_rgba(0,0,0,0.04)]">
      <div className="divide-y divide-black/[0.06]">{children}</div>
    </div>
  )
}

function SettingsRow({
  label,
  detail,
  children,
}: {
  label: string
  detail?: string
  children: ReactNode
}) {
  return (
    <div className="flex min-h-[46px] items-center gap-4 px-4 py-2.5">
      <div className="min-w-0 flex-1">
        <p className="settings-row-title text-foreground">{label}</p>
        {detail ? (
          <p className="settings-detail mt-0.5 text-muted-foreground">{detail}</p>
        ) : null}
      </div>
      <div className="flex w-[200px] shrink-0 items-center justify-end gap-2">{children}</div>
    </div>
  )
}

function SettingsLinkRow({
  label,
  detail,
  onClick,
}: {
  label: string
  detail: string
  onClick: () => void
}) {
  return (
    <button
      type="button"
      className="flex min-h-[56px] w-full items-center gap-4 px-4 py-2.5 text-left transition-colors hover:bg-black/[0.025] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring"
      onClick={onClick}
    >
      <div className="min-w-0 flex-1">
        <p className="settings-link-title text-foreground">{label}</p>
        <p className="settings-detail mt-0.5 truncate text-muted-foreground">{detail}</p>
      </div>
      <ChevronRight className="h-4 w-4 shrink-0 text-muted-foreground/70" aria-hidden="true" />
    </button>
  )
}

function RhythmProfilePicker({
  value,
  options,
  ariaLabel,
  customLabel,
  onChange,
}: {
  value: RhythmProfileId | 'custom'
  options: Array<{ value: RhythmProfileId; label: string }>
  ariaLabel: string
  customLabel: string
  onChange: (next: RhythmProfileId) => void
}) {
  return (
    <div>
      <div
        role="radiogroup"
        aria-label={ariaLabel}
        className="grid grid-cols-3 gap-1 rounded-lg bg-black/[0.05] p-1"
      >
        {options.map((option) => {
          const active = option.value === value

          return (
            <button
              key={option.value}
              type="button"
              role="radio"
              aria-checked={active}
              className={cn(
                'min-h-8 rounded-md px-2 py-1.5 text-[13px] font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
                active
                  ? 'bg-white font-semibold text-foreground shadow-[0_1px_2px_rgba(0,0,0,0.06),0_0_0_0.5px_rgba(0,0,0,0.04)]'
                  : 'text-muted-foreground hover:text-foreground',
              )}
              onClick={() => onChange(option.value)}
            >
              {option.label}
            </button>
          )
        })}
      </div>
      {value === 'custom' ? (
        <p className="settings-detail mt-2 font-medium text-foreground">{customLabel}</p>
      ) : null}
    </div>
  )
}

type BreakPromptCopy = {
  eyebrow: string
  title: string
  body: string
}

type BreakPromptTypingPhase = 'idle' | 'title' | 'body'

function getBreakTypewriterDelay(char: string, phase: Exclude<BreakPromptTypingPhase, 'idle'>) {
  if (char === '\n') {
    return 0
  }

  if (/[，、]/.test(char)) {
    return phase === 'body' ? 104 : 148
  }

  if (/[。！？.!?]/.test(char)) {
    return phase === 'body' ? 156 : 220
  }

  if (/[:：；;]/.test(char)) {
    return phase === 'body' ? 124 : 176
  }

  return phase === 'body' ? 28 : 44
}

function buildBreakPromptCopy(
  copy: BreakPromptCopy,
  language: AppLanguage,
): BreakPromptCopy {
  return {
    eyebrow: copy.eyebrow,
    title: splitBreakPromptLines(copy.title, language, 'hero').join('\n'),
    body: splitBreakPromptLines(copy.body, language, 'detail').join('\n'),
  }
}

function resolveBreakPromptSequence({
  currentBreak,
  language,
  settings,
}: {
  currentBreak: CurrentBreakSnapshot | null
  language: AppLanguage
  settings: PauzaSettings
}): BreakPromptCopy[] {
  if (!currentBreak) {
    return [buildBreakPromptCopy({
      eyebrow: '',
      title: '',
      body: t(language, 'ui.breakCopy.clearedDetail'),
    }, language)]
  }

  if (currentBreak.manualAwaiting) {
    return [buildBreakPromptCopy({
      eyebrow: currentBreak.title,
      title: t(language, 'ui.breakCopy.manualAwaiting'),
      body: '',
    }, language)]
  }

  const promptEntries = settings.breakIdeasEnabled
    ? rotateBreakPromptEntries(language, currentBreak.kind, currentBreak.startedAtMs)
    : []
  const fallbackBody = t(language, `ui.breakCopy.defaultPrompt.${currentBreak.kind}`)

  if (promptEntries.length > 0) {
    return promptEntries.map((entry) =>
      buildBreakPromptCopy(
        entry.title
          ? {
              eyebrow: entry.title === currentBreak.title ? '' : currentBreak.title,
              title: entry.title,
              body: entry.text,
            }
          : {
              eyebrow: currentBreak.title,
              title: entry.text,
              body: '',
            },
        language,
      ),
    )
  }

  return [buildBreakPromptCopy({
    eyebrow: currentBreak.title,
    title: fallbackBody,
    body: '',
  }, language)]
}

function initialAnimatedBreakPrompt({
  currentBreak,
  language,
  settings,
}: {
  currentBreak: CurrentBreakSnapshot | null
  language: AppLanguage
  settings: PauzaSettings
}): BreakPromptCopy {
  const initialPrompt =
    resolveBreakPromptSequence({
      currentBreak,
      language,
      settings,
    })[0] ?? { eyebrow: '', title: '', body: '' }
  const prefersReducedMotion =
    typeof window.matchMedia === 'function' &&
    window.matchMedia('(prefers-reduced-motion: reduce)').matches

  if (!currentBreak || currentBreak.manualAwaiting || prefersReducedMotion) {
    return initialPrompt
  }

  return {
    eyebrow: initialPrompt.eyebrow,
    title: '',
    body: '',
  }
}

function buildBreakPresentationPalette(
  contrastMode: BreakContrastMode,
  scene: ReturnType<typeof getBreakScene>,
) {
  if (contrastMode === 'dark') {
    return {
      overlayVeil:
        'radial-gradient(circle at 18% 24%, rgba(15,23,42,0.12) 0%, rgba(15,23,42,0) 36%), linear-gradient(180deg, rgba(8,15,28,0.34) 0%, rgba(8,15,28,0.14) 34%, rgba(8,15,28,0.46) 100%)',
      contentGlow:
        'radial-gradient(circle at 22% 32%, rgba(8,15,28,0.42) 0%, rgba(8,15,28,0.14) 38%, rgba(8,15,28,0) 68%)',
      textPrimary: '#f8fafc',
      textSecondary: 'rgba(241,245,249,0.82)',
      chipBackground: 'rgba(255,255,255,0.1)',
      chipBorder: 'rgba(255,255,255,0.18)',
      chipText: 'rgba(255,255,255,0.88)',
      textShadow: '0 14px 36px rgba(8,15,28,0.24)',
      bodyShadow: '0 10px 28px rgba(8,15,28,0.22)',
      actionBorder: 'rgba(255,255,255,0.18)',
      actionGlass: 'rgba(255,255,255,0.1)',
      actionGlassHover: 'rgba(255,255,255,0.16)',
      actionText: '#f8fafc',
      primaryText: '#08111f',
      countdownGlow: '0 16px 40px rgba(8,15,28,0.28)',
      meterTrack: 'rgba(255,255,255,0.2)',
      meterFill: scene.meterFill,
    }
  }

  return {
    overlayVeil:
      'radial-gradient(circle at 18% 24%, rgba(255,255,255,0.38) 0%, rgba(255,255,255,0.14) 34%, rgba(255,255,255,0) 66%), linear-gradient(180deg, rgba(255,255,255,0.18) 0%, rgba(255,255,255,0.02) 30%, rgba(248,250,252,0.26) 100%)',
    contentGlow:
      'radial-gradient(circle at 24% 28%, rgba(255,255,255,0.42) 0%, rgba(255,255,255,0.18) 30%, rgba(255,255,255,0) 64%)',
    textPrimary: '#0f172a',
    textSecondary: 'rgba(15,23,42,0.74)',
    chipBackground: 'rgba(255,255,255,0.42)',
    chipBorder: 'rgba(15,23,42,0.08)',
    chipText: 'rgba(15,23,42,0.74)',
    textShadow: '0 12px 26px rgba(255,255,255,0.16)',
    bodyShadow: '0 8px 20px rgba(255,255,255,0.12)',
    actionBorder: 'rgba(15,23,42,0.1)',
    actionGlass: 'rgba(255,255,255,0.32)',
    actionGlassHover: 'rgba(255,255,255,0.48)',
    actionText: '#0f172a',
    primaryText: '#ffffff',
    countdownGlow: '0 14px 32px rgba(255,255,255,0.18)',
    meterTrack: 'rgba(15,23,42,0.08)',
    meterFill: scene.meterFill,
  }
}

/* ------------------------------------------------------------------ */
/*  Break window                                                      */
/* ------------------------------------------------------------------ */

function BreakWindow({
  busyAction,
  currentBreak,
  language,
  runCommand,
  settings,
}: {
  busyAction: string | null
  currentBreak: CurrentBreakSnapshot | null
  language: AppLanguage
  runCommand: CommandFn
  settings: PauzaSettings
}) {
  const remaining = currentBreak ? Math.max(currentBreak.endsAtMs - Date.now(), 0) : 0
  const progress = currentBreak
    ? currentBreak.manualAwaiting
      ? 0
      : Math.max(0, Math.min(1, remaining / Math.max(currentBreak.durationMs, 1)))
    : 0
  const hasCustomBackdrop =
    settings.breakBackdrop === 'custom' && Boolean(settings.breakCustomBackdropDataUrl)
  const scene = getBreakScene(hasCustomBackdrop ? 'custom' : settings.breakBackdrop)
  const [contrastMode, setContrastMode] = useState<BreakContrastMode>(scene.contrastMode)
  const [animatedPrompt, setAnimatedPrompt] = useState<BreakPromptCopy>(() =>
    initialAnimatedBreakPrompt({
      currentBreak,
      language,
      settings,
    }),
  )
  const [typingPhase, setTypingPhase] = useState<BreakPromptTypingPhase>('idle')
  const lastEndSoundRef = useRef<string | null>(null)
  const endSoundTimerRef = useRef<number | null>(null)
  const breakKey = currentBreak ? `${currentBreak.kind}:${currentBreak.startedAtMs}` : null
  const titleLines = animatedPrompt.title ? animatedPrompt.title.split('\n') : []
  const bodyLines = animatedPrompt.body ? animatedPrompt.body.split('\n') : []
  const countdownText = formatCountdown(currentBreak?.manualAwaiting ? 0 : remaining)
  const palette = buildBreakPresentationPalette(contrastMode, scene)
  const backgroundStyle = hasCustomBackdrop
    ? {
        backgroundImage: `${palette.overlayVeil}, url("${settings.breakCustomBackdropDataUrl}")`,
        backgroundPosition: 'center',
        backgroundRepeat: 'no-repeat',
        backgroundSize: 'cover',
      }
    : {
        backgroundImage: scene.background,
      }

  useEffect(() => {
    let cancelled = false

    setContrastMode(scene.contrastMode)

    void resolveBreakContrastMode(settings.breakBackdrop, settings.breakCustomBackdropDataUrl).then(
      (nextMode) => {
        if (!cancelled) {
          setContrastMode(nextMode)
        }
      },
    )

    return () => {
      cancelled = true
    }
  }, [scene.contrastMode, settings.breakBackdrop, settings.breakCustomBackdropDataUrl])

  useEffect(() => {
    if (endSoundTimerRef.current !== null) {
      window.clearTimeout(endSoundTimerRef.current)
      endSoundTimerRef.current = null
    }

    if (!currentBreak || !hasTauriRuntime() || !breakKey) {
      return
    }

    const sound =
      currentBreak.kind === 'microbreak'
        ? settings.microbreakEndSound
        : settings.longBreakEndSound
    const playKey = `${breakKey}:end`

    const play = () => {
      if (lastEndSoundRef.current === playKey) {
        return
      }
      lastEndSoundRef.current = playKey
      playBreakSound(sound, settings.breakSoundVolume)
    }

    if (currentBreak.manualAwaiting) {
      play()
      return
    }

    endSoundTimerRef.current = window.setTimeout(
      play,
      Math.max(currentBreak.endsAtMs - Date.now(), 0),
    )

    return () => {
      if (endSoundTimerRef.current !== null) {
        window.clearTimeout(endSoundTimerRef.current)
        endSoundTimerRef.current = null
      }
    }
  }, [
    breakKey,
    currentBreak?.endsAtMs,
    currentBreak?.kind,
    currentBreak?.manualAwaiting,
    settings.breakSoundVolume,
    settings.longBreakEndSound,
    settings.microbreakEndSound,
  ])

  useEffect(() => {
    const promptSequence = resolveBreakPromptSequence({
      currentBreak,
      language,
      settings,
    })
    const initialPrompt = promptSequence[0] ?? { eyebrow: '', title: '', body: '' }
    const prefersReducedMotion =
      typeof window.matchMedia === 'function' &&
      window.matchMedia('(prefers-reduced-motion: reduce)').matches
    const timerHandles = new Set<number>()
    let cancelled = false

    const pause = (ms: number) =>
      new Promise<void>((resolve) => {
        const handle = window.setTimeout(() => {
          timerHandles.delete(handle)
          resolve()
        }, ms)
        timerHandles.add(handle)
      })

    const cleanupTimers = () => {
      for (const handle of timerHandles) {
        window.clearTimeout(handle)
      }
      timerHandles.clear()
    }

    if (!currentBreak || currentBreak.manualAwaiting || prefersReducedMotion) {
      setAnimatedPrompt(initialPrompt)
      setTypingPhase('idle')

      return () => {
        cancelled = true
        cleanupTimers()
      }
    }

    const typeField = async (
      phase: Exclude<BreakPromptTypingPhase, 'idle'>,
      text: string,
    ) => {
      if (!text) {
        return
      }

      setTypingPhase(phase)

      for (let length = 1; length <= text.length; length += 1) {
        if (cancelled) {
          return
        }

        const nextText = text.slice(0, length)
        setAnimatedPrompt((current) => ({
          ...current,
          [phase]: nextText,
        }))
        await pause(getBreakTypewriterDelay(text[length - 1], phase))
      }
    }

    void (async () => {
      for (let index = 0; !cancelled; index = (index + 1) % promptSequence.length) {
        const nextPrompt = promptSequence[index] ?? initialPrompt
        setAnimatedPrompt({
          eyebrow: nextPrompt.eyebrow,
          title: '',
          body: '',
        })

        await typeField('title', nextPrompt.title)
        if (cancelled) {
          return
        }

        await typeField('body', nextPrompt.body)
        if (cancelled) {
          return
        }

        setTypingPhase('idle')

        if (promptSequence.length <= 1) {
          return
        }

        await pause(LONG_BREAK_PROMPT_HOLD_MS)
        if (cancelled) {
          return
        }

        setAnimatedPrompt({ eyebrow: '', title: '', body: '' })
        await pause(LONG_BREAK_PROMPT_SWITCH_GAP_MS)
        if (cancelled) {
          return
        }
      }
    })()

    return () => {
      cancelled = true
      cleanupTimers()
    }
  }, [
    breakKey,
    currentBreak?.manualAwaiting,
    currentBreak?.title,
    language,
    settings.breakIdeasEnabled,
  ])

  return (
    <main className="relative min-h-screen overflow-hidden" style={backgroundStyle}>
      <div
        className="pointer-events-none absolute inset-0 opacity-30"
        style={{ backgroundImage: scene.texture }}
      />
      <div className="pointer-events-none absolute inset-0" style={{ background: palette.contentGlow }} />

      <section className="relative flex min-h-screen flex-col items-center justify-center px-8 py-12 sm:px-14 sm:py-16">
        {currentBreak?.showClock ? (
          <div
            className="absolute right-6 top-6 rounded-full border px-3 py-1.5 text-[11px] font-medium tracking-[0.16em] sm:right-8 sm:top-8"
            style={{
              color: palette.chipText,
              background: palette.chipBackground,
              borderColor: palette.chipBorder,
              backdropFilter: 'blur(18px)',
            }}
          >
            {clockLabel(language)}
          </div>
        ) : null}

        {/* Text content — centered column */}
        <div className="break-terminal-shell flex w-full max-w-[min(90vw,820px)] flex-col gap-4">
          <p
            className="break-terminal-prompt type-break"
            style={{
              color: palette.textSecondary,
              ['--terminal-line' as string]: palette.chipBorder,
            }}
          >
            <span className="break-terminal-prompt-mark" style={{ color: palette.textPrimary }}>
              Pauza&gt;
            </span>
          </p>

          <div className="break-terminal-output flex flex-col items-center gap-4 text-center">
            {animatedPrompt.eyebrow ? (
              <p
                className="type-break text-[12px] leading-6 tracking-[0.12em] sm:text-[13px]"
                style={{
                  color: palette.textSecondary,
                  textShadow: palette.bodyShadow,
                }}
              >
                {animatedPrompt.eyebrow}
              </p>
            ) : null}

            <div
                className={cn(
                  'type-break leading-[1.42]',
                  titleLines.length >= 4
                    ? 'text-[clamp(1.28rem,3vw,2.08rem)]'
                    : 'text-[clamp(1.5rem,3.5vw,2.55rem)]',
              )}
              style={{
                color: palette.textPrimary,
                textShadow: palette.textShadow,
              }}
            >
              {titleLines.map((line, index) => (
                <span
                  key={`title-${index}-${line}`}
                  className={cn('mx-auto block max-w-full text-balance', index > 0 && 'mt-[0.28em]')}
                >
                  {line}
                  {typingPhase === 'title' && index === titleLines.length - 1 ? (
                    <span
                      aria-hidden="true"
                      className="ml-[0.08em] inline-block h-[0.92em] w-[0.08em] animate-break-caret align-[-0.08em]"
                      style={{ background: palette.textPrimary }}
                    />
                  ) : null}
                </span>
              ))}
            </div>

            {bodyLines.length > 0 ? (
              <div
                className="type-break flex max-w-[min(88vw,44rem)] flex-col items-center text-[clamp(0.9rem,1.45vw,1.06rem)] leading-[1.9]"
                style={{
                  color: palette.textSecondary,
                  textShadow: palette.bodyShadow,
                }}
              >
                {bodyLines.map((line, index) => (
                  <p
                    key={`body-${index}-${line}`}
                    className={cn('max-w-full text-balance', index > 0 && 'mt-[0.18em]')}
                  >
                    {line}
                    {typingPhase === 'body' && index === bodyLines.length - 1 ? (
                      <span
                        aria-hidden="true"
                        className="ml-[0.08em] inline-block h-[0.92em] w-[0.08em] animate-break-caret align-[-0.08em]"
                        style={{ background: palette.textSecondary }}
                      />
                    ) : null}
                  </p>
                ))}
              </div>
            ) : null}
          </div>
        </div>

        {/* Countdown + meter + actions */}
        <div className="mt-12 flex w-full max-w-[min(86vw,480px)] flex-col items-center gap-5 text-center sm:mt-14">
          <div
            className="type-break animate-break-breathe text-[clamp(4rem,11vw,7.5rem)] leading-none tabular-nums"
            style={{
              color: palette.textPrimary,
              textShadow: palette.countdownGlow,
            }}
          >
            {countdownText}
          </div>

          <div
            className="h-[3px] w-full overflow-hidden rounded-full"
            style={{ background: palette.meterTrack }}
          >
            <div
              className="h-full rounded-full transition-[width] duration-700 ease-out"
              style={{
                width: `${Math.round(progress * 100)}%`,
                background: palette.meterFill,
              }}
            />
          </div>

          {currentBreak?.manualAwaiting ? (
            <p className="text-sm font-medium" style={{ color: palette.textSecondary }}>
              {t(language, 'ui.breakCopy.awaitingFinish')}
            </p>
          ) : null}

          <div className="flex w-full flex-col gap-3 sm:items-center">
            {currentBreak?.manualAwaiting ? (
              <Button
                type="button"
                className="h-12 w-full rounded-full border-0 px-8 text-base shadow-[0_18px_48px_-28px_rgba(15,23,42,0.45)] sm:w-auto sm:min-w-[220px]"
                disabled={busyAction !== null}
                style={{
                  background: scene.accent,
                  color: palette.primaryText,
                }}
                onClick={() =>
                  void runCommand('finish break', 'finish_current_break', undefined, (current) => ({
                    ...current,
                    currentBreak: null,
                    lastAction: t(language, 'ui.previewAction.done'),
                  }))
                }
              >
                {t(language, 'ui.breakCopy.actions.resumeWork')}
              </Button>
            ) : null}
            {currentBreak?.canPostpone ? (
              <Button
                type="button"
                variant="outline"
                className="h-12 w-full rounded-full px-8 text-base shadow-none sm:w-auto sm:min-w-[220px]"
                disabled={busyAction !== null}
                style={{
                  color: palette.actionText,
                  background: palette.actionGlass,
                  borderColor: palette.actionBorder,
                  backdropFilter: 'blur(18px)',
                }}
                onMouseEnter={(event) => {
                  event.currentTarget.style.background = palette.actionGlassHover
                }}
                onMouseLeave={(event) => {
                  event.currentTarget.style.background = palette.actionGlass
                }}
                onClick={() =>
                  void runCommand('postpone break', 'postpone_current_break', undefined, (current) => ({
                    ...current,
                    currentBreak: null,
                    nextBreakInMs: 2 * 60_000,
                    lastAction: t(language, 'ui.previewAction.later'),
                  }))
                }
              >
                {t(language, 'ui.breakCopy.actions.later')}
              </Button>
            ) : null}
          </div>
        </div>
      </section>
    </main>
  )
}

/* ------------------------------------------------------------------ */
/*  App                                                               */
/* ------------------------------------------------------------------ */

function App() {
  const [snapshot, setSnapshot] = useState<DesktopSnapshot | null>(null)
  const [form, setForm] = useState<PauzaSettings>(defaultSettings())
  const [busyAction, setBusyAction] = useState<string | null>(null)
  const [dirty, setDirty] = useState(false)
  const [formRevision, setFormRevision] = useState(0)
  const [saveRetryToken, setSaveRetryToken] = useState(0)
  const [error, setError] = useState<string | null>(null)
  const [settingsRoute, setSettingsRoute] = useState<SettingsRoute>('overview')
  const customBackdropInputRef = useRef<HTMLInputElement | null>(null)
  const preBreakSoundTimerRef = useRef<number | null>(null)
  const [runningApps, setRunningApps] = useState<string[] | null>(null)
  const [appSearchQuery, setAppSearchQuery] = useState('')
  const [appSearchLoading, setAppSearchLoading] = useState(false)
  const latestFormRevisionRef = useRef(0)
  const latestFormRef = useRef(form)
  const saveInFlightRef = useRef(false)
  const pendingSaveRetryRef = useRef(false)

  const breakMode = isBreakWindow()
  // Keep copy and document direction on the persisted locale until the save roundtrip completes.
  const language = resolveUiLanguage(form.language, snapshot?.settings.language)
  const languageConfig = getLanguageConfig(language)
  const showTimeToBreakInTrayLabel = (() => {
    const nextKey = 'ui.showTimeToBreakInTray'
    const translated = t(language, nextKey)
    if (translated !== nextKey) {
      return translated
    }

    const legacyKey = 'contributorPreferences.showTimeToBreakInTray'
    const legacyTranslated = t(language, legacyKey)
    return legacyTranslated !== legacyKey ? legacyTranslated : translated
  })()
  const loadStateErrorMessage = useEffectEvent(() => t(language, 'ui.error.loadState'))

  useEffect(() => {
    document.documentElement.lang = languageConfig.code
    document.documentElement.dir = languageConfig.direction
    document.body.setAttribute('lang', languageConfig.code)
    document.body.setAttribute('dir', languageConfig.direction)
  }, [languageConfig.code, languageConfig.direction])

  useEffect(() => {
    let active = true

    const load = async () => {
      try {
        const next = hasTauriRuntime()
          ? await invoke<DesktopSnapshot>('get_snapshot')
          : previewSnapshot()

        if (!active) return
        setSnapshot(next)
        setError(null)
      } catch (loadError) {
        if (!active) return
        setError(
          loadError instanceof Error ? loadError.message : loadStateErrorMessage(),
        )
      }
    }

    void load()
    const timer = window.setInterval(() => {
      void load()
    }, 1000)

    return () => {
      active = false
      window.clearInterval(timer)
    }
  }, [])

  useEffect(() => {
    if (snapshot && !dirty) {
      setForm(snapshot.settings)
    }
  }, [snapshot, dirty])

  useEffect(() => {
    latestFormRef.current = form
  }, [form])

  // Pre-break sound: schedule a sound X seconds before the next break
  useEffect(() => {
    if (preBreakSoundTimerRef.current !== null) {
      window.clearTimeout(preBreakSoundTimerRef.current)
      preBreakSoundTimerRef.current = null
    }

    if (!snapshot?.nextBreakDueMs || !snapshot?.nextBreakKind || !hasTauriRuntime()) return

    const s = snapshot.settings
    const kind = snapshot.nextBreakKind
    const enabled = kind === 'microbreak' ? s.microbreakNotificationEnabled : s.longBreakNotificationEnabled
    if (!enabled || s.breakSoundVolume <= 0) return

    const thresholdMs = kind === 'microbreak'
      ? s.microbreakNotificationSeconds * 1_000
      : s.longBreakNotificationSeconds * 1_000
    const delay = snapshot.nextBreakDueMs - thresholdMs - Date.now()
    if (delay < 0) return // already past the window; skip retroactive plays

    const sound = kind === 'microbreak' ? s.microbreakStartSound : s.longBreakStartSound
    preBreakSoundTimerRef.current = window.setTimeout(() => {
      playBreakSound(sound, s.breakSoundVolume)
      preBreakSoundTimerRef.current = null
    }, delay)

    return () => {
      if (preBreakSoundTimerRef.current !== null) {
        window.clearTimeout(preBreakSoundTimerRef.current)
        preBreakSoundTimerRef.current = null
      }
    }
  }, [
    snapshot?.nextBreakDueMs,
    snapshot?.nextBreakKind,
    snapshot?.settings.microbreakNotificationEnabled,
    snapshot?.settings.microbreakNotificationSeconds,
    snapshot?.settings.longBreakNotificationEnabled,
    snapshot?.settings.longBreakNotificationSeconds,
    snapshot?.settings.breakSoundVolume,
    snapshot?.settings.microbreakStartSound,
    snapshot?.settings.longBreakStartSound,
  ])

  const flushSettingsSave = useEffectEvent(async (settingsToSave: PauzaSettings, saveRevision: number) => {
    if (saveInFlightRef.current) {
      pendingSaveRetryRef.current = true
      return
    }

    let shouldRetry = false
    saveInFlightRef.current = true
    pendingSaveRetryRef.current = false

    try {
      setBusyAction('save settings')
      setError(null)

      if (!hasTauriRuntime()) {
        setSnapshot((current) => ({
          ...(current ?? previewSnapshot()),
          settings: settingsToSave,
          status: t(settingsToSave.language, 'ui.preview.status'),
          statusDetail: t(settingsToSave.language, 'ui.preview.detail'),
          lastAction: t(settingsToSave.language, 'ui.previewAction.save'),
        }))
      } else {
        const next = await invoke<DesktopSnapshot>('update_settings', { settings: settingsToSave })
        setSnapshot(next)
      }

      if (latestFormRevisionRef.current === saveRevision) {
        setDirty(false)
      } else {
        shouldRetry = true
      }
    } catch (saveError) {
      setError(saveError instanceof Error ? saveError.message : 'Failed to save settings')
    } finally {
      saveInFlightRef.current = false
      setBusyAction(null)
      if (shouldRetry || pendingSaveRetryRef.current) {
        pendingSaveRetryRef.current = false
        setSaveRetryToken((current) => current + 1)
      }
    }
  })

  useEffect(() => {
    if (breakMode || !dirty) {
      return
    }

    let cancelled = false
    const timer = window.setTimeout(() => {
      if (cancelled) {
        return
      }
      void flushSettingsSave(latestFormRef.current, latestFormRevisionRef.current)
    }, 220)

    return () => {
      cancelled = true
      window.clearTimeout(timer)
    }
  }, [breakMode, dirty, formRevision, saveRetryToken])

  const markFormDirty = () => {
    latestFormRevisionRef.current += 1
    setFormRevision(latestFormRevisionRef.current)
    setDirty(true)
  }

  const updateForm = <K extends keyof PauzaSettings>(key: K, value: PauzaSettings[K]) => {
    markFormDirty()
    setForm((current) => ({ ...current, [key]: value }))
  }

  const updateFormPatch = (patch: Partial<PauzaSettings>) => {
    markFormDirty()
    setForm((current) => ({ ...current, ...patch }))
  }

  const handleCustomBackdropFile = async (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    event.target.value = ''

    if (!file) {
      return
    }

    try {
      setBusyAction('prepare custom backdrop')
      setError(null)
      const dataUrl = await prepareCustomBackdrop(file)
      updateFormPatch({
        breakBackdrop: 'custom',
        breakCustomBackdropLabel: file.name,
        breakCustomBackdropDataUrl: dataUrl,
      })
    } catch (fileError) {
      setError(fileError instanceof Error ? fileError.message : 'Failed to prepare wallpaper')
    } finally {
      setBusyAction(null)
    }
  }

  const clearCustomBackdrop = () => {
    updateFormPatch({
      breakBackdrop: form.breakBackdrop === 'custom' ? 'paper' : form.breakBackdrop,
      breakCustomBackdropLabel: null,
      breakCustomBackdropDataUrl: null,
    })
  }

  const runCommand: CommandFn = async (action, command, args, previewTransform) => {
    try {
      setBusyAction(action)
      setError(null)

      if (!hasTauriRuntime()) {
        setSnapshot((current) => {
          const base = current ?? previewSnapshot()
          return previewTransform ? previewTransform(base) : base
        })
        return
      }

      const next = await invoke<DesktopSnapshot>(command, args)
      setSnapshot(next)
    } catch (commandError) {
      setError(commandError instanceof Error ? commandError.message : `Failed to ${action}`)
    } finally {
      setBusyAction(null)
    }
  }

  if (!snapshot) {
    return (
      <main className="grid min-h-screen place-items-center px-4">
        {t(language, 'ui.loading')}
      </main>
    )
  }

  if (breakMode) {
    return (
      <BreakWindow
        busyAction={busyAction}
        currentBreak={snapshot.currentBreak}
        language={language}
        runCommand={runCommand}
        settings={snapshot.settings}
      />
    )
  }

  const now = Date.now()
  const pauseActive =
    snapshot.pausedIndefinitely ||
    (snapshot.pauseUntilMs !== null && snapshot.pauseUntilMs > now)
  const focusActive = !pauseActive && snapshot.focusUntilMs !== null && snapshot.focusUntilMs > now
  const restorePreviewRuntime = (
    current: DesktopSnapshot,
    lastAction: string,
  ): DesktopSnapshot => ({
    ...current,
    ...previewNextBreak(current.settings),
    currentBreak: null,
    pauseUntilMs: null,
    pausedIndefinitely: false,
    focusUntilMs: null,
    status: t(language, 'ui.preview.status'),
    statusDetail: t(language, 'ui.preview.detail'),
    lastAction,
  })

  const rhythmProfile = matchRhythmProfile(form)
  const longBreakIntervalMinutes = form.microbreakIntervalMinutes * form.longBreakEvery
  const rhythmSummary = form.microbreakEnabled && form.longBreakEnabled
    ? t(language, 'ui.rhythmSummary', {
        microInterval: form.microbreakIntervalMinutes,
        microDuration: form.microbreakDurationSeconds,
        longInterval: longBreakIntervalMinutes,
        longDuration: form.longBreakDurationMinutes,
      })
    : form.microbreakEnabled
      ? t(language, 'ui.rhythmMicroOnlySummary', {
          microInterval: form.microbreakIntervalMinutes,
          microDuration: form.microbreakDurationSeconds,
        })
      : form.longBreakEnabled
        ? t(language, 'ui.rhythmLongOnlySummary', {
            longInterval: longBreakIntervalMinutes,
            longDuration: form.longBreakDurationMinutes,
          })
        : t(language, 'ui.rhythmOffSummary')
  const reminderSummary = form.reminderMode === 'smart'
    ? t(language, 'ui.reminderSummarySmart')
    : t(language, 'ui.reminderSummaryForced')
  const backdropLabel = t(
    language,
    BREAK_BACKDROP_OPTIONS.find(({ value }) => value === form.breakBackdrop)?.labelKey ??
      'ui.breakBackdropPaper',
  )
  const sharedEndSound = form.microbreakEndSound === form.longBreakEndSound
    ? BREAK_SOUND_OPTIONS.find(({ value }) => value === form.microbreakEndSound)
    : null
  const soundLabel = form.breakSoundVolume === 0
    ? t(language, 'ui.breakSoundSilence')
    : sharedEndSound
      ? t(language, sharedEndSound.labelKey)
      : t(language, 'ui.soundCustom')
  const soundSummary = t(language, 'ui.soundSummary', {
    sound: soundLabel,
    volume: form.breakSoundVolume,
  })
  const automationSummary = form.naturalBreaks && form.monitorDnd
    ? t(language, 'ui.automationBoth')
    : form.naturalBreaks
      ? t(language, 'ui.automationNatural')
      : form.monitorDnd
        ? t(language, 'ui.automationDnd')
        : t(language, 'ui.automationOff')
  const routeTitles: Record<SettingsRoute, string> = {
    overview: t(language, 'ui.settings'),
    rhythm: t(language, 'ui.detailRhythm'),
    reminders: t(language, 'ui.detailReminders'),
    appearance: t(language, 'ui.detailAppearance'),
    automation: t(language, 'ui.detailAutomation'),
    system: t(language, 'ui.detailSystem'),
  }

  const goBackInSettings = () => {
    setSettingsRoute('overview')
  }

  const renderSettingsRoute = () => {
    switch (settingsRoute) {
      case 'overview':
        return (
          <>
            <div>
              <SectionLabel>{t(language, 'ui.rhythmProfile')}</SectionLabel>
              <SettingsCard>
                <div className="px-4 py-3.5">
                  <RhythmProfilePicker
                    value={rhythmProfile}
                    ariaLabel={t(language, 'ui.rhythmProfile')}
                    options={[
                      { value: 'gentle', label: t(language, 'ui.rhythmProfileGentle') },
                      {
                        value: 'balanced',
                        label: `${t(language, 'ui.rhythmProfileBalanced')} · ${t(language, 'ui.recommended')}`,
                      },
                      { value: 'active', label: t(language, 'ui.rhythmProfileActive') },
                    ]}
                    customLabel={t(language, 'ui.rhythmProfileCustom')}
                    onChange={(next) => updateFormPatch(rhythmProfilePatch(next))}
                  />
                  <p className="settings-summary mt-3 text-muted-foreground">{rhythmSummary}</p>
                  {rhythmProfile === 'custom' ? (
                    <p className="settings-detail mt-1.5 text-muted-foreground">
                      {t(language, 'ui.customRhythmNotice')}
                    </p>
                  ) : null}
                </div>
                <SettingsLinkRow
                  label={t(language, 'ui.detailRhythm')}
                  detail={t(language, 'ui.detailRhythmHint')}
                  onClick={() => setSettingsRoute('rhythm')}
                />
                <SettingsLinkRow
                  label={t(language, 'ui.detailReminders')}
                  detail={reminderSummary}
                  onClick={() => setSettingsRoute('reminders')}
                />
              </SettingsCard>
            </div>

            <div>
              <SectionLabel>{t(language, 'ui.commonSettings')}</SectionLabel>
              <SettingsCard>
                <SettingsRow label={t(language, 'ui.breakDisplayMode')}>
                  <SegmentedControl
                    ariaLabel={t(language, 'ui.breakDisplayMode')}
                    value={form.fullscreen ? 'fullscreen' : 'window'}
                    options={[
                      { value: 'window', label: t(language, 'ui.window') },
                      { value: 'fullscreen', label: t(language, 'ui.fullscreen') },
                    ]}
                    onChange={(next) => updateForm('fullscreen', next === 'fullscreen')}
                  />
                </SettingsRow>
                <SettingsLinkRow
                  label={t(language, 'ui.detailAppearance')}
                  detail={`${form.fullscreen ? t(language, 'ui.fullscreen') : t(language, 'ui.window')} · ${backdropLabel} · ${soundSummary}`}
                  onClick={() => setSettingsRoute('appearance')}
                />
                <SettingsRow label={t(language, 'ui.launchOnLogin')}>
                  <Switch
                    checked={snapshot.autostartEnabled}
                    onCheckedChange={() =>
                      void runCommand(
                        'toggle autostart',
                        'toggle_autostart',
                        undefined,
                        (current) => ({
                          ...current,
                          autostartEnabled: !current.autostartEnabled,
                          lastAction: current.autostartEnabled
                            ? t(language, 'ui.previewAction.autostartOff')
                            : t(language, 'ui.previewAction.autostartOn'),
                        }),
                      )
                    }
                    aria-label={t(language, 'ui.launchOnLogin')}
                  />
                </SettingsRow>
                <SettingsRow label={t(language, 'ui.language')}>
                  <Select
                    value={form.language}
                    onValueChange={(next) => updateForm('language', next as AppLanguage)}
                  >
                    <SelectTrigger className="h-7 min-w-[210px] text-[12px]">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent className="max-h-[320px]">
                      {DESKTOP_LANGUAGE_CONFIGS.map((config) => (
                        <SelectItem key={config.code} value={config.code}>
                          {config.nativeLabel === config.label
                            ? config.nativeLabel
                            : `${config.nativeLabel} · ${config.label}`}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </SettingsRow>
              </SettingsCard>
            </div>

            <div>
              <SectionLabel>{t(language, 'ui.moreSettings')}</SectionLabel>
              <SettingsCard>
                <SettingsLinkRow
                  label={t(language, 'ui.detailAutomation')}
                  detail={automationSummary}
                  onClick={() => setSettingsRoute('automation')}
                />
                <SettingsLinkRow
                  label={t(language, 'ui.detailSystem')}
                  detail={`${snapshot.autostartEnabled ? t(language, 'ui.enabled') : t(language, 'ui.disabled')} · ${languageConfig.nativeLabel}`}
                  onClick={() => setSettingsRoute('system')}
                />
              </SettingsCard>
            </div>
          </>
        )

      case 'appearance':
      case 'reminders':
      case 'automation':
      case 'system':
        return (
          <>
            {settingsRoute === 'appearance' ? (
              <>
                <input
              ref={customBackdropInputRef}
              type="file"
              accept="image/png,image/jpeg,image/webp,image/avif,image/gif"
              className="hidden"
              onChange={(event) => {
                void handleCustomBackdropFile(event)
              }}
            />
            <div>
              <SectionLabel>{t(language, 'ui.breakSounds')}</SectionLabel>
              <p className="settings-detail mb-2 px-1 text-muted-foreground">
                {t(language, 'ui.breakSoundsHint')}
              </p>
              <SettingsCard>
                <SettingsRow label={t(language, 'ui.microbreakStartSound')}>
                  <Select
                    value={form.microbreakStartSound}
                    onValueChange={(next) =>
                      updateForm('microbreakStartSound', next as BreakSound)
                    }
                  >
                    <SelectTrigger className="h-7 min-w-[150px] text-[12px]">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {BREAK_SOUND_OPTIONS.map((option) => (
                        <SelectItem key={`micro-${option.value}`} value={option.value}>
                          {t(language, option.labelKey)}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <button
                    type="button"
                    title={t(language, 'ui.previewSound')}
                    aria-label={t(language, 'ui.previewSound')}
                    className="flex h-7 w-7 items-center justify-center rounded-md border border-white/70 bg-white/60 text-[11px] hover:bg-white/90 active:scale-90 transition-transform"
                    onClick={() => playBreakSound(form.microbreakStartSound, Math.max(form.breakSoundVolume, 50))}
                  >▶</button>
                </SettingsRow>
                <SettingsRow label={t(language, 'ui.microbreakEndSound')}>
                  <Select
                    value={form.microbreakEndSound}
                    onValueChange={(next) =>
                      updateForm('microbreakEndSound', next as BreakSound)
                    }
                  >
                    <SelectTrigger className="h-7 min-w-[150px] text-[12px]">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {BREAK_SOUND_OPTIONS.map((option) => (
                        <SelectItem key={`micro-end-${option.value}`} value={option.value}>
                          {t(language, option.labelKey)}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <button
                    type="button"
                    title={t(language, 'ui.previewSound')}
                    aria-label={t(language, 'ui.previewSound')}
                    className="flex h-7 w-7 items-center justify-center rounded-md border border-white/70 bg-white/60 text-[11px] hover:bg-white/90 active:scale-90 transition-transform"
                    onClick={() => playBreakSound(form.microbreakEndSound, Math.max(form.breakSoundVolume, 50))}
                  >▶</button>
                </SettingsRow>
                <SettingsRow label={t(language, 'ui.longBreakStartSound')}>
                  <Select
                    value={form.longBreakStartSound}
                    onValueChange={(next) =>
                      updateForm('longBreakStartSound', next as BreakSound)
                    }
                  >
                    <SelectTrigger className="h-7 min-w-[150px] text-[12px]">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {BREAK_SOUND_OPTIONS.map((option) => (
                        <SelectItem key={`long-${option.value}`} value={option.value}>
                          {t(language, option.labelKey)}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <button
                    type="button"
                    title={t(language, 'ui.previewSound')}
                    aria-label={t(language, 'ui.previewSound')}
                    className="flex h-7 w-7 items-center justify-center rounded-md border border-white/70 bg-white/60 text-[11px] hover:bg-white/90 active:scale-90 transition-transform"
                    onClick={() => playBreakSound(form.longBreakStartSound, Math.max(form.breakSoundVolume, 50))}
                  >▶</button>
                </SettingsRow>
                <SettingsRow label={t(language, 'ui.longBreakEndSound')}>
                  <Select
                    value={form.longBreakEndSound}
                    onValueChange={(next) =>
                      updateForm('longBreakEndSound', next as BreakSound)
                    }
                  >
                    <SelectTrigger className="h-7 min-w-[150px] text-[12px]">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {BREAK_SOUND_OPTIONS.map((option) => (
                        <SelectItem key={`long-end-${option.value}`} value={option.value}>
                          {t(language, option.labelKey)}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <button
                    type="button"
                    title={t(language, 'ui.previewSound')}
                    aria-label={t(language, 'ui.previewSound')}
                    className="flex h-7 w-7 items-center justify-center rounded-md border border-white/70 bg-white/60 text-[11px] hover:bg-white/90 active:scale-90 transition-transform"
                    onClick={() => playBreakSound(form.longBreakEndSound, Math.max(form.breakSoundVolume, 50))}
                  >▶</button>
                </SettingsRow>
                <div className="px-4 pb-3 pt-1">
                  <div className="rounded-[16px] border border-white/70 bg-white/70 px-3 py-3">
                    <div className="mb-2 flex items-center justify-between gap-3 text-[12px] text-muted-foreground">
                      <span>{t(language, 'ui.breakSoundVolume')}</span>
                      <span>{form.breakSoundVolume}%</span>
                    </div>
                    <input
                      type="range"
                      min={0}
                      max={100}
                      step={5}
                      value={form.breakSoundVolume}
                      onChange={(event) =>
                        updateForm('breakSoundVolume', Number(event.target.value))
                      }
                      aria-label={t(language, 'ui.breakSoundVolume')}
                      className="slider h-2 w-full rounded-full"
                      style={{
                        background: `linear-gradient(90deg, rgba(56,83,137,0.92) ${form.breakSoundVolume}%, rgba(56,83,137,0.14) ${form.breakSoundVolume}%)`,
                      }}
                    />
                  </div>
                </div>
              </SettingsCard>
            </div>

            <div>
              <SectionLabel>{t(language, 'ui.breakSurface')}</SectionLabel>
              <SettingsCard>
                <SettingsRow label={t(language, 'ui.breakDisplayMode')}>
                  <SegmentedControl
                    ariaLabel={t(language, 'ui.breakDisplayMode')}
                    value={form.fullscreen ? 'fullscreen' : 'window'}
                    options={[
                      { value: 'window', label: t(language, 'ui.window') },
                      { value: 'fullscreen', label: t(language, 'ui.fullscreen') },
                    ]}
                    onChange={(next) => updateForm('fullscreen', next === 'fullscreen')}
                  />
                </SettingsRow>
                <SettingsRow
                  label={t(language, 'ui.breakBackdrop')}
                  detail={t(language, 'ui.breakBackdropHint')}
                >
                  <Select
                    value={form.breakBackdrop}
                    onValueChange={(next) => updateForm('breakBackdrop', next as BreakBackdrop)}
                  >
                    <SelectTrigger className="h-7 min-w-[150px] text-[12px]">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {BREAK_BACKDROP_OPTIONS.map((option) => (
                        <SelectItem key={option.value} value={option.value}>
                          {t(language, option.labelKey)}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </SettingsRow>
                <SettingsRow
                  label={t(language, 'ui.customWallpaper')}
                  detail={t(language, 'ui.customWallpaperHint')}
                >
                  <Button
                    type="button"
                    variant="secondary"
                    className="h-7 px-3 text-[12px]"
                    disabled={busyAction !== null}
                    onClick={() => customBackdropInputRef.current?.click()}
                  >
                    {form.breakCustomBackdropDataUrl
                      ? t(language, 'ui.customWallpaperReplace')
                      : t(language, 'ui.customWallpaperChoose')}
                  </Button>
                </SettingsRow>
                {form.breakBackdrop === 'custom' || form.breakCustomBackdropDataUrl ? (
                  <div className="px-4 pb-3 pt-1">
                    <div className="rounded-[16px] border border-white/70 bg-white/70 px-3 py-3">
                      <div className="flex items-start justify-between gap-3">
                        <div className="min-w-0">
                          <p className="truncate text-[13px] font-semibold leading-5 text-foreground">
                            {form.breakCustomBackdropLabel ??
                              t(language, 'ui.customWallpaperEmpty')}
                          </p>
                          <p className="settings-detail mt-1 text-muted-foreground">
                            {form.breakCustomBackdropDataUrl
                              ? t(language, 'ui.customWallpaperReady')
                              : t(language, 'ui.customWallpaperEmptyHint')}
                          </p>
                        </div>
                        {form.breakCustomBackdropDataUrl ? (
                          <Button
                            type="button"
                            variant="ghost"
                            className="h-7 shrink-0 px-2 text-[12px]"
                            disabled={busyAction !== null}
                            onClick={clearCustomBackdrop}
                          >
                            {t(language, 'ui.customWallpaperRemove')}
                          </Button>
                        ) : null}
                      </div>
                      <div className="mt-3 overflow-hidden rounded-[14px] border border-white/60 bg-slate-100">
                        <div className="flex aspect-[16/5] items-center justify-center bg-[linear-gradient(140deg,rgba(15,23,42,0.08),rgba(255,255,255,0.82))]">
                          {form.breakCustomBackdropDataUrl ? (
                            <img
                              src={form.breakCustomBackdropDataUrl}
                              alt={form.breakCustomBackdropLabel ?? t(language, 'ui.customWallpaper')}
                              className="h-full w-full object-contain"
                            />
                          ) : (
                            <p className="settings-detail px-4 text-center text-muted-foreground">
                              {t(language, 'ui.customWallpaperEmptyHint')}
                            </p>
                          )}
                        </div>
                      </div>
                    </div>
                  </div>
                ) : null}
                <SettingsRow
                  label={t(language, 'ui.breakIdeas')}
                  detail={t(language, 'ui.breakIdeasHint')}
                >
                  <Switch
                    checked={form.breakIdeasEnabled}
                    onCheckedChange={(next) => updateForm('breakIdeasEnabled', next)}
                    aria-label={t(language, 'ui.breakIdeas')}
                  />
                </SettingsRow>
                <SettingsRow
                  label={t(language, 'ui.currentTimeInBreaks')}
                  detail={t(language, 'ui.currentTimeInBreaksHint')}
                >
                  <Switch
                    checked={form.currentTimeInBreaks}
                    onCheckedChange={(next) => updateForm('currentTimeInBreaks', next)}
                    aria-label={t(language, 'ui.currentTimeInBreaks')}
                  />
                </SettingsRow>
              </SettingsCard>
                </div>

              </>
            ) : null}

            {settingsRoute === 'reminders' ? (
              <>
                <div>
              <SectionLabel>{t(language, 'ui.reminderMode')}</SectionLabel>
              <SettingsCard>
                <SettingsRow
                  label={t(language, 'ui.reminderMode')}
                  detail={t(language, 'ui.reminderModeHint')}
                >
                  <SegmentedControl
                    ariaLabel={t(language, 'ui.reminderMode')}
                    value={form.reminderMode}
                    options={[
                      { value: 'smart', label: t(language, 'ui.reminderModeSmart') },
                      { value: 'forced', label: t(language, 'ui.reminderModeForced') },
                    ]}
                    onChange={(next) => updateForm('reminderMode', next as ReminderMode)}
                  />
                </SettingsRow>
              </SettingsCard>
                </div>

                <div>
                  <SectionLabel>{t(language, 'ui.postpone')}</SectionLabel>
                  <SettingsCard>
                    <SettingsRow label={t(language, 'ui.microbreaks')}>
                      <CompactNumber
                        value={form.microbreakPostponeMinutes}
                        min={1}
                        max={30}
                        suffix={t(language, 'ui.suffix.minutes')}
                        onChange={(value) => updateForm('microbreakPostponeMinutes', value)}
                      />
                      <Switch
                        checked={form.microbreakAllowPostpone}
                        onCheckedChange={(next) => updateForm('microbreakAllowPostpone', next)}
                        aria-label={t(language, 'ui.microbreaks')}
                      />
                    </SettingsRow>
                    <SettingsRow label={t(language, 'ui.longBreaks')}>
                      <CompactNumber
                        value={form.longBreakPostponeMinutes}
                        min={1}
                        max={60}
                        suffix={t(language, 'ui.suffix.minutes')}
                        onChange={(value) => updateForm('longBreakPostponeMinutes', value)}
                      />
                      <Switch
                        checked={form.longBreakAllowPostpone}
                        onCheckedChange={(next) => updateForm('longBreakAllowPostpone', next)}
                        aria-label={t(language, 'ui.longBreaks')}
                      />
                    </SettingsRow>
                  </SettingsCard>
                </div>

                <div>
                  <SectionLabel>{t(language, 'ui.preBreakNotifications')}</SectionLabel>
                  <SettingsCard>
                    <SettingsRow label={t(language, 'ui.microbreaks')}>
                      <CompactNumber
                        value={form.microbreakNotificationSeconds}
                        min={5}
                        max={300}
                        suffix={t(language, 'ui.suffix.secondsBefore')}
                        onChange={(value) => updateForm('microbreakNotificationSeconds', value)}
                      />
                      <Switch
                        checked={form.microbreakNotificationEnabled}
                        onCheckedChange={(next) => updateForm('microbreakNotificationEnabled', next)}
                        aria-label={t(language, 'ui.microbreaks')}
                      />
                    </SettingsRow>
                    <SettingsRow label={t(language, 'ui.longBreaks')}>
                      <CompactNumber
                        value={form.longBreakNotificationSeconds}
                        min={5}
                        max={600}
                        suffix={t(language, 'ui.suffix.secondsBefore')}
                        onChange={(value) => updateForm('longBreakNotificationSeconds', value)}
                      />
                      <Switch
                        checked={form.longBreakNotificationEnabled}
                        onCheckedChange={(next) => updateForm('longBreakNotificationEnabled', next)}
                        aria-label={t(language, 'ui.longBreaks')}
                      />
                    </SettingsRow>
                  </SettingsCard>
                </div>
              </>
            ) : null}

            {settingsRoute === 'automation' ? (
              <>
                <div>
              <SectionLabel>{t(language, 'ui.smartPause')}</SectionLabel>
              <SettingsCard>
                <SettingsRow
                  label={t(language, 'ui.naturalBreaks')}
                  detail={t(language, 'ui.naturalBreaksHint')}
                >
                  <CompactNumber
                    value={form.naturalBreakResetMinutes}
                    min={1}
                    max={60}
                    suffix={t(language, 'ui.suffix.minutes')}
                    onChange={(value) => updateForm('naturalBreakResetMinutes', value)}
                  />
                  <Switch
                    checked={form.naturalBreaks}
                    onCheckedChange={(next) => updateForm('naturalBreaks', next)}
                    aria-label={t(language, 'ui.naturalBreaks')}
                  />
                </SettingsRow>
                <SettingsRow
                  label={t(language, 'ui.dnd')}
                  detail={t(language, 'ui.dndHint')}
                >
                  <Switch
                    checked={form.monitorDnd}
                    onCheckedChange={(next) => updateForm('monitorDnd', next)}
                    aria-label={t(language, 'ui.dnd')}
                  />
                </SettingsRow>
              </SettingsCard>
                </div>

                <div>
              <SectionLabel>{t(language, 'ui.appExclusions')}</SectionLabel>
              <SettingsCard>
                <SettingsRow
                  label={t(language, 'ui.appExclusions')}
                  detail={t(language, 'ui.appExclusionsHint')}
                >
                  <Select
                    value={form.appExclusionRule}
                    onValueChange={(next) =>
                      updateForm('appExclusionRule', next as AppExclusionRule)
                    }
                  >
                    <SelectTrigger className="h-7 min-w-[130px] text-[12px]">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="pause">
                        {t(language, 'ui.appRulePause')}
                      </SelectItem>
                      <SelectItem value="resume">
                        {t(language, 'ui.appRuleResume')}
                      </SelectItem>
                    </SelectContent>
                  </Select>
                  <Switch
                    checked={form.appExclusionsEnabled}
                    onCheckedChange={(next) => updateForm('appExclusionsEnabled', next)}
                    aria-label={t(language, 'ui.appExclusions')}
                  />
                </SettingsRow>
                <div className="px-4 pb-3 pt-1 space-y-2">
                  {/* Running app search */}
                  <div className="relative flex items-center">
                    <svg className="pointer-events-none absolute left-2.5 h-3.5 w-3.5 shrink-0 text-muted-foreground/70" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.2" strokeLinecap="round" strokeLinejoin="round">
                      <circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/>
                    </svg>
                    <input
                      type="text"
                      placeholder={t(language, 'ui.appExclusionsSearchPlaceholder')}
                      value={appSearchQuery}
                      onChange={(e) => setAppSearchQuery(e.target.value)}
                      onFocus={() => {
                        if (!runningApps && !appSearchLoading && hasTauriRuntime()) {
                          setAppSearchLoading(true)
                          void invoke<string[]>('list_running_app_names').then((names) => {
                            setRunningApps(names)
                            setAppSearchLoading(false)
                          }).catch(() => setAppSearchLoading(false))
                        }
                      }}
                      className="h-8 w-full rounded-md border border-white/80 bg-white/70 pl-7 pr-8 text-[12px] shadow-[inset_0_1px_2px_rgba(0,0,0,0.04)] placeholder:text-muted-foreground/50 focus:bg-white focus:outline-none focus:ring-1 focus:ring-black/10"
                    />
                    {appSearchLoading ? (
                      <span className="absolute right-2.5 text-[10px] text-muted-foreground">{t(language, 'ui.appExclusionsSearchLoading')}</span>
                    ) : appSearchQuery ? (
                      <button
                        type="button"
                        onClick={() => setAppSearchQuery('')}
                        className="absolute right-2 flex h-4 w-4 items-center justify-center rounded-full bg-muted-foreground/20 text-[10px] leading-none text-muted-foreground hover:bg-muted-foreground/30"
                        aria-label="清除"
                      >✕</button>
                    ) : null}
                  </div>
                  {/* Search results */}
                  {appSearchQuery.trim().length > 0 && (
                    <div className="max-h-28 overflow-y-auto rounded-[10px] border border-white/70 bg-white/85 text-[12px] shadow-sm">
                      {(runningApps ?? [])
                        .filter(name => name.toLowerCase().includes(appSearchQuery.toLowerCase()))
                        .slice(0, 20)
                        .map((name) => (
                          <button
                            key={name}
                            type="button"
                            title={t(language, 'ui.appExclusionsAdd')}
                            className="flex w-full items-center gap-2 px-3 py-1.5 text-left hover:bg-black/5 active:bg-black/10"
                            onClick={() => {
                              const current = form.appExclusionCommands.trim()
                              const lines = current ? current.split('\n') : []
                              if (!lines.includes(name)) {
                                updateForm('appExclusionCommands', [...lines, name].join('\n'))
                              }
                              setAppSearchQuery('')
                            }}
                          >
                            <span className="h-1.5 w-1.5 shrink-0 rounded-full bg-emerald-400/70" />
                            {name}
                          </button>
                        ))}
                      {(runningApps ?? []).filter(n => n.toLowerCase().includes(appSearchQuery.toLowerCase())).length === 0 && (
                        <p className="px-3 py-2 text-muted-foreground">{t(language, 'ui.appExclusionsSearchEmpty')}</p>
                      )}
                    </div>
                  )}
                  {/* Manual textarea */}
                  <p className="settings-detail text-muted-foreground">{t(language, 'ui.appExclusionsManualHint')}</p>
                  <Textarea
                    rows={3}
                    value={form.appExclusionCommands}
                    placeholder={'zoom.us\nmeet.google.com\nobs'}
                    onChange={(event) => updateForm('appExclusionCommands', event.target.value)}
                    className="w-full text-[12px]"
                  />
                </div>
              </SettingsCard>
                </div>
              </>
            ) : null}

            {settingsRoute === 'system' ? (
              <div>
              <SectionLabel>{t(language, 'ui.general')}</SectionLabel>
              <SettingsCard>
                <SettingsRow label={t(language, 'ui.launchOnLogin')}>
                  <Switch
                    checked={snapshot.autostartEnabled}
                    onCheckedChange={() =>
                      void runCommand(
                        'toggle autostart',
                        'toggle_autostart',
                        undefined,
                        (current) => ({
                          ...current,
                          autostartEnabled: !current.autostartEnabled,
                          lastAction: current.autostartEnabled
                            ? t(language, 'ui.previewAction.autostartOff')
                            : t(language, 'ui.previewAction.autostartOn'),
                        }),
                      )
                    }
                    aria-label={t(language, 'ui.launchOnLogin')}
                  />
                </SettingsRow>
                <SettingsRow label={showTimeToBreakInTrayLabel}>
                  <Switch
                    checked={form.showTimeToBreakInTray}
                    onCheckedChange={(next) => updateForm('showTimeToBreakInTray', next)}
                    aria-label={showTimeToBreakInTrayLabel}
                  />
                </SettingsRow>
                <SettingsRow label={t(language, 'ui.language')}>
                  <Select
                    value={form.language}
                    onValueChange={(next) => updateForm('language', next as AppLanguage)}
                  >
                    <SelectTrigger className="h-7 min-w-[210px] text-[12px]">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent className="max-h-[320px]">
                      {DESKTOP_LANGUAGE_CONFIGS.map((config) => (
                        <SelectItem key={config.code} value={config.code}>
                          {config.nativeLabel === config.label
                            ? config.nativeLabel
                            : `${config.nativeLabel} · ${config.label}`}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </SettingsRow>
              </SettingsCard>
              </div>
            ) : null}
          </>
        )

      case 'rhythm':
        return (
          <>
            <SchedulePresetCard
              label={t(language, 'ui.microbreaks')}
              switchLabel={t(language, 'ui.microbreaks')}
              checked={form.microbreakEnabled}
              onCheckedChange={(next) => updateForm('microbreakEnabled', next)}
            >
              <PresetChipRow
                label={t(language, 'ui.interval')}
                unit={t(language, 'ui.suffix.minutes')}
                ariaLabel={`${t(language, 'ui.microbreaks')} ${t(language, 'ui.interval')}`}
                customLabel={t(language, 'ui.customValue')}
                value={form.microbreakIntervalMinutes}
                options={MICROBREAK_INTERVAL_PRESETS}
                min={1}
                max={180}
                onChange={(value) => updateForm('microbreakIntervalMinutes', value)}
              />
              <PresetChipRow
                label={t(language, 'ui.durationLabel')}
                unit={t(language, 'ui.suffix.secondsLong')}
                ariaLabel={`${t(language, 'ui.microbreaks')} ${t(language, 'ui.durationLabel')}`}
                customLabel={t(language, 'ui.customValue')}
                value={form.microbreakDurationSeconds}
                options={MICROBREAK_DURATION_PRESETS}
                min={5}
                max={300}
                onChange={(value) => updateForm('microbreakDurationSeconds', value)}
              />
            </SchedulePresetCard>

            <SchedulePresetCard
              label={t(language, 'ui.longBreaks')}
              switchLabel={t(language, 'ui.longBreaks')}
              checked={form.longBreakEnabled}
              onCheckedChange={(next) => updateForm('longBreakEnabled', next)}
            >
              <PresetChipRow
                label={t(language, 'ui.every')}
                unit={t(language, 'ui.suffix.reminderCycles')}
                ariaLabel={`${t(language, 'ui.longBreaks')} ${t(language, 'ui.every')}`}
                customLabel={t(language, 'ui.customValue')}
                value={form.longBreakEvery}
                options={LONGBREAK_EVERY_PRESETS}
                min={1}
                max={12}
                onChange={(value) => updateForm('longBreakEvery', value)}
              />
              <p className="settings-caption pl-[68px] text-muted-foreground">
                {t(language, 'ui.fullBreakCadenceHint', {
                  minutes: longBreakIntervalMinutes,
                  cycles: form.longBreakEvery,
                })}
              </p>
              <PresetChipRow
                label={t(language, 'ui.durationLabel')}
                unit={t(language, 'ui.suffix.minutes')}
                ariaLabel={`${t(language, 'ui.longBreaks')} ${t(language, 'ui.durationLabel')}`}
                customLabel={t(language, 'ui.customValue')}
                value={form.longBreakDurationMinutes}
                options={LONGBREAK_DURATION_PRESETS}
                min={1}
                max={60}
                onChange={(value) => updateForm('longBreakDurationMinutes', value)}
              />
            </SchedulePresetCard>
          </>
        )

      default:
        return null
    }
  }

  return (
    <main className="flex h-screen flex-col select-none bg-background">
      <header className="grid h-14 shrink-0 grid-cols-[32px_1fr_32px] items-center px-5">
        <div>
          {settingsRoute !== 'overview' ? (
            <button
              type="button"
              title={t(language, 'ui.back')}
              aria-label={t(language, 'ui.back')}
              className="flex h-8 w-8 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-black/[0.05] hover:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
              onClick={goBackInSettings}
            >
              <ArrowLeft className="h-4 w-4" aria-hidden="true" />
            </button>
          ) : null}
        </div>
        <h1 className="settings-page-title text-center text-foreground">
          {routeTitles[settingsRoute]}
        </h1>
        <div className="flex h-8 items-center justify-end" aria-live="polite">
          {busyAction === 'save settings' ? (
            <span className="h-1.5 w-1.5 rounded-full bg-foreground/45" title={t(language, 'ui.saving')} />
          ) : null}
        </div>
      </header>

      {error ? (
        <div className="mx-5 mb-2 rounded-lg bg-rose-50 px-3 py-2 text-[12px] text-rose-600">
          {error}
        </div>
      ) : null}

      <div className="flex-1 overflow-y-auto px-5 pb-5">
        <div className="space-y-3">
          {pauseActive || focusActive ? (
            <div>
              <SectionLabel>{t(language, 'ui.currentStatus')}</SectionLabel>
              <SettingsCard>
                <SettingsRow label={snapshot.status} detail={snapshot.statusDetail}>
                  {pauseActive ? (
                    <Button
                      type="button"
                      className="h-7 px-3 text-[12px]"
                      disabled={busyAction !== null}
                      onClick={() =>
                        void runCommand(
                          'resume schedule',
                          'resume_breaks',
                          undefined,
                          (current) =>
                            restorePreviewRuntime(current, t(language, 'ui.previewAction.resume')),
                        )
                      }
                    >
                      {t(language, 'runtime.tray.resume')}
                    </Button>
                  ) : focusActive ? (
                    <Button
                      type="button"
                      className="h-7 px-3 text-[12px]"
                      disabled={busyAction !== null}
                      onClick={() =>
                        void runCommand(
                          'clear focus session',
                          'clear_focus_session',
                          undefined,
                          (current) =>
                            restorePreviewRuntime(
                              current,
                              t(language, 'ui.previewAction.focusCleared'),
                            ),
                        )
                      }
                    >
                      {t(language, 'ui.clearFocus')}
                    </Button>
                  ) : null}
                </SettingsRow>
              </SettingsCard>
            </div>
          ) : null}

          {renderSettingsRoute()}
        </div>
      </div>
    </main>
  )
}

export default App
