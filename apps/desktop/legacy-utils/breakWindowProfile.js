function clamp (value, min, max) {
  return Math.min(max, Math.max(min, Math.round(value)))
}

function normalizeBreakPromptStyle (style) {
  if (style === 'balanced' || style === 'immersive' || style === 'gentle') {
    return style
  }
  return 'gentle'
}

function centeredPosition (bounds, width, height) {
  return {
    x: Math.floor(bounds.x + ((bounds.width - width) / 2)),
    y: Math.floor(bounds.y + ((bounds.height - height) / 2))
  }
}

function bottomRightPosition (bounds, width, height, offset = 28) {
  return {
    x: Math.floor(bounds.x + bounds.width - width - offset),
    y: Math.floor(bounds.y + bounds.height - height - offset)
  }
}

function getBreakWindowProfile ({
  breakType,
  bounds,
  breakPromptStyle,
  breakWindowWidth,
  breakWindowHeight,
  fullscreen,
  showBreaksAsRegularWindows
}) {
  const style = normalizeBreakPromptStyle(breakPromptStyle)
  const isMicrobreak = breakType === 'microbreak'

  if (style === 'gentle') {
    const width = isMicrobreak
      ? clamp(bounds.width * 0.28, 320, 420)
      : clamp(bounds.width * 0.46, 520, 720)
    const height = isMicrobreak
      ? clamp(bounds.height * 0.24, 220, 280)
      : clamp(bounds.height * 0.50, 400, 560)
    const position = isMicrobreak
      ? bottomRightPosition(bounds, width, height)
      : centeredPosition(bounds, width, height)

    return {
      style,
      width,
      height,
      x: position.x,
      y: position.y,
      frame: false,
      transparent: true,
      skipTaskbar: true,
      focusable: false,
      alwaysOnTop: true,
      fullscreen: false,
      kiosk: false,
      showInactive: true
    }
  }

  if (style === 'balanced') {
    const width = isMicrobreak
      ? clamp(bounds.width * 0.48, 460, 700)
      : clamp(bounds.width * 0.62, 640, 880)
    const height = isMicrobreak
      ? clamp(bounds.height * 0.32, 280, 380)
      : clamp(bounds.height * 0.60, 480, 720)
    const position = centeredPosition(bounds, width, height)

    return {
      style,
      width,
      height,
      x: position.x,
      y: position.y,
      frame: false,
      transparent: true,
      skipTaskbar: true,
      focusable: false,
      alwaysOnTop: true,
      fullscreen: false,
      kiosk: false,
      showInactive: true
    }
  }

  const width = fullscreen ? bounds.width : Math.floor(bounds.width * breakWindowWidth)
  const height = fullscreen ? bounds.height : Math.floor(bounds.height * breakWindowHeight)
  const position = fullscreen
    ? { x: Math.floor(bounds.x), y: Math.floor(bounds.y) }
    : centeredPosition(bounds, width, height)

  return {
    style,
    width,
    height,
    x: position.x,
    y: position.y,
    frame: showBreaksAsRegularWindows,
    transparent: !showBreaksAsRegularWindows,
    skipTaskbar: !showBreaksAsRegularWindows,
    focusable: showBreaksAsRegularWindows,
    alwaysOnTop: !showBreaksAsRegularWindows,
    fullscreen,
    kiosk: fullscreen && !showBreaksAsRegularWindows,
    showInactive: !showBreaksAsRegularWindows
  }
}

export {
  normalizeBreakPromptStyle,
  getBreakWindowProfile
}
