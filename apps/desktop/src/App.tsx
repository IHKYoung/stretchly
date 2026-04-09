import { invoke } from '@tauri-apps/api/core'
import { useEffect, useRef, useState, type ChangeEvent, type ReactNode } from 'react'

import { Button } from '@/components/ui/button'
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
  pickBreakPrompt,
  prepareCustomBackdrop,
  type BreakBackdrop,
  type BreakKind,
  type BreakSound,
} from '@/lib/break-prompt'
import { getBreakMessageCopy } from '@/locales/break-message-copy'
import { cn } from '@/lib/utils'
import {
  DESKTOP_LANGUAGE_CONFIGS,
  formatCountdown,
  formatDuration,
  normalizeLanguage,
  t,
  type AppLanguage,
} from './i18n'

type AppExclusionRule = 'pause' | 'resume'
type ReminderMode = 'smart' | 'forced'
type TargetScreen = 'primary' | 'cursor'
type SettingsCategory = 'schedule' | 'preferences'
type PreviewRuntimeMode = 'default' | 'paused' | 'focus'

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
  idleOpportunitySeconds: number
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

const MICROBREAK_INTERVAL_PRESETS = [5, 10, 15, 20, 30]
const MICROBREAK_DURATION_PRESETS = [15, 20, 30, 60]
const LONGBREAK_EVERY_PRESETS = [2, 3, 4, 5]
const LONGBREAK_DURATION_PRESETS = [3, 5, 10, 15]

function defaultSettings(): PauzaSettings {
  return {
    language: 'zh-CN',
    microbreakEnabled: true,
    microbreakIntervalMinutes: 10,
    microbreakDurationSeconds: 20,
    microbreakNotificationEnabled: true,
    microbreakNotificationSeconds: 10,
    microbreakAllowPostpone: true,
    microbreakPostponeMinutes: 2,
    microbreakPostponesLimit: 1,
    reminderMode: 'smart',
    idleOpportunitySeconds: 12,
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
  const previewBreakStartedAtMs = Date.now() - 6_000
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

function clampNumber(value: string, min: number, max: number) {
  const parsed = Number(value)
  if (!Number.isFinite(parsed)) {
    return min
  }

  return Math.min(max, Math.max(min, Math.round(parsed)))
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
  return (
    <div className="flex items-center gap-1.5">
      <div className="flex h-7 items-center overflow-hidden rounded-md bg-black/[0.04]">
        <button
          type="button"
          onClick={() => onChange(Math.max(min, value - step))}
          disabled={value <= min}
          className="flex h-full w-6 items-center justify-center border-r border-black/[0.06] text-[13px] text-muted-foreground transition-colors hover:bg-black/[0.04] hover:text-foreground disabled:opacity-30"
          aria-label="decrease"
        >
          −
        </button>
        <input
          type="number"
          min={min}
          max={max}
          value={value}
          onChange={(event) => onChange(clampNumber(event.target.value, min, max))}
          className="w-9 bg-transparent text-center text-[13px] font-medium text-foreground outline-none [appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none"
        />
        <button
          type="button"
          onClick={() => onChange(Math.min(max, value + step))}
          disabled={value >= max}
          className="flex h-full w-6 items-center justify-center border-l border-black/[0.06] text-[13px] text-muted-foreground transition-colors hover:bg-black/[0.04] hover:text-foreground disabled:opacity-30"
          aria-label="increase"
        >
          +
        </button>
      </div>
      <span className="whitespace-nowrap text-[11px] text-muted-foreground">{suffix}</span>
    </div>
  )
}

function PresetChipGroup({
  ariaLabel,
  value,
  options,
  min,
  max,
  onChange,
}: {
  ariaLabel: string
  value: number
  options: number[]
  min: number
  max: number
  onChange: (next: number) => void
}) {
  const isPreset = options.includes(value)

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
        type="number"
        min={min}
        max={max}
        value={value}
        onChange={(e) => onChange(clampNumber(e.target.value, min, max))}
        className={cn(
          'w-12 rounded-lg border py-1 text-center text-[13px] font-medium tabular-nums outline-none transition-all [appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none',
          isPreset
            ? 'border-transparent bg-transparent text-muted-foreground/40 hover:border-black/[0.06] hover:text-muted-foreground'
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
  value,
  options,
  min,
  max,
  onChange,
}: {
  label: string
  unit: string
  ariaLabel: string
  value: number
  options: number[]
  min: number
  max: number
  onChange: (next: number) => void
}) {
  return (
    <div className="flex items-center gap-3">
      <span className="w-10 shrink-0 text-right text-[12px] font-medium text-muted-foreground">
        {label}
      </span>
      <PresetChipGroup
        ariaLabel={ariaLabel}
        value={value}
        options={options}
        min={min}
        max={max}
        onChange={onChange}
      />
      <span className="shrink-0 text-[11px] text-muted-foreground">{unit}</span>
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
        <p className="text-[13px] font-medium text-foreground">{label}</p>
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
    <h3 className="mb-1.5 px-1 text-[12px] font-medium text-muted-foreground">{children}</h3>
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
    <div className="flex min-h-[42px] items-center gap-4 px-4 py-2">
      <div className="min-w-0 flex-1">
        <p className="text-[13px] text-foreground">{label}</p>
        {detail ? (
          <p className="mt-0.5 text-[11px] leading-4 text-muted-foreground">{detail}</p>
        ) : null}
      </div>
      <div className="flex w-[200px] shrink-0 items-center justify-end gap-2">{children}</div>
    </div>
  )
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
  const lastStartSoundRef = useRef<string | null>(null)
  const lastEndSoundRef = useRef<string | null>(null)
  const endSoundTimerRef = useRef<number | null>(null)
  const breakKey = currentBreak ? `${currentBreak.kind}:${currentBreak.startedAtMs}` : null
  const breakCopy = getBreakMessageCopy(language)
  const promptText = currentBreak?.manualAwaiting
    ? breakCopy.manualAwaiting
    : currentBreak
      ? settings.breakIdeasEnabled
        ? pickBreakPrompt(language, currentBreak.kind, currentBreak.startedAtMs) ||
          breakCopy.defaultPrompt[currentBreak.kind]
        : breakCopy.defaultPrompt[currentBreak.kind]
      : breakCopy.clearedDetail
  const countdownText = formatCountdown(currentBreak?.manualAwaiting ? 0 : remaining)
  const backgroundStyle = hasCustomBackdrop
    ? {
        backgroundImage: `linear-gradient(140deg, rgba(15,23,42,0.34), rgba(15,23,42,0.12)), url("${settings.breakCustomBackdropDataUrl}")`,
        backgroundPosition: 'center',
        backgroundRepeat: 'no-repeat',
        backgroundSize: 'cover',
      }
    : {
        backgroundImage: scene.background,
      }

  useEffect(() => {
    if (!currentBreak || !hasTauriRuntime() || !breakKey) {
      return
    }

    const sound =
      currentBreak.kind === 'microbreak'
        ? settings.microbreakStartSound
        : settings.longBreakStartSound
    const playKey = `${breakKey}:start`

    if (settings.breakSoundVolume <= 0 || lastStartSoundRef.current === playKey) {
      return
    }

    lastStartSoundRef.current = playKey
    playBreakSound(sound, settings.breakSoundVolume)
  }, [
    breakKey,
    currentBreak?.kind,
    settings.breakSoundVolume,
    settings.longBreakStartSound,
    settings.microbreakStartSound,
  ])

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

  return (
    <main className="relative min-h-screen overflow-hidden" style={backgroundStyle}>
      <div
        className="pointer-events-none absolute inset-0 opacity-30"
        style={{ backgroundImage: scene.texture }}
      />

      <section className="relative mx-auto flex min-h-screen w-full max-w-[760px] flex-col items-center justify-center px-6 py-8 text-center sm:px-10 sm:py-10">
        {currentBreak?.showClock ? (
          <div className="absolute right-6 top-6 text-[11px] font-medium tracking-[0.16em] text-slate-500/90 sm:right-8 sm:top-8">
            {clockLabel(language)}
          </div>
        ) : null}

        <div className="relative w-full overflow-hidden rounded-[36px] border border-white/42 bg-white/24 px-6 py-10 shadow-[0_32px_120px_-52px_rgba(15,23,42,0.42)] backdrop-blur-[28px] sm:px-10 sm:py-12">
          <div className="pointer-events-none absolute inset-0 bg-[linear-gradient(180deg,rgba(255,255,255,0.42),rgba(255,255,255,0.14))]" />

          <div className="relative">
            <div className="mx-auto max-w-[22ch] text-balance text-[2rem] font-semibold leading-[1.08] tracking-[-0.06em] text-slate-950 sm:text-[2.8rem]">
              {promptText}
            </div>

            <div className="mt-8 text-[3.75rem] font-semibold tracking-[-0.08em] text-slate-950 tabular-nums sm:text-[5.75rem]">
              {countdownText}
            </div>

            <div
              className="mx-auto mt-6 h-[3px] w-full max-w-[360px] overflow-hidden rounded-full"
              style={{ background: scene.meterTrack }}
            >
              <div
                className="h-full rounded-full transition-[width] duration-700 ease-out"
                style={{
                  width: `${Math.round(progress * 100)}%`,
                  background: scene.meterFill,
                }}
              />
            </div>

            {currentBreak?.manualAwaiting ? (
              <p className="mt-4 text-sm text-slate-600">{breakCopy.awaitingFinish}</p>
            ) : null}

            <div className="mt-10 flex flex-col gap-3 sm:flex-row">
              <Button
                type="button"
                className="flex-1"
                disabled={!currentBreak || busyAction !== null}
                onClick={() =>
                  void runCommand('finish break', 'finish_current_break', undefined, (current) => ({
                    ...current,
                    currentBreak: null,
                    lastAction: t(language, 'ui.previewAction.done'),
                  }))
                }
              >
                {currentBreak?.manualAwaiting
                  ? breakCopy.actions.resumeWork
                  : breakCopy.actions.done}
              </Button>
              {currentBreak?.canPostpone ? (
                <Button
                  type="button"
                  variant="secondary"
                  className="flex-1"
                  disabled={busyAction !== null}
                  onClick={() =>
                    void runCommand('postpone break', 'postpone_current_break', undefined, (current) => ({
                      ...current,
                      currentBreak: null,
                      nextBreakInMs: 2 * 60_000,
                      lastAction: t(language, 'ui.previewAction.later'),
                    }))
                  }
                >
                  {breakCopy.actions.later}
                </Button>
              ) : null}
              {currentBreak?.canSkip ? (
                <Button
                  type="button"
                  variant="outline"
                  className="flex-1"
                  disabled={busyAction !== null}
                  onClick={() =>
                    void runCommand('skip break', 'skip_current_break', undefined, (current) => ({
                      ...current,
                      currentBreak: null,
                      lastAction: t(language, 'ui.previewAction.skip'),
                    }))
                  }
                >
                  {breakCopy.actions.skip}
                </Button>
              ) : null}
            </div>
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
  const [error, setError] = useState<string | null>(null)
  const [activeCategory, setActiveCategory] = useState<SettingsCategory>('schedule')
  const customBackdropInputRef = useRef<HTMLInputElement | null>(null)

  const breakMode = isBreakWindow()
  const language = normalizeLanguage(form.language)

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
          loadError instanceof Error ? loadError.message : t(language, 'ui.error.loadState'),
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
  }, [language])

  useEffect(() => {
    if (snapshot && !dirty) {
      setForm(snapshot.settings)
    }
  }, [snapshot, dirty])

  useEffect(() => {
    if (breakMode || !dirty) {
      return
    }

    let cancelled = false
    const timer = window.setTimeout(async () => {
      try {
        setBusyAction('save settings')
        setError(null)

        if (!hasTauriRuntime()) {
          if (!cancelled) {
            setSnapshot((current) => ({
              ...(current ?? previewSnapshot()),
              settings: form,
              status: t(form.language, 'ui.preview.status'),
              statusDetail: t(form.language, 'ui.preview.detail'),
              lastAction: t(form.language, 'ui.previewAction.save'),
            }))
            setDirty(false)
          }
          return
        }

        const next = await invoke<DesktopSnapshot>('update_settings', { settings: form })
        if (!cancelled) {
          setSnapshot(next)
          setDirty(false)
        }
      } catch (saveError) {
        if (!cancelled) {
          setError(saveError instanceof Error ? saveError.message : 'Failed to save settings')
        }
      } finally {
        if (!cancelled) {
          setBusyAction(null)
        }
      }
    }, 220)

    return () => {
      cancelled = true
      window.clearTimeout(timer)
    }
  }, [breakMode, dirty, form])

  const updateForm = <K extends keyof PauzaSettings>(key: K, value: PauzaSettings[K]) => {
    setDirty(true)
    setForm((current) => ({ ...current, [key]: value }))
  }

  const updateFormPatch = (patch: Partial<PauzaSettings>) => {
    setDirty(true)
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
  const resetActionDisabled = busyAction !== null || snapshot.currentBreak?.strictMode === true
  const resetPreviewSchedule = (current: DesktopSnapshot): DesktopSnapshot => ({
    ...current,
    ...previewNextBreak(current.settings),
    currentBreak: null,
    lastAction: t(language, 'ui.previewAction.reset'),
  })
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

  const categoryItems: Array<{ id: SettingsCategory; title: string }> = [
    { id: 'schedule', title: t(language, 'ui.schedule') },
    { id: 'preferences', title: t(language, 'ui.preferences') },
  ]

  const renderActiveCategory = () => {
    switch (activeCategory) {
      case 'preferences':
        return (
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
                          <p className="truncate text-[12px] font-medium text-foreground">
                            {form.breakCustomBackdropLabel ??
                              t(language, 'ui.customWallpaperEmpty')}
                          </p>
                          <p className="mt-1 text-[11px] leading-4 text-muted-foreground">
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
                            <p className="px-4 text-center text-[11px] leading-4 text-muted-foreground">
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

            <div>
              <SectionLabel>{t(language, 'ui.breakSounds')}</SectionLabel>
              <SettingsCard>
                <SettingsRow
                  label={t(language, 'ui.microbreakStartSound')}
                  detail={t(language, 'ui.breakSoundsHint')}
                >
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
                <div className="px-4 pb-3 pt-1">
                  <Textarea
                    rows={4}
                    value={form.appExclusionCommands}
                    placeholder={'zoom.us\nmeet.google.com\nobs'}
                    onChange={(event) => updateForm('appExclusionCommands', event.target.value)}
                    className="w-full text-[12px]"
                  />
                </div>
              </SettingsCard>
            </div>

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
                <SettingsRow label={t(language, 'ui.language')}>
                  <SegmentedControl
                    ariaLabel={t(language, 'ui.language')}
                    value={form.language}
                    options={DESKTOP_LANGUAGE_CONFIGS.map((config) => ({
                      value: config.code,
                      label: config.nativeLabel,
                    }))}
                    onChange={(next) => updateForm('language', next)}
                  />
                </SettingsRow>
              </SettingsCard>
            </div>
          </>
        )

      case 'schedule':
      default:
        return (
          <>
            <div>
              <SectionLabel>{t(language, 'ui.scheduleControl')}</SectionLabel>
              <SettingsCard>
                <SettingsRow
                  label={t(language, 'runtime.tray.reset')}
                  detail={t(language, 'ui.resetScheduleHint')}
                >
                  <Button
                    type="button"
                    variant="secondary"
                    className="h-7 px-3 text-[12px]"
                    disabled={resetActionDisabled}
                    onClick={() =>
                      void runCommand(
                        'reset schedule',
                        'reset_breaks',
                        undefined,
                        resetPreviewSchedule,
                      )
                    }
                  >
                    {t(language, 'runtime.tray.reset')}
                  </Button>
                </SettingsRow>
              </SettingsCard>
            </div>

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
                unit={t(language, 'ui.suffix.cycles')}
                ariaLabel={`${t(language, 'ui.longBreaks')} ${t(language, 'ui.every')}`}
                value={form.longBreakEvery}
                options={LONGBREAK_EVERY_PRESETS}
                min={1}
                max={12}
                onChange={(value) => updateForm('longBreakEvery', value)}
              />
              <PresetChipRow
                label={t(language, 'ui.durationLabel')}
                unit={t(language, 'ui.suffix.minutes')}
                ariaLabel={`${t(language, 'ui.longBreaks')} ${t(language, 'ui.durationLabel')}`}
                value={form.longBreakDurationMinutes}
                options={LONGBREAK_DURATION_PRESETS}
                min={1}
                max={60}
                onChange={(value) => updateForm('longBreakDurationMinutes', value)}
              />
            </SchedulePresetCard>

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
          </>
        )
    }
  }

  return (
    <main className="flex h-screen flex-col select-none bg-background">
      <header className="shrink-0 px-5 pb-3 pt-4">
        <nav className="flex items-center justify-center gap-1">
          {categoryItems.map((item) => (
            <button
              key={item.id}
              type="button"
              onClick={() => setActiveCategory(item.id)}
              className={cn(
                'rounded-full px-3.5 py-1.5 text-[13px] font-medium transition-all',
                item.id === activeCategory
                  ? 'bg-white text-foreground shadow-[0_1px_3px_rgba(0,0,0,0.08),0_0_0_0.5px_rgba(0,0,0,0.04)]'
                  : 'text-muted-foreground hover:text-foreground',
              )}
            >
              {item.title}
            </button>
          ))}
        </nav>
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

          {renderActiveCategory()}
        </div>
      </div>
    </main>
  )
}

export default App
