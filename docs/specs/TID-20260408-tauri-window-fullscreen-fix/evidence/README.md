# Task-ID: TID-20260408-tauri-window-fullscreen-fix Evidence

## Coverage
- 证据目标：
  - 证明主窗口最小尺寸已提升到 `800x600`
  - 证明 break windowed 宿主逻辑已从“microbreak 右下角小浮窗”改为“统一居中且约占工作区 `80% x 80%`”
  - 证明 break fullscreen 宿主逻辑已从“直接 native fullscreen”改为“macOS simple fullscreen / 其他平台 fullscreen”
  - 证明 close path 会在 destroy 前退出 simple/native fullscreen

## Available Evidence
- 代码路径：
  - `apps/desktop/src-tauri/tauri.conf.json`
  - `apps/desktop/src-tauri/src/shell.rs`
- 构建验证：
  - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `npm --prefix apps/desktop run build`

## Gaps
- 本轮未采集新的 macOS windowed/fullscreen break 真实截图或录屏。
- 最终视觉确认仍建议在用户本机手动分别触发一次 windowed / fullscreen break，重点观察 windowed 是否稳定居中且接近工作区 `80%`，以及 fullscreen 顶部空白和退出过渡。
