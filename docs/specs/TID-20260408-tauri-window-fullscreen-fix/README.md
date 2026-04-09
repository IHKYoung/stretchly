# Task-ID: TID-20260408-tauri-window-fullscreen-fix

## Meta
- Title: 修复 Tauri 窗口尺寸边界与 break 宿主显示模式
- Date: 2026-04-08
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Links
- Plan (daily): ../../plans/2026-04-08.md
- Log (daily): ../../logs/2026-04-08.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 将主设置窗口的默认/最小尺寸收敛到 `960x640` / `800x600`，避免当前 `580x520` / `480x440` 的过小配置继续漂移。
- 将 break 的 windowed 宿主 profile 统一改为居中大窗，约占当前工作区 `80% x 80%`，不再保留 microbreak 右下角小浮窗分支。
- 将 break fullscreen 进入/退出统一收敛到 `set_break_window_fullscreen()`，macOS 改走 `set_simple_fullscreen()`，关闭时同时兜底退出 simple/native fullscreen。

## Governance Notes
- Requirement Brief: 用户明确指出 Tauri 打包后主窗口过小、window 模式 break 缩成右下角小浮窗，以及 fullscreen 顶部留白；本轮只修主窗口尺寸边界与 break 宿主显示策略，不改 break 页面视觉结构和业务状态机。
- Interaction Impact: direct
- Interaction Freeze: 主设置窗口只调整宿主尺寸边界；break windowed/fullscreen 只调整宿主尺寸、位置与 fullscreen 进入/退出策略，保持现有 CTA、内容和时序语义不变。
- Execution Safety Block: service_impact=仅桌面窗口宿主行为；touches_running_service=no；backup_required=no；backup_plan=依赖 VCS、`cargo check`、`npm --prefix apps/desktop run build` 与代码路径审查；rollback_plan=回退 `apps/desktop/src-tauri/{tauri.conf.json,src/shell.rs}` 与本任务 docs；destructive_operations=替换当前 break windowed/fullscreen 宿主策略和主窗口尺寸边界；operator_approval_required=no；rationale=纯本地桌面壳层修复，无数据、权限或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；Rust host 与前端构建均已通过，剩余缺口仅为 macOS 本机视觉确认。
- Retention Decision: keep

## Notes
- `cargo fmt --check` 在当前环境 BLOCKED，因为 toolchain 未安装 `cargo-fmt/rustfmt`；本轮以 `cargo check` 和前端构建作为机械验证。
- 最终视觉确认仍建议在你的 macOS 本机手动分别触发一次 windowed / fullscreen break，重点观察 windowed 是否稳定居中且接近工作区 `80%`，以及 fullscreen 顶部空白和关闭回到桌面的过渡。
