# Task-ID: TID-20260408-break-prompt-parity-upgrade

## Test Strategy
- Unit: 无新增独立单测；本轮主要是前台 UI / schema 补齐。
- Integration:
  - `npm --prefix apps/desktop run typecheck`
  - `npm --prefix apps/desktop run build`
  - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- E2E (if applicable):
  - Playwright 打开 `http://127.0.0.1:43179/?window=break`，采集默认增强版 break prompt 截图
  - 手工代码审查偏好页自定义壁纸上传入口与 preview 逻辑

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 审查 `apps/desktop/src/App.tsx`，确认偏好页新增背景主题、自定义壁纸、随机交互语、开始音和音量控件
- AC2 -> 审查 `apps/desktop/src-tauri/src/state.rs`，确认新 settings 字段已进入 `PauzaSettings`、default、sanitize 和 migration
- AC3 -> 通过 Playwright 截图 + `App.tsx` 审查，确认 break prompt 含 cue card、线性倒计时和圆形倒计时
- AC4 -> 审查 `BreakWindow` 的音频 effect，确认开始音按 break kind 只播放一次，且 `silence` / `volume=0` 不播放
- AC5 -> 审查 `prepareCustomBackdrop()` 和 settings 持久化字段，确认本地图片被压缩为 data URL 并可跨会话复用
- AC6 -> 运行 `typecheck`、前端 build、`cargo check` 和 workflow docs validation

## Interaction Contract Coverage
- Interaction impact: direct
- Primary flow -> tests/evidence: Playwright 截图 `docs/specs/TID-20260408-break-prompt-parity-upgrade/evidence/break-prompt-parity-preview.png` + `App.tsx` 审查，确认默认 break prompt 已恢复 cue card、linear meter 和更完整的背景氛围。
- Fallback / secondary flow -> tests/evidence: `App.tsx` + `state.rs` 审查，确认自定义壁纸为空时回退预设主题、`silence` 或音量为 0 时不播放开始音。
- Visible states / transitions -> tests/evidence: `BreakWindow` 中 `progress`、`manualAwaiting`、`lastPlayedBreakRef` 和 custom wallpaper preview 逻辑证明 break 运行态、静音态和无自定义壁纸回退态的可见行为都已定义。

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- `DONE` 任务对应的 spec 仍保留 `TBD/INIT` 占位。
- `orchestrator` 未显式使用 `sandbox_mode = "danger-full-access"` 与 `approval_policy = "never"`，却仍宣称当前仓库运行在 aggressive 基线。
- 代码变更前未记录 `Source Basis`，导致实现依据不可追溯。
- 子 agent 未显式 `approval_policy = "never"`。
- `moderate/complex` 任务通过缩小 `Required Roles` 伪装为 trivial fallback。
- 已使用 `single-agent-fallback`，但 logs/plans 没有单独记录 `Execution Mode` / `Fallback Scope` / `Fallback Reason Code`。
- 需要受角色边界约束的文件系统写命令没有经过 `run_role_guard.py`，只在结案时补跑范围校验。
- `git add -- <explicit paths...>` 仍被包进 wrapper / helper script，导致运行时看不到裸命令前缀。
- `interaction_impact != none`，但 plan/testplan/ui spec 没有定义 primary flow / fallback flow / visible states / evidence coverage。

## Evidence Capture (UI / E2E)
- Required: partial
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260408-break-prompt-parity-upgrade/evidence/
- What to capture:
  - Screenshots: 默认增强版 break prompt 截图 `break-prompt-parity-preview.png`
  - Screenshots: 自定义壁纸设置卡与预览逻辑通过代码路径审查补证
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: 上传按钮、CTA、select 和 switch 保持键盘可达
- perf budget: 自定义壁纸在前端先压缩再存储，避免直接把原图写入 settings
- error handling / observability: 解码失败通过现有前台 error 状态暴露；开始音播放失败静默回退
- security / privacy: 不新增网络请求或外部权限；自定义壁纸仅保存在本地 settings

## Boundary / Invalid Input Cases
- 选择 `custom` 但没有上传图片时，break 不能出现空背景
- 移除自定义壁纸后，当前 `custom` 选择必须回退到 `paper`
- `silence` 或音量为 `0` 时，开始音必须稳定禁用
- 同一轮 break 不应因组件重渲染重复播放开始音

## Concurrency / Race Cases (if applicable)
- 设置页上传自定义壁纸时，前端先完成图片压缩，再进入现有自动保存节流流程；本轮不改保存模型。

## Mocks & Test Data
- 无额外 mock；直接使用本地开发服务器和真实前台代码路径。

## Commands to Run
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 偏好页和 break prompt 的增强功能都已落盘，spec 不保留占位
  - typecheck / build / cargo check / workflow docs validation 全部通过
  - evidence 目录含默认 break prompt 截图
- Outputs to keep (10~20 lines snippet):
  - `✓ 1825 modules transformed.`
  - `✓ built in 1.41s`
  - `Finished 'dev' profile [unoptimized + debuginfo] target(s) in 1.33s`
