# Task-ID: TID-20260408-macos-icon-fidelity

## Meta
- Title: 修复 macOS 顶部与 Dock 图标清晰度和尺寸
- Date: 2026-04-08
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-08.md
- Log (daily): ../../logs/2026-04-08.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 2026-04-08：确认根因不只是源 SVG，而是 `tray-icon` crate 在 macOS 上固定使用 `18pt` 逻辑尺寸，且现有 patch 通过枚举全部 status items 替换图像，风险高且不稳定。
- 2026-04-08：Dock icon 改为直接使用 bundled `icon.icns` 的 multi-representation 资源，不再以原始 `icon.png` 手工覆盖应用图标。
- 2026-04-08：图标生成脚本一并修复，以免后续继续在 ffmpeg 原地覆盖与 `ensure_srgb` 调用签名错误上反复卡住。
- 2026-04-08：根据用户追加反馈，Dock icon 进一步收紧为“release `.app` 不再执行 runtime patch”，并给 `app-icon.svg` 增加 `64px` 安全边距。

## Governance Notes
- Requirement Brief: 解决 macOS tray icon 发糊 / 偏大与 Dock icon 偏大的问题；只改宿主图标接入、tray 资源几何和资源生成链，不动业务逻辑。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限 macOS 图标显示与本地资源生成链；touches_running_service=no；backup_required=no；backup_plan=以 Git diff、重新生成资源和构建链结果为边界；rollback_plan=回退 `shell.rs`、`graphics/*` 与再生的 icon 产物；destructive_operations=替换 tray / dock 图标接入方式并刷新 icon 资源；operator_approval_required=no；rationale=不涉及数据、权限、外部副作用或提权。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；主要限制是当前终端未授予 Screen Recording，导致系统级截图验证被阻塞。
- Retention Decision: keep

## Notes
- 视觉最终核验仍建议在用户本机授予 Screen Recording 后补一张菜单栏 / Dock 截图，但这不阻塞本轮代码与资源链修复收口。

## Outcome Summary
- 顶部 tray icon：
  - 不再复用默认 app icon，而是改走独立的 `graphics/tray-icon.svg`
  - 宿主层改为只 patch Pauza 自己的 `NSStatusItem`
  - tray image 改为 template image，并显式提供 1x / 2x 表示
  - 根据真实菜单栏截图，又将 glyph 追加放大约 `8%`，把视觉体积拉回到更接近系统图标的级别
- Dock icon：
  - `graphics/app-icon.svg` 增加 `64px` 安全边距，收紧 Dock 中的视觉重量
  - release `.app` 不再执行 Dock runtime patch，正式产物直接使用 bundle 内 `icon.icns`
- 生成链：
  - `graphics/generate_icon_assets.py` 修复了 ffmpeg 原地覆盖失败和 `ensure_srgb()` 签名不一致问题
  - 图标修改后的标准流程固定为：改 `graphics/*.svg` -> 跑 `python3 graphics/generate_icon_assets.py` -> 跑 Tauri build
- 根因总结：
  - 之前把问题当成“替换 PNG”处理，但真实根因是小尺寸几何、macOS 原生 tray 渲染路径、Dock app icon 构图和生成脚本稳定性四层问题叠加
