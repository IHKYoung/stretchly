# Task-ID: TID-20260408-break-prompt-parity-upgrade

## Summary
- Title: 补齐 Tauri break prompt 的原版体验能力
- Date: 2026-04-08
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 让当前 Tauri break prompt 重新具备接近原版的“休息氛围”和可配置能力，至少补回背景主题/自定义壁纸、开始音、随机交互语和线性倒计时，同时保持现有 Tauri reminder/window shell 逻辑不回退。
- In-scope:
  - `apps/desktop/src/App.tsx` 的 break prompt 结构和偏好页 break surface 配置项
  - `apps/desktop/src/lib/break-prompt.ts` 的 cue / background / sound helper
  - `apps/desktop/src/assets/audio/*.wav` 资源接入
  - `apps/desktop/src-tauri/src/state.rs` 的设置 schema 与 legacy migration
  - 对应 locale 与 workflow/docs
- Out-of-scope:
  - reminder state machine、window shell、fullscreen / window 宿主逻辑
  - 用户自定义 cue 列表编辑器
  - 重新引入 Electron 的旧页面结构
- Assumptions:
  - 旧版原始体验基线主要由 `app/break.html`、`app/microbreak.html`、`app/*break-renderer.js`、`app/main.js` 和默认 ideas / audio 资源构成
  - 当前 Tauri 壳未配置 `assetProtocol` 和 dialog plugin，因此本地壁纸若要跨会话稳定使用，应直接以压缩后的 data URL 存储在 settings 中
  - 浏览器 / Tauri webview 的一次性 `Audio` 播放足以覆盖“开始音”需求
- Risks:
  - 自定义壁纸会增大 `settings.json`
  - webview 对音频自动播放的策略可能受平台影响
  - 本轮仍未恢复“用户自定义 cue 列表”，只提供内置随机交互语
- Interaction impact: direct
- Primary visible flow: 用户在偏好页选择背景主题或上传自定义壁纸、配置开始音和随机交互语后，下一次 break 会以更完整的主视觉、提示卡和线性倒计时出现。
- Fallback / secondary flow: 若未上传自定义壁纸或所选图片被移除，break prompt 自动回退到预设主题；若选择静音，break 不播放进场音。
- User-visible boundary: 偏好页的 break surface / break sounds 配置区，以及 `?window=break` break prompt 的视觉结构与内容模块。
- Key visible states / transitions:
  - 预设背景 -> 自定义壁纸
  - cue enabled -> cue disabled
  - sound selected -> silence
  - break running -> linear countdown + circular countdown 同步更新

## Goal
- 让 Tauri break prompt 至少恢复到“可用且有质感”的原版体验，而不是只剩一个基础圆环计时器。

## Scope
- In-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/lib/break-prompt.ts`
  - `apps/desktop/src/assets/audio/*.wav`
  - `apps/desktop/src/locales/{zh-CN,en}.json`
  - `apps/desktop/src-tauri/src/state.rs`
  - `docs/specs/TID-20260408-break-prompt-parity-upgrade/*`
  - `docs/{plans,logs}/2026-04-08.md`
  - `docs/{UI,Architecture,SettingsInventory,CodeMap,CHANGELOG}.md`
- Out-of-scope:
  - `apps/desktop/src-tauri/src/{engine,shell}.rs`
  - Electron legacy 页面本身
  - 新增外部依赖或 Tauri plugin

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `app/break.html`
  - `app/microbreak.html`
  - `app/break-renderer.js`
  - `app/microbreak-renderer.js`
  - `app/main.js`
  - `app/utils/defaultBreakIdeas.js`
  - `app/utils/defaultMicrobreakIdeas.js`
  - `app/utils/defaultSettings.js`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/state.rs`
- Related docs/specs/logs reviewed:
  - `docs/UI.md`
  - `docs/Architecture.md`
  - `docs/SettingsInventory.md`
  - `docs/logs/2026-04-08.md`
- Why these are sufficient:
  - 原版 break 体验的来源和配置项都集中在上述 Electron 文件
  - 当前 Tauri break surface、偏好页与设置持久化分别集中在 `App.tsx` 和 `state.rs`
  - 长期文档已能覆盖当前 Tauri 壳的行为边界，足够作为 parity 改造的基线

## Acceptance Criteria (AC)
- AC1: 偏好页新增 break 背景主题、自定义壁纸上传、随机交互语开关、开始音配置和音量控制。
- AC2: `PauzaSettings` 持久化 `breakBackdrop`、自定义壁纸信息、随机交互语开关、微休息/休息开始音和音量。
- AC3: break prompt 重新具备更有氛围的背景、随机交互语卡片和线性倒计时条，同时保留现有圆形计时器和 CTA。
- AC4: break 开始时按 break kind 播放一次开始音；选择静音或音量为 0 时不播放。
- AC5: 用户选择本地图片后，后续 break 可直接复用这张自定义壁纸，不依赖额外 plugin 或 asset protocol。
- AC6: `npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` 通过，并补齐 evidence / logs / plans。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen before code
- Primary flow: 用户在偏好页完成 break surface 配置后，下一次 break 以增强版 prompt 呈现
- Fallback / secondary flow: 无自定义壁纸时回退预设主题；静音时不播放开始音
- Interaction authority / ownership boundary: 只改 Tauri 前台设置页与 break prompt 组件，不改 Rust 调度状态机和宿主窗口策略
- Visible entrypoints / handoff cues: 偏好页 `休息窗口 / 休息音效` 区域；break 自动弹窗 `?window=break`
- In-scope interactions:
  - 选择背景主题
  - 上传 / 替换 / 移除自定义壁纸
  - 打开 / 关闭随机交互语
  - 选择微休息 / 休息开始音与音量
  - break 运行时查看 cue card、线性倒计时和圆形倒计时
- Out-of-scope interactions:
  - 修改 break CTA 语义
  - 改 reminder delivery / strict mode 逻辑
  - 编辑自定义 cue 列表
- Interaction acceptance criteria:
  - 偏好页可完成上述配置且自动保存
  - break prompt 默认态有 cue card 和线性倒计时
  - 自定义壁纸可被选中并在 break 里复用
  - 开始音只在 break 开始时播放一次

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,ui_designer,architect,coder,tester,scribe,evidence_collector
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 本地桌面前台 break prompt、偏好页和设置 schema
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 VCS、`typecheck`、前端 build、`cargo check` 和 evidence 截图
- rollback_plan: 回退 `apps/desktop/src/{App.tsx,lib/break-prompt.ts,locales/*,assets/audio/*}`、`apps/desktop/src-tauri/src/state.rs` 与本任务 docs
- destructive_operations: 替换当前 break prompt 视觉结构，扩展设置持久化字段
- operator_approval_required: no
- rationale: 纯本地桌面体验增强，不涉及数据迁移、提权、运行中服务或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 对齐 legacy break prompt 的功能缺口
  - DoD: 确认原版 cues、sounds、linear countdown 和背景能力的来源文件与迁移边界
- [x] Task-2: 扩展 Tauri break prompt 与偏好页
  - DoD: `App.tsx` 增加 cue card、线性倒计时、背景主题/自定义壁纸和开始音设置 UI
- [x] Task-3: 扩展 Tauri settings schema
  - DoD: `state.rs` 新字段可序列化、反序列化、sanitized 并兼容 legacy settings
- [x] Task-4: 补齐资源、多语言和证据
  - DoD: 音频资源、locale、evidence、changelog 和长期文档同步完成
- [x] Task-5: 完成机械验证
  - DoD: `typecheck`、build、`cargo check` 与 workflow docs validation 通过

## Evidence Plan (UI / E2E)
- Evidence required: partial
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260408-break-prompt-parity-upgrade/evidence/
- Interaction validation note: 当前已补 Playwright 截图用于证明增强版默认 break prompt；自定义壁纸的原生 Tauri 上传/运行时效果仍以代码路径和设置 UI 审查为主。
- Required states to capture:
  - 默认增强版 break prompt（cue card + linear countdown + circular timer）
  - 偏好页自定义壁纸上传入口与已选图片预览
  - 音量 / 开始音配置区
  - 静音或无自定义壁纸时的回退路径

## Observability / Debug Plan
- Logs: 本轮未新增 runtime logger；若后续自定义壁纸或开始音有问题，优先检查 `prepareCustomBackdrop()`、`BreakWindow` 音频 effect 和 `state.rs::sanitized()`
- Error codes: 沿用前端普通错误提示，无新增 error code
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 自定义壁纸图片过大时会让 settings 文件增大
  - 某些平台的 webview 可能更严格限制自动播放
  - cue 文案目前仍是内置静态池，不支持用户自定义
- Rollback plan:
  - 回退新增的 break prompt helper、settings 字段、locale 与 docs；若只需撤销自定义壁纸，可清空 `breakCustomBackdrop*` 字段并切回预设主题

## Sequential Phases
- phase_execution: N/A
- phase_confirmation_policy: N/A
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 以 legacy Electron break 体验作为 source basis，确认需要补回的 cue / sound / countdown / backdrop 能力
  2. 在 Tauri 前台新增 helper、音频资源和 break prompt 布局增强
  3. 扩展 settings schema 和 locale
  4. 采集默认 break prompt 截图证据
  5. 同步 specs / logs / plans / 长期文档并完成校验

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes
- Approved: 用户在当前线程明确要求“优化补充”当前 Tauri 休息窗口，并点名原版的自定义壁纸、声音、随机交互语与倒数条作为补齐目标
