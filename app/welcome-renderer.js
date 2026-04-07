import HtmlTranslate from './utils/htmlTranslate.js'
import { setSameWidths } from './utils/sameWidths.js'
import './platform.js'

let eventsAttached = false

window.onload = async (event) => {
  const settings = await window.settings.currentSettings()
  await new HtmlTranslate(document).translate()

  document.ondragover = event =>
    event.preventDefault()

  document.ondrop = event =>
    event.preventDefault()

  setTimeout(() => { eventsAttached = true }, 500)

  window.pauza.onTranslate(async () => {
    await new HtmlTranslate(document).translate()
    setTimeout(() => setSameWidths(), 100)
  })

  document.querySelectorAll('input[type="radio"]').forEach(radio => {
    let value
    switch (radio.value) {
      case 'true':
        value = true
        break
      case 'false':
        value = false
        break
      default:
        value = radio.value
    }
    radio.checked = settings[radio.name] === value
    if (!eventsAttached) {
      radio.onchange = (event) => {
        window.settings.saveSettings(radio.name, value)
      }
    }
  })

  document.querySelector('#language').value = settings.language
  document.querySelector('#language').focus()
  if (!eventsAttached) {
    document.querySelector('#language').onchange = (event) => {
      window.settings.saveSettings('language', event.target.value)
    }
  }

  setTimeout(() => setSameWidths(), 100)

  document.querySelectorAll('button').forEach(button => {
    if (!eventsAttached) {
      button.onclick = () => {
        window.settings.saveSettings('isFirstRun', false)
        switch (button.getAttribute('data-location')) {
          case 'tutorial':
            window.electronApi.openExternal('https://hovancik.net/stretchly/about')
            break
          case 'preferences':
            window.pauza.openPreferences()
            break
          default:
            break
        }
        window.pauza.closeWindow()
      }
    }
  })
}
