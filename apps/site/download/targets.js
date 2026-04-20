// Homepage download button first tries GitHub latest release assets,
// then falls back to the pinned stable URL below if the API is unavailable.
window.PAUZA_DOWNLOAD_TARGETS = Object.freeze({
  macosAppleSilicon: Object.freeze({
    fallbackUrl: 'https://github.com/IHKYoung/Pauza/releases/download/v0.1.3/Pauza_0.1.3_aarch64.dmg',
    latestReleaseApi: 'https://api.github.com/repos/IHKYoung/Pauza/releases/latest',
    assetNameSuffix: '_aarch64.dmg',
  }),
})
