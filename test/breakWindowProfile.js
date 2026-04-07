import 'chai/register-should'
import { getBreakWindowProfile, normalizeBreakPromptStyle } from '../app/utils/breakWindowProfile.js'

describe('breakWindowProfile', function () {
  const bounds = { x: 0, y: 0, width: 1920, height: 1080 }

  it('normalizes unknown styles to gentle', function () {
    normalizeBreakPromptStyle('unexpected').should.equal('gentle')
    normalizeBreakPromptStyle(undefined).should.equal('gentle')
  })

  it('returns a compact non-focusable profile for gentle microbreaks', function () {
    const profile = getBreakWindowProfile({
      breakType: 'microbreak',
      bounds,
      breakPromptStyle: 'gentle',
      breakWindowWidth: 0.85,
      breakWindowHeight: 0.85,
      fullscreen: false,
      showBreaksAsRegularWindows: false
    })

    profile.style.should.equal('gentle')
    profile.frame.should.equal(false)
    profile.focusable.should.equal(false)
    profile.showInactive.should.equal(true)
    profile.width.should.equal(420)
    profile.height.should.equal(259)
    profile.x.should.equal(1472)
    profile.y.should.equal(793)
  })

  it('returns a centered balanced profile for long breaks', function () {
    const profile = getBreakWindowProfile({
      breakType: 'break',
      bounds,
      breakPromptStyle: 'balanced',
      breakWindowWidth: 0.85,
      breakWindowHeight: 0.85,
      fullscreen: false,
      showBreaksAsRegularWindows: false
    })

    profile.style.should.equal('balanced')
    profile.width.should.equal(880)
    profile.height.should.equal(648)
    profile.x.should.equal(520)
    profile.y.should.equal(216)
    profile.alwaysOnTop.should.equal(true)
  })

  it('keeps immersive mode compatible with legacy window settings', function () {
    const profile = getBreakWindowProfile({
      breakType: 'break',
      bounds,
      breakPromptStyle: 'immersive',
      breakWindowWidth: 0.85,
      breakWindowHeight: 0.85,
      fullscreen: true,
      showBreaksAsRegularWindows: true
    })

    profile.style.should.equal('immersive')
    profile.width.should.equal(1920)
    profile.height.should.equal(1080)
    profile.frame.should.equal(true)
    profile.focusable.should.equal(true)
    profile.skipTaskbar.should.equal(false)
  })
})
