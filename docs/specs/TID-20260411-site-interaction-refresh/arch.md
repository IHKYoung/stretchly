# Task-ID: TID-20260411-site-interaction-refresh

## Goals
- 在不引入依赖的前提下，为 `apps/site` 首页增加更强的视觉中心、节奏控制和轻量交互反馈。
- 保持静态站点结构简单，所有交互都落在现有 `index.html + styles.css + script.js + copy.js` 四件套里。

## Non-Goals
- 不新增打包步骤、状态管理层或第三方动画库。
- 不改下载路由结构，不接入后端或分析脚本。

## Constraints & Assumptions
- 站点必须继续保持纯静态、无依赖。
- 交互反馈需要可在 `prefers-reduced-motion` 下优雅降级。
- 视觉语言应延续纸面/网格纸和极简舞台，不转向花哨的 3D landing page。

## System Boundaries
- Modules:
  - `index.html`：舞台结构、下载按钮、交互层入口
  - `styles.css`：居中布局、粒子/气泡/光晕动画、响应式
  - `script.js`：打字机时序、pointer 反馈、点击/滚动互动逻辑
  - `copy.js`：长文案池 + 点击调侃短句池
- Ownership:
  - 首页所有新交互都由 `apps/site` 自己负责，不外溢到 `download/**`
- Dependency direction:
  - `script.js` 消费 `copy.js` 暴露的只读文案数组

## API / Contract
- Frontend globals:
  - `window.PAUZA_SITE_LINES`: 打字机长文案池
  - `window.PAUZA_SITE_NUDGES`: 点击弹出的短句池
- Error model:
  - 若文案池缺失，脚本需回退到内置默认文案而不是报错

## Data Model / Storage
- 无持久化。
- 所有前端交互状态只存在于运行时 DOM 节点和 JS 内存中。

## Invariants
- typed output 只能由一个打字机循环控制。
- 粒子和气泡节点必须在动画结束后自动移除。
- 下载按钮仍必须可直接点击，不受互动层阻断。

## Concurrency / Lifecycle / Memory Model
- pointer move / wheel / click 事件共用一个交互层，但各自创建的临时 DOM 需要限流并在动画结束后回收。
- 打字机循环不能被点击事件中断或重启。

## Observability Plan (Debug-Driven)
- Logs:
  - 主要依赖浏览器 console 0 error / 0 warning
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - `prefers-reduced-motion`

## Security & Privacy Considerations
- 不新增远程资源请求、不采集用户输入、不写本地存储。

## Risks & Rollback
- Failure modes:
  - 粒子节点未回收导致 DOM 膨胀
  - pointer follow 过强导致阅读干扰
  - 短句气泡位置溢出视口
- Rollback steps:
  - 先回退交互层与粒子逻辑，再评估是否保留 10 秒 hold 和居中布局

## Acceptance Criteria (System)
- 运行时无 console error。
- 页面交互增强不引入新依赖，仍可直接用静态服务器预览。

## Open Questions / Decision Requests
- 无；按当前单页极简方向直接推进。
