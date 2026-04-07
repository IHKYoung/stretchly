import { invoke } from '@tauri-apps/api/core'
import { BellRing } from 'lucide-react'
import { useEffect, useState, type ReactNode } from 'react'

import { Badge } from '@/components/ui/badge'
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
import { cn } from '@/lib/utils'
import { formatDuration, normalizeLanguage, t, type AppLanguage } from './i18n'

type BreakKind = 'microbreak' | 'longBreak'
type AppExclusionRule = 'pause' | 'resume'
type BreakPromptStyle = 'gentle' | 'balanced' | 'immersive'
type TargetScreen = 'primary' | 'cursor'
type SettingsCategory = 'schedule' | 'preferences'

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
  microbreakStrictMode: boolean
  microbreakManualFinish: boolean
  longBreakEnabled: boolean
  longBreakEvery: number
  longBreakDurationMinutes: number
  longBreakNotificationEnabled: boolean
  longBreakNotificationSeconds: number
  longBreakAllowPostpone: boolean
  longBreakPostponeMinutes: number
  longBreakPostponesLimit: number
  longBreakStrictMode: boolean
  longBreakManualFinish: boolean
  naturalBreaks: boolean
  naturalBreakResetMinutes: number
  monitorDnd: boolean
  appExclusionsEnabled: boolean
  appExclusionRule: AppExclusionRule
  appExclusionCommands: string
  breakPromptStyle: BreakPromptStyle
  fullscreen: boolean
  showBreaksOnAllScreens: boolean
  targetScreen: TargetScreen
  currentTimeInBreaks: boolean
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
    microbreakStrictMode: false,
    microbreakManualFinish: false,
    longBreakEnabled: true,
    longBreakEvery: 3,
    longBreakDurationMinutes: 5,
    longBreakNotificationEnabled: true,
    longBreakNotificationSeconds: 30,
    longBreakAllowPostpone: true,
    longBreakPostponeMinutes: 5,
    longBreakPostponesLimit: 1,
    longBreakStrictMode: false,
    longBreakManualFinish: false,
    naturalBreaks: true,
    naturalBreakResetMinutes: 5,
    monitorDnd: true,
    appExclusionsEnabled: false,
    appExclusionRule: 'pause',
    appExclusionCommands: '',
    breakPromptStyle: 'gentle',
    fullscreen: false,
    showBreaksOnAllScreens: true,
    targetScreen: 'primary',
    currentTimeInBreaks: false,
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
  const currentBreakDurationMs = settings.microbreakDurationSeconds * 1_000
  const previewBreakStartedAtMs = Date.now() - 6_000
  const previewBreakEndsAtMs = previewBreakStartedAtMs + currentBreakDurationMs

  return {
    productName: 'Pauza',
    runtime: 'Browser preview',
    platform: 'preview',
    appVersion: '0.1.0',
    autostartEnabled: false,
    settings,
    status: t(language, 'ui.preview.status'),
    statusDetail: t(language, 'ui.preview.detail'),
    nextBreakKind: 'microbreak',
    nextBreakDueMs: Date.now() + 8 * 60_000,
    nextBreakInMs: 8 * 60_000,
    currentBreak: breakMode
      ? {
          kind: 'microbreak',
          title: t(language, 'runtime.break.microbreak.title'),
          detail: t(language, 'runtime.break.microbreak.detail'),
          startedAtMs: previewBreakStartedAtMs,
          endsAtMs: previewBreakEndsAtMs,
          durationMs: currentBreakDurationMs,
          strictMode: false,
          manualAwaiting: false,
          canPostpone: true,
          canSkip: true,
          showClock: true,
        }
      : null,
    pauseUntilMs: null,
    pausedIndefinitely: false,
    focusUntilMs: null,
    idleMs: 0,
    dndActive: false,
    appExclusionActive: false,
    appExclusionMatch: null,
    lastAction: t(language, 'ui.preview.action'),
  }
}

function hasTauriRuntime() {
  return Boolean((window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__)
}

function isBreakWindow() {
  return new URLSearchParams(window.location.search).get('window') === 'break'
}

function clockLabel(language: AppLanguage, value = Date.now()) {
  return new Intl.DateTimeFormat(language === 'zh-CN' ? 'zh-CN' : 'en-US', {
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
}: {
  busyAction: string | null
  currentBreak: CurrentBreakSnapshot | null
  language: AppLanguage
  runCommand: CommandFn
}) {
  const remaining = currentBreak ? Math.max(currentBreak.endsAtMs - Date.now(), 0) : 0
  const progress = currentBreak
    ? currentBreak.manualAwaiting
      ? 1
      : Math.max(0, Math.min(1, remaining / Math.max(currentBreak.durationMs, 1)))
    : 0
  const breakKindLabel = currentBreak
    ? t(
        language,
        currentBreak.kind === 'microbreak' ? 'ui.status.microbreak' : 'ui.status.longBreak',
      )
    : null
  const circleSizeClass =
    currentBreak?.kind === 'longBreak'
      ? 'size-52 sm:size-60 md:size-64'
      : 'size-44 sm:size-52 md:size-56'
  const surfacePaddingClass =
    currentBreak?.kind === 'longBreak' ? 'px-8 py-8 sm:px-10 sm:py-10' : 'px-6 py-6 sm:px-8 sm:py-8'

  return (
    <main className="min-h-screen bg-[linear-gradient(180deg,#edf5fa_0%,#f7fbff_100%)]">
      <section className={cn('mx-auto flex min-h-screen w-full max-w-[980px] flex-col animate-surface-in', surfacePaddingClass)}>
        <div className="flex items-center justify-between gap-3">
          <Badge variant={currentBreak?.manualAwaiting ? 'warning' : 'secondary'}>
            {breakKindLabel ?? 'Pauza'}
          </Badge>
          {currentBreak?.showClock ? (
            <span className="text-sm text-muted-foreground">{clockLabel(language)}</span>
          ) : null}
        </div>

        <div className="flex flex-1 flex-col justify-center">
          <div className="mx-auto flex w-full max-w-[760px] flex-col gap-8">
            <div className="flex justify-center">
              <div
                className={cn(
                  'relative flex items-center justify-center rounded-full border border-slate-200 bg-white/70 p-4 shadow-[0_30px_70px_-40px_rgba(15,23,42,0.28)]',
                  circleSizeClass,
                )}
                style={{
                  backgroundImage: `conic-gradient(rgba(31,58,120,0.92) ${Math.round(
                    progress * 360,
                  )}deg, rgba(205,214,229,0.42) ${Math.round(progress * 360)}deg 360deg)`,
                }}
              >
                <div className="absolute inset-3 rounded-full bg-white" />
                <div className="relative text-center">
                  <BellRing className="mx-auto size-5 text-primary/70" />
                  <div className="mt-4 text-4xl font-semibold tracking-[-0.05em] text-slate-950 sm:text-5xl">
                    {currentBreak?.manualAwaiting
                      ? t(language, 'ui.break.awaitingFinish')
                      : formatDuration(remaining, language)}
                  </div>
                  <p className="mt-2 text-sm text-muted-foreground">
                    {currentBreak?.manualAwaiting
                      ? t(language, 'ui.break.manualAwaiting')
                      : t(language, 'ui.break.remainingLabel')}
                  </p>
                </div>
              </div>
            </div>

            <div className="space-y-3 text-center">
              <h1 className="text-[2rem] font-semibold tracking-[-0.05em] text-slate-950 sm:text-[2.4rem]">
                {currentBreak?.title ?? t(language, 'ui.break.clearedTitle')}
              </h1>
              <p className="mx-auto max-w-[34rem] text-base leading-7 text-muted-foreground sm:text-lg">
                {currentBreak?.manualAwaiting
                  ? t(language, 'ui.break.manualAwaiting')
                  : currentBreak?.detail ?? t(language, 'ui.break.clearedDetail')}
              </p>
            </div>
          </div>
        </div>

        <div className="mx-auto w-full max-w-[760px]">
          <div className="flex flex-col gap-3 sm:flex-row">
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
                ? t(language, 'ui.break.resumeWork')
                : t(language, 'ui.break.done')}
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
                {t(language, 'ui.break.later')}
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
                {t(language, 'ui.break.skip')}
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
  const [error, setError] = useState<string | null>(null)
  const [activeCategory, setActiveCategory] = useState<SettingsCategory>('schedule')

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
      />
    )
  }

  const categoryItems: Array<{ id: SettingsCategory; title: string }> = [
    { id: 'schedule', title: t(language, 'ui.schedule') },
    { id: 'preferences', title: t(language, 'ui.preferences') },
  ]

  const renderActiveCategory = () => {
    switch (activeCategory) {
      case 'preferences':
        return (
          <>
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
                <SettingsRow label={t(language, 'ui.breakFeelLabel')}>
                  <SegmentedControl
                    ariaLabel={t(language, 'ui.breakFeelLabel')}
                    value={form.breakPromptStyle}
                    options={[
                      { value: 'gentle', label: t(language, 'ui.promptStyleGentle') },
                      { value: 'balanced', label: t(language, 'ui.promptStyleBalanced') },
                      { value: 'immersive', label: t(language, 'ui.promptStyleImmersive') },
                    ]}
                    onChange={(next) => updateForm('breakPromptStyle', next)}
                  />
                </SettingsRow>
              </SettingsCard>
            </div>

            <div>
              <SectionLabel>{t(language, 'ui.strictModes')}</SectionLabel>
              <SettingsCard>
                <SettingsRow label={t(language, 'ui.microbreaks')}>
                  <Switch
                    checked={form.microbreakStrictMode}
                    onCheckedChange={(next) => updateForm('microbreakStrictMode', next)}
                    aria-label={t(language, 'ui.microbreakStrict')}
                  />
                </SettingsRow>
                <SettingsRow label={t(language, 'ui.longBreaks')}>
                  <Switch
                    checked={form.longBreakStrictMode}
                    onCheckedChange={(next) => updateForm('longBreakStrictMode', next)}
                    aria-label={t(language, 'ui.longBreakStrict')}
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
                    options={[
                      { value: 'zh-CN', label: t(language, 'ui.languageChinese') },
                      { value: 'en', label: t(language, 'ui.languageEnglish') },
                    ]}
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
        <div className="space-y-3">{renderActiveCategory()}</div>
      </div>
    </main>
  )
}

export default App
