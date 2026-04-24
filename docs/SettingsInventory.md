# Settings Inventory

这份清单用于回答两个问题：

1. 现在设置页里已经有哪些设置。
2. 还有哪些设置可以加回设置页，或者值得在后续版本里重新实现。

源码依据：

- `apps/desktop/src/App.tsx`
- `apps/desktop/src-tauri/src/state.rs`
- `apps/desktop/src-tauri/src/commands.rs`
- `app/utils/defaultSettings.js`（仅作原版参考）

## 1. 当前设置页已经提供的设置

当前前端设置页已经把 break 体验相关的核心设置重新暴露出来，不再只剩基础节奏项。

### 节奏

- 节奏控制：`重置节奏`（运行时动作，非持久化设置；只重算下一次微休息/休息，不自动解除暂停）
- 微休息开关
- 微休息间隔分钟数
- 微休息时长秒数
- 休息开关
- 休息每几次微休息触发
- 休息时长分钟数

### 设置窗口中的上下文运行时控制（非持久化设置）

- 暂停态顶部出现 `恢复提醒`
- 专注态顶部出现 `结束专注`
- 普通运行态不显示恢复类动作

### 提醒与打断

- 微休息提前提示开关
- 微休息提前提示秒数
- 休息提前提示开关
- 休息提前提示秒数
- 微休息允许延后
- 微休息每次延后分钟数
- 休息允许延后
- 休息每次延后分钟数
- 提醒方式：`smart / forced`
- 休息显示方式：窗口 / 全屏
- 背景主题：`paper / dawn / forest / night / custom`
- 自定义壁纸上传与移除
- 自定义壁纸完整预览
- 交互语开关（控制 break prompt 是否轮播 locale 中维护的交互语）
- 休息窗口显示当前时间
- 微休息开始音
- 微休息结束音
- 休息开始音
- 休息结束音
- 提示音音量

### 智能暂停

- 自然休息开关
- 自然休息后多少分钟重置
- 监控 DND / 专注模式
- 应用排除开关
- 应用排除规则：`pause / resume`
- 应用排除命令列表

### 通用

- 开机自启动
- 在 tray 显示下次休息时间
- 语言

补充说明：

- 当前“提前提示”不再只代表系统通知。
- 对桌面端现实现状，更准确的语义是：在 break 到点前的 lead time 内，Pauza 会进入一个可见的 heads-up 阶段，并同步到设置页运行时状态与 tray 文本；若系统通知可用，也可以额外投递一次辅助通知。

## 2. 当前 Tauri 已经支持，但前端还没放出来的设置

下面这些项已经有 Tauri 真源，或者已经有快捷键/运行时绑定。它们属于“可以直接加回设置页”的范围，不需要先重新设计后端数据结构。

### 延后与完成策略

- 微休息延后次数上限：`microbreak_postpones_limit`
- 休息延后次数上限：`long_break_postpones_limit`
- 微休息手动结束：`microbreak_manual_finish`
- 休息手动结束：`long_break_manual_finish`
- 当前运行时补充规则：`延后当前休息` 只在 break 倒计时开始后的前 `10s` 可用；`完成当前休息` 不再允许提前触发，只保留给 `manualAwaiting` 阶段。

### 多屏与显示细节

- 所有屏幕都显示：`show_breaks_on_all_screens`
- 目标屏幕：`target_screen`
- 强制提醒下仍允许 tray menu：`show_tray_menu_in_strict_mode`

### 快捷键

- 打开设置：`reveal_settings_shortcut`
- 专注 45 分钟：`focus_45_shortcut`
- 切换暂停：`pause_toggle_shortcut`
- 暂停 30 分钟：`pause_30_shortcut`
- 暂停 60 分钟：`pause_60_shortcut`
- 暂停 120 分钟：`pause_120_shortcut`
- 暂停 300 分钟：`pause_300_shortcut`
- 跳到下一个计划休息：`skip_next_scheduled_shortcut`
- 跳到下一个微休息：`skip_next_microbreak_shortcut`
- 跳到下一个休息：`skip_next_long_break_shortcut`
- 重置节奏：`reset_breaks_shortcut`

### 说明

- 这些项已经存在于 `PauzaSettings`，或者已经被 `shortcut_bindings()` 读取。
- 也就是说，如果你决定把它们重新加回设置页，主要是前端信息架构和交互表达的问题，不是后端能力缺失的问题。
- 当前 `breakBackdrop`、`breakCustomBackdrop*`、`breakIdeasEnabled`、`microbreakStartSound`、`microbreakEndSound`、`longBreakStartSound`、`longBreakEndSound`、`breakSoundVolume` 与 `current_time_in_breaks` 都已经是前台可见设置。
- `idle_opportunity_seconds` 现在只保留为 host 载入旧配置时的兼容字段：
  - 前台设置模型不再显式携带它
  - 新写回的 settings/snapshot 也不再继续序列化它
  - 当前 smart reminder 实际采用的是内置固定策略，而不是可调单阈值字段
- 当前 host 实际采用的是 reminder 最小模型：
  - 微休息：连续空闲 `8s`，最长等待 `90s`
  - 休息：连续空闲 `12s`，最长等待 `180s`
  - pause/focus/DND/app exclusion 只冻结投递，解除后平移 due / waiting timer，不再整轮 reset
  - natural breaks 只负责长时间离开后的 full reset
  - 这些策略值目前都不单独暴露为设置项。

## 3. 历史设置基线里有过，但当前 Tauri 还没接回来的候选设置

下面这些项在原版参考文件 `app/utils/defaultSettings.js` 里仍能看到，但当前 Tauri `PauzaSettings` 没有对应真源。它们不是“直接加 UI”就能生效的项，而是“要先恢复或重写后端能力”的候选项。

### 外观与主题

- 主题来源：`themeSource`
- 主色 / 微休息色：`mainColor` / `miniBreakColor`
- 透明模式：`transparentMode`
- 模糊背景：`blurredBackground`
- 窗口透明度：`opacity`

### 声音与通知体验

- 静音通知：`silentNotifications`

### 休息内容与想法

- 是否使用设置里的 ideas：`useIdeasFromSettings`
- 休息 ideas：`breakIdeas`
- 微休息 ideas：`microbreakIdeas`

### 更新与系统行为

- 新版本提醒：`notifyNewVersion`
- 检查更新：`checkNewVersion`
- 禁用应用更新功能：`disableAppUpdateFeatures`
- 锁屏 / 睡眠时自动暂停：`pauseForSuspendOrLock`
- 早晨时间：`morningHour`
- 暂停到早晨快捷键：`pauseBreaksUntilMorningShortcut`

### 托盘与图标

- Tray 图标风格：`trayIconStyle`
- 是否显示 tray 图标：`showTrayIcon`
- 单色 tray 图标：`useMonochromeTrayIcon`
- 反相单色 tray 图标：`useMonochromeInvertedTrayIcon`

### 休息窗口尺寸与模式

- 休息窗口宽度比例：`breakWindowWidth`
- 休息窗口高度比例：`breakWindowHeight`
- 以普通窗口显示休息：`showBreaksAsRegularWindows`

### 其他候选

- 自定义 preferences message：`customPreferencesMessage`
- 健康模式：`breakHealthMode`
- 地理位置：`posLatitude` / `posLongitude`

## 4. 不建议放进“设置”的运行时动作

这些在当前 Tauri 里是命令，不是设置。它们原则上更适合放在 tray、快捷操作、快捷键或 break window CTA，而不是主设置页。

当前设置窗口只保留了最小化的例外：

- `恢复提醒`：只在暂停态作为上下文动作出现
- `清除专注 session`：只在 focus 态作为上下文动作出现
- `重置节奏`：独立放在 `节奏控制` 区，明确它不是“恢复提醒”的别名

其余运行时动作仍不建议重新塞回主设置页。

- 暂停提醒
- 恢复提醒
- 开启专注 session
- 清除专注 session
- 完成当前休息
- 跳过当前休息
- 延后当前休息
- 跳到下一个计划休息
- 跳到下一个微休息
- 跳到下一个休息
- 重置节奏

## 5. 如果要继续做设置页，建议优先级

如果你想继续重做设置页，我建议优先把这些“已支持但没露出”的项加回来：

### 第一优先级

- 微休息 / 休息延后次数上限
- 微休息 / 休息手动结束
- 所有屏幕都显示
- 目标屏幕

### 第二优先级

- 休息窗口显示当前时间
- 强制提醒下仍允许 tray menu
- 快捷键设置

### 第三优先级

- 主题 / 透明度 / 声音 / ideas / tray 图标 / 更新检查

第三优先级不是不重要，而是它们现在没有 Tauri 真源，先做会把任务从“重做前端设置页”升级成“恢复一批后端能力”。
