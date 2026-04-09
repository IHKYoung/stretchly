# Task-ID: TID-20260409-break-surface-i18n-reduction

## Test Strategy
- Unit:
  - 前端 `typecheck` 覆盖 settings schema / i18n registry / App.tsx 类型一致性。
  - Rust `cargo check` 覆盖 `PauzaSettings`、`normalize_language`、window profile 相关编译约束。
- Integration:
  - `npm --prefix apps/desktop run build` 覆盖 Vite 打包、locale 引用与 break audio 资源。
  - `python3 scripts/validate_workflow_docs.py --mode manual` 覆盖文档门禁。
- E2E (if applicable):
  - 浏览器或桌面预览截图验证 break glass 卡片和壁纸预览状态。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> break window 截图 + 前端 build
- AC2 -> `cargo check` + `shell.rs` 代码路径审查
- AC3 -> 设置页截图 + 前端 build
- AC4 -> `typecheck` / build + 实机/preview 音效逻辑代码审查
- AC5 -> 设置页截图 + locale diff 审查
- AC6 -> `typecheck` + `cargo check` + registry 文件审查
- AC7 -> `rg`/入口审查输出 + 文档结论

## Interaction Contract Coverage
- Interaction impact: direct

### Primary flow -> tests/evidence
break 主界面截图、设置页音效/壁纸/语言区域截图，以及 `typecheck` / `build` / `cargo check`。

### Fallback / secondary flow -> tests/evidence
browser preview 正常渲染；未上传壁纸时仍显示空状态说明，不出现拉伸占位图。

### Visible states / transitions -> tests/evidence
break 运行中、倒计时归零后的结束音配置生效、中文设置文案纯净、自定义壁纸预览改为完整显示图片。

### Validator expectation
当 `interaction_impact != none` 时，本节三项与 Evidence Capture 的 `Required` 不得继续保留占位内容。

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
- Required: yes
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260409-break-surface-i18n-reduction/evidence/
- What to capture:
  - Screenshots:
    - break prompt glass 版本
    - 设置页自定义壁纸完整预览
    - 设置页音效/语言区域
  - Video/trace (optional): N/A
  - HAR/console logs (optional): 如构图验证需要，可补控制台输出

## Quality Gates (Non-functional)
- a11y: 关键控件继续可聚焦、可键盘操作
- perf budget: 不新增外部依赖；locale 注册不能显著拉高前端 bundle 复杂度
- error handling / observability: locale 缺失时回退默认语言，不露 key
- security / privacy: 自定义壁纸仍只保存在本地 settings

## Boundary / Invalid Input Cases
- 未上传壁纸却切到自定义壁纸
- 结束音为静音或音量为 0
- 语言配置存在但消息文件缺失 / 消息文件存在但配置缺失

## Concurrency / Race Cases (if applicable)
- React 重渲染导致同一轮 break 开始音/结束音重复播放
- break 在手动完成等待态与 close path 间快速切换

## Mocks & Test Data
- 使用现有 browser preview snapshot、自定义壁纸 data URL 和内置音频文件

## Commands to Run
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 所有命令通过
  - 截图与代码审查证明 AC1-AC7 达成
- Outputs to keep (10~20 lines snippet):
  - build / cargo check / docs validator 的 PASS 摘要
