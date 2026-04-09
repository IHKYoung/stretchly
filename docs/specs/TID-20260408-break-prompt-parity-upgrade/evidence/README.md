# Task-ID: TID-20260408-break-prompt-parity-upgrade Evidence

## Coverage
- 证据目标：
  - 证明 Tauri break prompt 已恢复更完整的背景氛围、随机交互语和线性倒计时
  - 证明开始音资源已进入前台构建产物并可按 break kind 配置
  - 证明设置页已新增背景主题 / 自定义壁纸 / 声音相关配置

## Available Evidence
- UI 截图：
  - `docs/specs/TID-20260408-break-prompt-parity-upgrade/evidence/break-prompt-parity-preview.png`
- 构建验证：
  - `npm --prefix apps/desktop run typecheck`
  - `npm --prefix apps/desktop run build`
  - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- 代码路径：
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/lib/break-prompt.ts`
  - `apps/desktop/src-tauri/src/state.rs`

## Notes
- 当前截图展示的是默认增强版 break prompt：包含 cue card、线性倒计时、圆形倒计时与更完整背景氛围。
- 自定义壁纸上传和设置页 preview 本轮以代码路径审查为主，尚未额外采集原生 Tauri 截图。

## Gaps
- 仍缺少“用户上传自定义壁纸后在原生 Tauri break window 中展示”的最终视觉截图。
- 若后续要把本轮 parity 作为 release gate，建议再补一张带自定义壁纸的本机截图。
