# UI

## 窗口与入口
- `apps/desktop/src/App.tsx`：当前唯一有效的桌面端前台入口；默认渲染主设置页，`?window=break` 下渲染独立 break prompt。
- `apps/desktop/src/components/ui/*`：主设置页和 break prompt 共用的基础控件。
- `apps/desktop/src/styles.css`：主界面的设计 token、布局和动效。
- `apps/desktop/src/locales/messages/*.json`：当前唯一有效的 UI 文案真源；break 消息页提示语统一维护在 `ui.breakCopy.*`。
- `apps/site/index.html`：当前官网单页入口；负责用一块极简打字机舞台展示 Pauza 的提醒文案，并保留单一下载按钮。
- `apps/site/download/*/index.html`：平台/用途拆分的稳定下载路由页；当前真实下载地址统一由 `apps/site/download/targets.js` 提供。

## 官网站点
- 官网当前采用纯静态 `HTML + CSS + JS` 实现，目标是先承接品牌表达和下载入口，而不是演化成多页内容站。
- 首页当前进一步收敛为单舞台结构：纯白纸面背景上只保留一块中央文案展示区和右上角下载按钮。
- 视觉语言改为更纯净的“草稿纸 / 网格纸”方向：白底、淡网格、极弱噪点和细边框，不再使用多卡片信息布局。
- 官网当前整页已统一到单一字体：`LXGW WenKai Screen` 同时用于 `Pauza>`、主文案、下载按钮、互动提示和粒子字元，不再混用第二套 UI 字体。
- 首页舞台当前已进一步去卡片化：中央输出区不再带边框、底色、阴影或高光层，而是直接贴在纸面背景上。
- `Pauza>` 现作为单独的终端提示头放在舞台区域左上角，主文案继续保持中央居中，形成“prompt + output”的终端层级。
- `Pauza>` 终端提示头当前已进一步加重：字号、字重和右侧分隔线都更明显，不再像弱提示标签。
- 主视觉文案采用打字机式逐字输入、停留、退格、切换；当前单条文案完整显示后会停留约 `10s`，展示内容精选自 App 已内嵌的提醒文案，并使用 `LXGW WenKai Screen` 字体资源实现更有书写感的中文气质。
- 首页主舞台现已明确居中，桌面端宽度约占视口 `80%`；主输出文案本身也按浏览器宽度 `80%` 收敛。舞台本身可聚焦，键盘 `Enter / Space` 也能触发互动提醒。
- 首页新增一层克制的原生交互：鼠标移动会带来轻微 aura / trail，滚动和点击时的粒子已从光点改成更偏 `0 / 1 / # / @ / ！ / ¥ / $` 的符号字元，且数量与字号都比上一版更高；点击空白或舞台会弹出一条调侃式“别久坐”短句；`prefers-reduced-motion` 下会自动弱化高频动画。
- 下载按钮不直接硬编码外部文件地址，而是统一先进入站内稳定路由；这样后续无论接 GitHub Releases、R2 还是其他对象存储，都不需要改首页 CTA。

## 前端实现模式
- 当前桌面端前台采用 `React + TypeScript + Tauri command` 模式，不再以旧 Electron 的 `html + preload + renderer` 作为默认实现。
- 旧 Electron 壳已从当前工作树移除；任何历史 UI 行为都应通过 git 历史或既有 docs/specs 追溯，而不是恢复 `app/**` 目录。
- 前台只消费 `DesktopSnapshot`，不自行维护另一份运行时真源。
- 浏览器 preview 和原生 runtime 共用同一套 React UI，避免“网页可看、原生才会用”的双实现。

## 主要用户路径
1. 用户进入主设置页，按 `节奏 / 偏好` 两个主分类调整提醒、显示方式、智能暂停、声音、语言与启动项。
2. 常规运行时主要通过 tray、快捷键和设置页管理 pause / focus / reset。
3. 若启用了“提前提示”，break 在 due 前的 lead time 内会先通过设置页状态和 tray 文本显示 `即将开始 / Up next`；系统通知只作为辅助，不再是唯一有效表面。
4. break 到点后，`?window=break` 对应的前台会展示独立 break prompt；`智能提醒` 下会先找更明显的空档，再逐步放宽阈值；`强制提醒` 下到点直接进入 break。
5. break prompt 在倒计时进行中只提供 `Later / Skip` 这类运行时允许的动作；不会再允许提前完成。只有 manual finish 开启且计时结束后进入 `manualAwaiting` 状态时，主按钮才显示为 `Resume work`。

## Tauri 设置页结构
- 主设置页现为固定双栏工具窗口：左侧导航区约 `168px`，右侧为单一主面板。
- 左侧导航当前只保留 2 个分类：`节奏`、`偏好`。
- 当运行时处于 pause / focus 时，主窗口内容区顶部会出现一张 `当前状态` 卡：pause 态只显示 `恢复提醒`，focus 态只显示 `结束专注`；普通运行态不显示恢复动作。
- `节奏`：顶部先放独立的 `节奏控制` 区，只保留 `重置节奏`，并明确“只重算计时，不自动解除暂停”；下方再是 microbreak、break 的开关与 preset 芯片，以及 pre-break heads-up cue 与 postpone。
- `偏好`：`显示方式：窗口 / 全屏`、`提醒方式：智能提醒 / 强制提醒`、natural breaks、DND、app exclusions、开机自启动、语言。
- `偏好 -> 语言` 当前采用“草稿值只更新下拉、持久化成功后才切换整页文案与 `dir/lang`”的边界，避免在 autosave 往返期间因为整页重翻译而卡住设置窗口。
- 设置页分钟/秒数字 stepper 当前也采用“本地草稿 -> blur/Enter/加减按钮提交”的交互边界，允许临时清空、多位数输入，并用 `Escape` 恢复当前值，避免每击键触发 autosave 回弹。
- `偏好` 中的 break surface 现已重新接回更强的体验配置：背景主题、自定义壁纸上传与完整预览、交互语开关、当前时间显示，以及微休息 / 休息的开始音、结束音和共享音量。
- break style 与独立 strict mode 已从设置页移除，不再暴露给用户。
- overview、常驻 quick actions、快捷键编辑与额外第三列 save rail 已从设置页主结构移除；保留在设置窗口中的运行时入口只剩与当前状态直接相关的上下文动作和独立的节奏重置。
- 页面视觉已继续收紧为偏桌面工具的平面风格：暖灰背景、纯白主面板、浅边框、小圆角、几乎无装饰背景，并移除侧栏状态区与重复标题。
- 基础控件（button / select / segmented control / number input）统一压平为更小的圆角和更紧的高度，不再强调悬浮卡片感。
- 设置页现已改为自动保存，顶部只在保存中或刚发生变更时显示轻量状态，不再保留显式保存按钮。
- `节奏` 分类已进一步压成两张紧凑 preset 卡：微休息与休息顶部直接用芯片选择间隔/频率与时长，下方“提前提醒 / 延后”继续保留 stepper 作为低频细调。
- 当前设置页不再直接暴露 `idle_opportunity_seconds` 阈值；提醒策略继续采用 host 内部的递减阈值曲线，避免用户为提醒方式承担额外参数心智。
- “提前提示”现在表达的是 due 前的可见 cue，而不是承诺一定弹出系统通知；即使系统层没有显示通知，设置页状态和 tray 文本也会进入 heads-up 阶段。
- Tauri 主窗口默认尺寸调整为 `960x640`，最小尺寸为 `800x600`，避免设置页继续被压到不可用尺寸。

## Tauri Break Prompt
- break 页面现在直接填满宿主窗口本身，不再在 break window 里再套一张固定宽度的小卡片。
- window 模式下，break 宿主窗口现统一居中，窗口比例按 16:9 收敛，并优先以内聚的大窗呈现；不再为 microbreak 保留右下角小浮窗分支。
- break prompt 现进一步收敛为单列纯净布局：中央只保留一条交互语、数字倒计时和细条形进度，不再保留双栏、cue card、break kind 标签或环形倒计时；玻璃卡片也改得更轻、更通透。
- break prompt 的中央文案现已从“普通段落自动换行”改为“整句优先、超长句按分句换行”；中文不再从任意字位折断，长句会在逗号/句号等自然停顿处独立成行。
- break 背景不再只有单层浅色底，现支持 `paper / dawn / forest / night` 预设主题，以及用户上传的自定义壁纸；自定义图在前端压缩后存入本地设置，后续 break 可直接复用。
- 顶部仅在 `currentTimeInBreaks` 打开时显示当前时间；底部 CTA 会按当前阶段收紧：倒计时进行中不再显示提前完成，`Later` 也只在倒计时开始后的前 10 秒内出现。
- fullscreen break 的宿主层现在按平台处理：macOS 使用 simple fullscreen，避免 native fullscreen/titlebar 语义造成顶部空白；其他平台继续沿用标准 fullscreen。
- macOS 下即使 break 走 window 模式或其它 non-focusable 路径，宿主层也会在显示后显式激活应用，尽量把 break prompt 带到当前全屏 Space。
- 主 CTA 只在 `manualAwaiting` 态显示 `Resume work`；普通倒计时阶段不再允许提前完成。
- 次操作按钮按能力显示：允许 postpone 才显示 `Later`，允许 skip 才显示 `Skip`；当前 postpone 窗口固定收紧为倒计时开始后的前 10 秒。
- `currentTimeInBreaks` 打开时，在右上角显示当前时间。
- `breakIdeasEnabled` 打开时，break prompt 会按语言和 break kind 从 `apps/desktop/src/locales/messages/*.json` 顶层的 `miniBreakIdeas` / `longBreakIdeas` 中选取 `.text` 交互语；关闭后回退到 `ui.breakCopy.defaultPrompt.*`。`ui.breakCopy.prompts` 这类过渡字段已删除，不再允许重新引入。
- `microbreakStartSound` / `longBreakStartSound` / `microbreakEndSound` / `longBreakEndSound` 与 `breakSoundVolume` 控制休息开始/结束时的一次性提示音；`silence` 或音量为 `0` 时不播放。
- break prompt 的触发时机已从“到点立即出现”调整为由 reminder mode 决定：`智能提醒` 下若仍在连续输入，则先等待更明显的空档，再逐步放宽到更短空档，并在最终 deadline 到达时直接开始；`强制提醒` 下则到点直接出现，并且 break 期间不可跳过、不可延后、不可通过关窗绕过。

## UI 个性化开发建议
- 调整设置页与 break prompt 时，优先改 `apps/desktop/src/App.tsx`、`apps/desktop/src/styles.css`、`apps/desktop/src/components/ui/*`。
- 修改 break 文案时，统一改 `apps/desktop/src/locales/messages/*.json`；不要再新增 `break-message-copy.*` 这种并行入口。
- 修改托盘、快捷键、调度和窗口行为时，优先改 `apps/desktop/src-tauri/src/{state,engine,platform,shell,commands}.rs`。
- 如果需要追溯历史行为，请直接查看 git 历史、`docs/历史版本整理.md` 与既有 task specs，不要重新把当前实现接回旧 Electron 链路。
