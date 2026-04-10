import crystalGlassUrl from '@/assets/audio/crystal-glass.wav'
import reverieUrl from '@/assets/audio/reverie.wav'
import ticTocUrl from '@/assets/audio/tic-toc.wav'
import windChimeUrl from '@/assets/audio/wind-chime.wav'

export type BreakKind = 'microbreak' | 'longBreak'
export type BreakBackdrop = 'paper' | 'dawn' | 'forest' | 'night' | 'custom'
export type BreakSound = 'silence' | 'crystal-glass' | 'wind-chime' | 'tic-toc' | 'reverie'
export type BreakContrastMode = 'light' | 'dark'

export type BreakScene = {
  background: string
  texture: string
  orbOne: string
  orbTwo: string
  accent: string
  accentSoft: string
  cueBackground: string
  cueBorder: string
  cueTagBackground: string
  cueTagText: string
  meterTrack: string
  meterFill: string
  contrastMode: BreakContrastMode
}

type BuiltinBreakBackdrop = Exclude<BreakBackdrop, 'custom'>

const BREAK_SCENES: Record<BuiltinBreakBackdrop, BreakScene> = {
  paper: {
    background:
      'linear-gradient(180deg, rgba(237,245,250,1) 0%, rgba(247,251,255,1) 48%, rgba(255,250,243,1) 100%)',
    texture:
      'radial-gradient(circle at 20% 18%, rgba(255,255,255,0.9) 0%, rgba(255,255,255,0) 36%), radial-gradient(circle at 84% 10%, rgba(213,226,241,0.6) 0%, rgba(213,226,241,0) 28%), linear-gradient(135deg, rgba(255,255,255,0.18) 25%, rgba(255,255,255,0) 25%)',
    orbOne:
      'radial-gradient(circle at center, rgba(114,151,202,0.18) 0%, rgba(114,151,202,0) 68%)',
    orbTwo:
      'radial-gradient(circle at center, rgba(238,204,170,0.22) 0%, rgba(238,204,170,0) 72%)',
    accent: '#385389',
    accentSoft: 'rgba(56,83,137,0.18)',
    cueBackground: 'rgba(255,255,255,0.68)',
    cueBorder: 'rgba(56,83,137,0.14)',
    cueTagBackground: 'rgba(56,83,137,0.08)',
    cueTagText: '#385389',
    meterTrack: 'rgba(56,83,137,0.12)',
    meterFill: '#385389',
    contrastMode: 'light',
  },
  dawn: {
    background:
      'linear-gradient(145deg, rgba(255,240,218,1) 0%, rgba(249,216,206,1) 38%, rgba(233,236,255,1) 100%)',
    texture:
      'radial-gradient(circle at 15% 24%, rgba(255,255,255,0.75) 0%, rgba(255,255,255,0) 34%), radial-gradient(circle at 84% 16%, rgba(255,206,176,0.42) 0%, rgba(255,206,176,0) 24%)',
    orbOne:
      'radial-gradient(circle at center, rgba(255,162,119,0.24) 0%, rgba(255,162,119,0) 66%)',
    orbTwo:
      'radial-gradient(circle at center, rgba(129,131,209,0.22) 0%, rgba(129,131,209,0) 72%)',
    accent: '#a65c4c',
    accentSoft: 'rgba(166,92,76,0.18)',
    cueBackground: 'rgba(255,250,247,0.72)',
    cueBorder: 'rgba(166,92,76,0.14)',
    cueTagBackground: 'rgba(166,92,76,0.08)',
    cueTagText: '#a65c4c',
    meterTrack: 'rgba(166,92,76,0.12)',
    meterFill: '#a65c4c',
    contrastMode: 'light',
  },
  forest: {
    background:
      'linear-gradient(145deg, rgba(239,247,241,1) 0%, rgba(214,236,223,1) 42%, rgba(191,216,203,1) 100%)',
    texture:
      'radial-gradient(circle at 18% 14%, rgba(255,255,255,0.72) 0%, rgba(255,255,255,0) 30%), radial-gradient(circle at 86% 18%, rgba(144,181,159,0.36) 0%, rgba(144,181,159,0) 26%)',
    orbOne:
      'radial-gradient(circle at center, rgba(84,132,104,0.26) 0%, rgba(84,132,104,0) 70%)',
    orbTwo:
      'radial-gradient(circle at center, rgba(235,244,201,0.22) 0%, rgba(235,244,201,0) 76%)',
    accent: '#2f6a4f',
    accentSoft: 'rgba(47,106,79,0.18)',
    cueBackground: 'rgba(248,252,249,0.74)',
    cueBorder: 'rgba(47,106,79,0.14)',
    cueTagBackground: 'rgba(47,106,79,0.08)',
    cueTagText: '#2f6a4f',
    meterTrack: 'rgba(47,106,79,0.12)',
    meterFill: '#2f6a4f',
    contrastMode: 'light',
  },
  night: {
    background:
      'linear-gradient(150deg, rgba(15,23,47,1) 0%, rgba(24,37,84,1) 42%, rgba(25,56,116,1) 100%)',
    texture:
      'radial-gradient(circle at 20% 18%, rgba(117,147,255,0.26) 0%, rgba(117,147,255,0) 30%), radial-gradient(circle at 82% 16%, rgba(88,190,255,0.18) 0%, rgba(88,190,255,0) 24%)',
    orbOne:
      'radial-gradient(circle at center, rgba(73,118,245,0.3) 0%, rgba(73,118,245,0) 70%)',
    orbTwo:
      'radial-gradient(circle at center, rgba(73,186,255,0.18) 0%, rgba(73,186,255,0) 76%)',
    accent: '#8fb4ff',
    accentSoft: 'rgba(143,180,255,0.2)',
    cueBackground: 'rgba(255,255,255,0.12)',
    cueBorder: 'rgba(143,180,255,0.18)',
    cueTagBackground: 'rgba(143,180,255,0.14)',
    cueTagText: '#dbe7ff',
    meterTrack: 'rgba(255,255,255,0.14)',
    meterFill: '#8fb4ff',
    contrastMode: 'dark',
  },
}

const BREAK_SOUND_URLS: Record<Exclude<BreakSound, 'silence'>, string> = {
  'crystal-glass': crystalGlassUrl,
  reverie: reverieUrl,
  'tic-toc': ticTocUrl,
  'wind-chime': windChimeUrl,
}

export const BREAK_BACKDROP_OPTIONS: Array<{ value: BreakBackdrop; labelKey: string }> = [
  { value: 'paper', labelKey: 'ui.breakBackdropPaper' },
  { value: 'dawn', labelKey: 'ui.breakBackdropDawn' },
  { value: 'forest', labelKey: 'ui.breakBackdropForest' },
  { value: 'night', labelKey: 'ui.breakBackdropNight' },
  { value: 'custom', labelKey: 'ui.breakBackdropCustom' },
]

export const BREAK_SOUND_OPTIONS: Array<{ value: BreakSound; labelKey: string }> = [
  { value: 'silence', labelKey: 'ui.breakSoundSilence' },
  { value: 'crystal-glass', labelKey: 'ui.breakSoundCrystalGlass' },
  { value: 'wind-chime', labelKey: 'ui.breakSoundWindChime' },
  { value: 'tic-toc', labelKey: 'ui.breakSoundTicToc' },
  { value: 'reverie', labelKey: 'ui.breakSoundReverie' },
]

export function getBreakScene(backdrop: BreakBackdrop): BreakScene {
  return BREAK_SCENES[backdrop === 'custom' ? 'paper' : backdrop]
}

export async function resolveBreakContrastMode(
  backdrop: BreakBackdrop,
  customBackdropDataUrl?: string | null,
): Promise<BreakContrastMode> {
  if (backdrop !== 'custom' || !customBackdropDataUrl) {
    return getBreakScene(backdrop).contrastMode
  }

  try {
    const image = await loadImage(customBackdropDataUrl)
    const sampleSize = 24
    const canvas = document.createElement('canvas')
    canvas.width = sampleSize
    canvas.height = sampleSize

    const context = canvas.getContext('2d', { willReadFrequently: true })
    if (!context) {
      return 'light'
    }

    context.drawImage(image, 0, 0, sampleSize, sampleSize)
    const { data } = context.getImageData(0, 0, sampleSize, sampleSize)

    let weightedLuminance = 0
    let alphaTotal = 0
    for (let index = 0; index < data.length; index += 4) {
      const alpha = data[index + 3] / 255
      if (alpha <= 0) {
        continue
      }

      const red = data[index]
      const green = data[index + 1]
      const blue = data[index + 2]
      const luminance = 0.2126 * red + 0.7152 * green + 0.0722 * blue
      weightedLuminance += luminance * alpha
      alphaTotal += alpha
    }

    if (alphaTotal <= 0) {
      return 'light'
    }

    return weightedLuminance / alphaTotal >= 148 ? 'light' : 'dark'
  } catch {
    return 'light'
  }
}

export function getBreakSoundUrl(sound: BreakSound): string | null {
  if (sound === 'silence') {
    return null
  }
  return BREAK_SOUND_URLS[sound]
}

function loadImage(url: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const image = new Image()
    image.onload = () => resolve(image)
    image.onerror = () => reject(new Error('Failed to decode custom backdrop image'))
    image.src = url
  })
}

export async function prepareCustomBackdrop(file: File): Promise<string> {
  const fileUrl = URL.createObjectURL(file)

  try {
    const image = await loadImage(fileUrl)
    const longestEdge = Math.max(image.naturalWidth, image.naturalHeight, 1)
    const scale = Math.min(1, 2200 / longestEdge)
    const width = Math.max(1, Math.round(image.naturalWidth * scale))
    const height = Math.max(1, Math.round(image.naturalHeight * scale))

    const canvas = document.createElement('canvas')
    canvas.width = width
    canvas.height = height

    const context = canvas.getContext('2d')
    if (!context) {
      throw new Error('Canvas rendering is unavailable for custom backdrop processing')
    }

    context.fillStyle = '#f6f7fb'
    context.fillRect(0, 0, width, height)
    context.drawImage(image, 0, 0, width, height)

    return canvas.toDataURL('image/jpeg', 0.88)
  } finally {
    URL.revokeObjectURL(fileUrl)
  }
}
