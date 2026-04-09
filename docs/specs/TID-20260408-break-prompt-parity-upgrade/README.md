# Task-ID: TID-20260408-break-prompt-parity-upgrade

## Meta
- Title: 补齐 Tauri break prompt 的原版体验能力
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
- Evidence: ./evidence/README.md

## Decision Log
- 以旧 Electron `break.html / microbreak.html / break-renderer.js / microbreak-renderer.js / app/main.js` 作为体验基线，优先补回随机交互语、开始音、线性倒计时和更完整的 break 主视觉。
- 休息背景继续保留预设主题，但新增真正的本地自定义壁纸上传能力；图片在前端压缩后以 data URL 写入 `settings.json`，避免当前 Tauri 壳缺少 `assetProtocol` / dialog plugin 时无法稳定跨会话复用本地路径。
- 直接复用旧版 `app/audio/*.wav` 作为 Tauri break start sound 资源，并按微休息 / 休息分别配置，避免重新引入声音资产和额外依赖。

## Governance Notes
- Requirement Brief: 用户明确指出当前 Tauri break window 相比原版明显缺少“自定义壁纸、声音、随机交互语和更像原版的倒数条”，要求沿当前 break prompt 继续优化补齐，但不回退到 Electron 壳或改 reminder / window shell 状态机。
- Interaction Impact: direct
- Interaction Freeze: 设置页仅扩展 break surface 相关控制项；break prompt 仅增强背景、提示卡、开始音和线性倒计时，不改 Done / Later / Skip 语义与 break 生命周期。
- Execution Safety Block: service_impact=本地桌面前台与设置 schema；touches_running_service=no；backup_required=no；backup_plan=依赖 VCS、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` 与 Playwright 截图证据；rollback_plan=回退 `apps/desktop/src/App.tsx`、`apps/desktop/src/lib/break-prompt.ts`、`apps/desktop/src/locales/*.json`、`apps/desktop/src-tauri/src/state.rs` 与本任务 docs；destructive_operations=替换当前 break surface 配置项与 break prompt 视觉结构；operator_approval_required=no；rationale=纯本地桌面应用体验补齐，无数据迁移、提权或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；当前 session 的 developer policy 不允许未显式授权的 sub-agent delegation，因此由 orchestrator 直接完成代码、验证与文档闭环。
- Retention Decision: keep

## Notes
- 自定义壁纸目前以压缩后的 data URL 保存在本地 `settings.json`，这是为了在不新增 Tauri plugin / asset protocol 配置的前提下让 break window 能稳定复用用户图片。
- 本轮补回的是“内置随机交互语 + 开关”；旧版可手写自定义 ideas 列表的能力暂未恢复，后续若要继续追 parity，需要单独设计设置模型和编辑体验。
