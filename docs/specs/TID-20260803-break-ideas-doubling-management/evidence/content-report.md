# Content report

## Batch identity

- Batch: `2026-08-03-warm-human-expansion`
- Source language: `zh-CN`
- Official locales: `zh-CN / zh-TW / en`
- Microbreak: `364 -> 728`，新增 `aoa..bbz` 共 `364` 条。
- Full break: `244 -> 488`，新增 `ajk..ast` 共 `244` 条。
- 新增概念数：`608`；新增本地化 entry 数：`1,824`。
- Git `HEAD` 对照确认：三份 official bundle 的全部批次前 entry 在 key/order/value 上保持不变。

## Editorial distribution

| Kind | workday-banter | body-banter | health-note | emotional-care | daily-life | mini-mission |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Microbreak | 61 | 61 | 61 | 61 | 60 | 60 |
| Full break | 41 | 41 | 41 | 41 | 40 | 40 |

两个 kind 都按六类循环交错，最大同类连续数为 `1`；类别只存在于 batch manifest，不进入用户 UI 或 runtime selection。

## New-copy length distribution

字符数按 Unicode code point 计数。

| Locale / field | min | p50 | p90 | max |
| --- | ---: | ---: | ---: | ---: |
| zh-CN micro text | 9 | 17 | 21 | 24 |
| zh-CN full title | 4 | 8 | 10 | 12 |
| zh-CN full text | 51 | 61 | 67 | 75 |
| zh-TW micro text | 9 | 17 | 21 | 24 |
| zh-TW full title | 4 | 8 | 10 | 12 |
| zh-TW full text | 51 | 61 | 66 | 75 |
| en micro text | 35 | 60 | 73 | 88 |
| en full title | 13 | 27 | 35 | 44 |
| en full text | 158 | 207.5 | 227 | 258 |

最长完整休息正文为 `anx`；三语内容均落在 manifest 的批次预算内。微休息保持单次扫读长度，完整休息使用标题加约 2–4 句正文。

## Content checks

- 三语 official bundle 的 ordered ID、entry shape 和数量完全一致。
- 每个 official locale/kind 的用户可见字段在 Unicode NFKC、忽略大小写/空白/标点后没有重复标题或正文。
- manifest 精确覆盖新增 608 个 ID，且不保存任何用户可见标题或正文。
- 禁用短语检查覆盖常见模板口吻（如“允许自己 / 你值得 / gentle reminder”）和未经支撑的精确健康话术（如“研究显示 / studies show / 百分比”）；本批命中数为 `0`。
- 简中完成四轮创作与口吻复查；英文按英文口语重写，没有逐字翻译；繁中在 ICU 基础转换后人工修正 `螢幕 / 滑鼠 / 程式碼 / 檔案 / 收件匣 / 分頁 / 介面 / 訊號 / 鬆` 等台湾用词与多义字错误。
- 人工抽样覆盖每类、四分位 ID、最短/最长内容和三语同一 ID 对照；重点修掉了过于工整的结论句、`此刻 / 療癒 / 小小的` 等高频生成式措辞，以及简繁转换产生的 `乾活 / 軀乾 / 松` 等错误。
- 近重复扫描中，简中 micro/full 与英文 micro 的 `SequenceMatcher >= 0.72` 配对数均为 `0`；英文较长正文使用三词组 Jaccard `>= 0.20` 扫描，配对数同样为 `0`。
- 全部 `1,824` 个新增本地化 entry 经过 runtime `splitBreakPromptLines` round-trip，没有内容丢失；实际最大行数为 micro `3`、完整休息标题 `2`、简繁正文 `7`、英文正文 `6`，并受测试中的 `4 / 4 / 10` 行上限保护。

## Health-copy boundary

本批健康知识只采用稳定、低风险的提示：短而频繁离屏、看远、眨眼、起身活动与改变姿势；不写疾病诊断、保证性结果或精确风险数字。审阅依据：

- AOA Computer Vision Syndrome / 20-20-20: <https://www.aoa.org/healthy-eyes/eye-and-vision-conditions/computer-vision-syndrome/>
- HSE Working safely with display screen equipment: <https://www.hse.gov.uk/msd/dse/work-routine.htm>
- HSE DSE work breaks FAQ: <https://www.hse.gov.uk/contact/faqs/vdubreaks.htm>

## Asset size

| Asset | Before | After | Delta |
| --- | ---: | ---: | ---: |
| `messages/zh-CN.json` | 126,983 B | 223,105 B | +96,122 B |
| `messages/zh-TW.json` | 119,263 B | 215,354 B | +96,091 B |
| `messages/en.json` | 118,692 B | 224,762 B | +106,070 B |
| `registry.generated.json` | 3,450,120 B | 3,773,219 B | +323,099 B |

正文池翻倍，但 generated registry 只增加约 `323 KB`，因为本轮只扩充三个 official bundle；legacy bundle 保持不变。

production frontend build 产物 `index-*.js` 为 `4,561.25 kB`（gzip `1,427.56 kB`）；构建通过，并保留仓库既有的 `>500 kB` chunk warning。内容内联带来的体积增长符合本轮静态池扩充预期，未新增网络加载或运行时请求。
