# Pauza V1 六步产品演进方案

## 文档目的
- 把此前按“六周”讨论的产品方向，重写成按门槛推进的“六个连续步骤”。
- 用 AI 压缩文案、原型、实现和测试时间，但不跳过产品判断、默认值选择和取舍。
- 让 Pauza 从“另一个 Stretchly”逐步演进成“为 Mac 脑力工作者设计的、低打扰、高质感的恢复节奏工具”。

## 北极星
- 服务对象：长期坐在 Mac 前写代码、写文档、做设计、剪视频或开会的人。
- 核心承诺：在不粗暴打断心流的前提下，帮用户建立更稳的恢复节奏。
- 区分点：
  - 比 Stretchly 更像原生 Mac 工具
  - 比传统定时提醒器更少打扰
  - 比单纯的 break popup 更有节奏反馈和结果感

## 当前问题摘要
- 对外叙事仍停留在“久坐提醒/休息提醒”，用户心智还没脱离 Stretchly。
- 桌面端主入口仍是设置页，第一次使用更像在调参数而不是开始使用产品。
- “智能提醒”目前主要依赖 idle、DND 和 app exclusion，距离真正的上下文理解还不够远。
- 设置项仍然偏多，但对用户“今天恢复得如何”缺少结果反馈。
- 官网已经有气质，但还没有清楚说明“为什么我应该从 Stretchly 换到 Pauza”。

## 推进原则
- 先改产品心智，再加新功能。
- 先改主路径，再收高级设置。
- 先做低打扰能力，再谈商业化。
- 每一步都要能映射到当前仓库的真实模块，而不是停留在概念层。

## Step 1：统一定位和语言层

### 目标
- 把 Pauza 的对外表达从“久坐提醒/休息提醒”改成“低打扰恢复节奏工具”。

### 为什么先做
- 如果定位不先改，后面的 onboarding、首页、主窗口和付费边界都会继续沿用旧心智。
- 用户第一眼怎么理解 Pauza，决定了它是“一个定时器分叉”，还是“一个有判断力的 Mac 工具”。

### 具体修改内容
- 官网文案层：
  - 改写 `apps/site/index.html` 的 `<title>`、`meta description`、交互提示和下载入口文案。
  - 调整 `apps/site/copy.js` 中的首页打字机句子，让它不只是在“提醒喝水/伸展”，而是在表达“更少打扰、更懂节奏、更适合 Mac”。
  - 视情况为下载页补一小段价值说明，而不是只有“下载”动作。
- 桌面端文案层：
  - 全面审查 `apps/desktop/src/locales/messages/*.json`，把“休息提醒应用”“久坐提醒”这类词换成新的产品语言。
  - 清理仍带 Stretchly 影子的命名，例如 about、welcome、tray、tooltip、preview action 等位置的遗留表达。
  - 统一核心词汇：`恢复节奏`、`低打扰`、`保护心流`、`下一次恢复`、`今天节奏`。
- 文档层：
  - 更新 `README.md`、`docs/UI.md`、`docs/Architecture.md` 中对产品的描述，避免代码和文档继续输出两套心智。

### 涉及模块
- `apps/site/index.html`
- `apps/site/copy.js`
- `apps/desktop/src/locales/messages/*.json`
- `README.md`
- `docs/UI.md`
- `docs/Architecture.md`

### 完成标志
- 用户第一次看到官网或桌面端，不会自然把它归类成“另一个休息提醒器”。
- 仓库里最核心的外显语言已经从“提醒”切到“恢复节奏”。

### 暂不做
- 不在这一步加新功能。
- 不急着改付费文案和价格页。

## Step 2：把第一次使用体验改成 onboarding，而不是设置页

### 目标
- 新用户第一次打开 Pauza 时，不是先掉进参数面板，而是先完成一个 30 到 60 秒的上手流程。

### 为什么先做
- 当前默认入口就是设置页，这会让产品在第一分钟显得像一个复杂工具，而不是一款有默认判断的产品。
- 如果第一步体验不改，后续再做主窗口改版也会被旧用户心智拖回“去设置里找功能”。

### 具体修改内容
- 状态和持久化：
  - 在 `apps/desktop/src-tauri/src/state.rs` 增加首次使用相关字段，例如 `has_completed_onboarding`、`work_style_profile`、`interruptibility_profile`、`rhythm_preset`。
  - 让 host 在首次启动时返回明确的 onboarding 状态，而不是只能渲染设置页。
- 前台路由：
  - 在 `apps/desktop/src/App.tsx` 增加 onboarding 模式，不再只有 `settings` 和 `?window=break` 两个入口。
  - 让首次使用路径直接进入 onboarding，完成后再进入主窗口。
- onboarding 内容：
  - 第一步：你主要做什么工作。
    - 选项可以是开发 / 写作 / 设计 / 通用。
  - 第二步：你想要 Pauza 多克制。
    - 例如：轻提醒 / 平衡 / 更严格。
  - 第三步：给你一个默认节奏。
    - 例如：轻量恢复 / 标准节奏 / 深度工作日。
  - 第四步：预览一次 microbreak 或 long break 的实际体验。
  - 第五步：确认是否开机启动。
- 默认值生成：
  - `settings-controls.ts` 里的 preset 不再只作为裸参数，而是能被 onboarding 组合成“工作风格 preset”。

### 涉及模块
- `apps/desktop/src-tauri/src/state.rs`
- `apps/desktop/src-tauri/src/commands.rs`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/lib/settings-controls.ts`
- `apps/desktop/src/locales/messages/*.json`

### 完成标志
- 新用户可以在 1 分钟内开始用 Pauza，而不是先理解 microbreak、long break、postpone、notification 等术语。
- 第一次启动结束后，产品已经替用户给出一套合理默认值。

### 暂不做
- 不在这一步塞进复杂权限申请。
- 不做多页教程或冗长欢迎页。

## Step 3：把主窗口从设置中心改成“今天的节奏页”

### 目标
- 把 Pauza 主窗口改成一个能回答“我现在处于什么状态、下一次恢复是什么、今天节奏怎么样”的产品首页。

### 为什么先做
- 当前主窗口默认还是参数面板，用户很难感受到产品在“陪伴今天的工作节奏”。
- 只有主窗口变成产品首页，后面的反馈闭环和低打扰解释才有承载面。

### 具体修改内容
- 页面结构：
  - 主窗口默认进入 `Today` / `Home` 页面。
  - `节奏`、`偏好` 退为次级页或侧边入口。
- 首页信息结构：
  - 当前状态：运行中 / 暂停 / 保护心流 / 正在休息。
  - 下一次恢复：时间、类型、预计剩余。
  - 为什么当前没有打断你：例如会议中、全屏中、DND 中、自然空档中。
  - 快捷动作：开始专注、暂停 30 分钟、恢复节奏、立即休息预览。
  - 今日摘要占位：为 Step 5 的轻量反馈预留位置。
- 设置页收缩：
  - 常用项保留在首页或简化设置页。
  - 参数密度高的内容转入 “高级设置”。
  - `app exclusion` 不再作为高频首屏元素暴露。
- 命令面：
  - `DesktopSnapshot` 需要对首页提供更直观的 runtime 字段，而不是只服务设置控件。

### 涉及模块
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/styles.css`
- `apps/desktop/src-tauri/src/state.rs`
- `apps/desktop/src-tauri/src/commands.rs`
- `apps/desktop/src/locales/messages/*.json`

### 完成标志
- 主窗口看起来像产品首页，而不是系统偏好面板。
- 用户不需要先点进设置，也能理解 Pauza 当前在做什么。

### 暂不做
- 不在这一步引入复杂图表。
- 不做“全功能 dashboard”。

## Step 4：把“低打扰”从口号做成真正能力

### 目标
- 让 Pauza 真正理解“什么时候不该打断你”，并把原因解释出来。

### 为什么先做
- 这是 Pauza 相对 Stretchly 最有机会建立护城河的部分。
- 如果仍然主要靠固定时间 + idle 判断，产品最多只是“更好看一点的 break reminder”。

### 具体修改内容
- 运行时能力扩展：
  - 在 `platform.rs` 增强场景信号，不再只停留在 `idle / DND / app exclusion`。
  - 优先做：
    - fullscreen / presentation 场景
    - 会议场景（Zoom / Meet / Teams 等）
    - 录屏 / 演示场景（OBS、屏幕录制等）
    - 已手动开启的 focus session
- 产品层收敛：
  - 把当前的 `app exclusion` 文本框思路，逐步改成“内置场景保护包 + 可选高级自定义”。
  - 用户常用的是“会议中别打断我”，不是手动输入 `zoom.us`。
- 解释层：
  - `state.rs` 和 `DesktopSnapshot` 要增加更明确的 `protection_reason` 或同类字段。
  - tray 和主窗口首页都要显示“这次为什么还没提醒你”。
- 设置层：
  - 常用开关应变成：
    - 会议时避让
    - 全屏时避让
    - 录屏/演示时避让
    - DND 时避让
  - 把长文本 exclusion 输入区降为高级选项。

### 涉及模块
- `apps/desktop/src-tauri/src/platform.rs`
- `apps/desktop/src-tauri/src/state.rs`
- `apps/desktop/src-tauri/src/engine.rs`
- `apps/desktop/src-tauri/src/shell.rs`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/locales/messages/*.json`

### 完成标志
- 用户能明显感受到 Pauza 在“懂场景”，而不是只会定时弹窗。
- 主窗口和 tray 能解释为什么当前被保护而没有打断。

### 暂不做
- 不一开始做完所有第三方应用的深度集成。
- 不先做企业级策略和管理员配置。

## Step 5：增加最轻量的恢复反馈闭环

### 目标
- 让用户知道 Pauza 不只是“提醒了几次”，而是真的帮助自己保持了更稳的节奏。

### 为什么先做
- 没有反馈，产品只能停留在“被动提醒器”。
- 但如果一开始就上重报表和大盘，又会把产品做脏、做重。

### 具体修改内容
- 本地数据模型：
  - 在 `state.rs` 中增加按天归档的轻量统计，不需要云同步。
  - 第一版只做：
    - 完成的 microbreak 次数
    - 完成的 long break 次数
    - postpone / skip 次数
    - 因场景保护而推迟的次数
    - 最长连续工作时段
- 首页展示：
  - 在 Step 3 的今日页中加入一张轻量摘要卡。
  - 只显示 3 到 5 个最有意义的数字，不做复杂图表。
- 文案总结：
  - 根据今天的节奏生成一句简洁总结，例如：
    - “今天已经完成 4 次恢复，节奏比昨天更稳。”
    - “你今天一共被保护性延后 3 次，Pauza 没有粗暴打断你。”
- 边界控制：
  - 不做云端账号。
  - 不做行为画像。
  - 不做侵入式时间追踪。

### 涉及模块
- `apps/desktop/src-tauri/src/state.rs`
- `apps/desktop/src-tauri/src/commands.rs`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/locales/messages/*.json`

### 完成标志
- 用户在主窗口能看到“今天节奏”的结果，而不只是设置项和下次倒计时。
- 反馈层足够轻，不会把产品变成报表工具。

### 暂不做
- 不做周报/月报。
- 不做云同步统计。
- 不做过度量化的 productivity score。

## Step 6：最后才定义付费边界和商业实验

### 目标
- 在前五步把产品心智和护城河做出来后，再划清免费版和 Pro 的边界。

### 为什么最后做
- 如果一开始就定价，容易把不值钱的表层功能拿去收费。
- 用户真正愿意付钱的，通常不是“更多提醒”，而是“更少打扰但更有效”。

### 具体修改内容
- 免费版边界：
  - 基础节奏
  - 基础 microbreak / long break
  - 基础 smart / forced
  - 基础托盘和 break prompt
- Pro 候选边界：
  - 更强的场景保护能力
  - 更完整的低打扰规则
  - 更好的今日节奏反馈
  - 未来的多设备同步或团队默认策略
- 不建议作为主要付费点的内容：
  - 更多主题
  - 更多文案
  - 更多粒子和视觉装饰
- 商业实验面：
  - 官网补一版“为什么值得从 Stretchly 切换”的价值说明
  - 定义早鸟 Pro 的最小功能边界
  - 再决定是否进入直接买断、年费或 Setapp

### 涉及模块
- `apps/site/index.html`
- `apps/site/copy.js`
- `apps/site/download/*`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/locales/messages/*.json`
- 后续可能新增的 license / entitlement / purchase docs

### 完成标志
- 你可以清楚回答“为什么用户愿意从 Stretchly 切到 Pauza 并付钱”。
- 付费能力卖的是低打扰和结果感，而不是表层装饰。

### 暂不做
- 不立刻做团队版。
- 不先做 AI 教练。
- 不为收费而收费。

## 推荐执行顺序
- Step 1 先改心智，不然后面所有改动都会继续说旧话。
- Step 2 和 Step 3 连着做，把“第一次使用”和“日常主窗口”都从参数页拉出来。
- Step 4 是产品护城河，应该在主路径稳定后立即跟上。
- Step 5 负责让用户感受到产品价值，而不是只看到弹窗。
- Step 6 只有在前五步站住后才有意义。

## 不要提前做的事情
- 不要继续优先做主题、粒子、更多文案和背景。
- 不要为了“功能完整”继续补齐 Stretchly 式参数。
- 不要在用户留存和替换理由还没成立时，急着做复杂订阅系统。
- 不要用 AI 直接代替产品判断；AI 负责加速，不负责决定默认值和边界。

## 后续任务拆解建议
- 下一批最合理的执行顺序：
  - 任务 A：定位与官网/产品文案重写
  - 任务 B：首次启动 onboarding
  - 任务 C：主窗口改成今日节奏页
  - 任务 D：会议/全屏/录屏低打扰能力第一批
  - 任务 E：今日恢复反馈
  - 任务 F：免费 / Pro 边界与官网实验
