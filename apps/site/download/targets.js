// Homepage download button resolves the latest release asset name through GitHub's API,
// then builds a releases/latest/download URL. The pinned URL is the failure fallback.
window.PAUZA_DOWNLOAD_TARGETS = Object.freeze({
  macosAppleSilicon: Object.freeze({
    fallbackUrl: 'https://github.com/IHKYoung/Pauza/releases/download/v0.1.5/Pauza_0.1.5_aarch64.dmg',
    latestReleaseApi: 'https://api.github.com/repos/IHKYoung/Pauza/releases/latest',
    latestDownloadBaseUrl: 'https://github.com/IHKYoung/Pauza/releases/latest/download',
    assetNameSuffix: '_aarch64.dmg',
  }),
})
