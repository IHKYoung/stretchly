import { expect } from 'vitest'

import { splitBreakPromptLines } from '../apps/desktop/src/lib/break-copy-layout'

describe('Desktop break copy layout', () => {
  it('splits long Chinese prompt copy into punctuation-safe lines', () => {
    expect(
      splitBreakPromptLines(
        '双手搓热，轻轻捂住双眼，感受温暖和黑暗。这叫掌心热敷法，对缓解视疲劳很有效。',
        'zh-CN',
        'hero',
      ),
    ).toEqual([
      '双手搓热，',
      '轻轻捂住双眼，',
      '感受温暖和黑暗。',
      '这叫掌心热敷法，',
      '对缓解视疲劳很有效。',
    ])
  })

  it('keeps short English sentences one sentence per line', () => {
    expect(
      splitBreakPromptLines(
        'Look away from the screen. Let your eyes soften for a moment.',
        'en',
        'detail',
      ),
    ).toEqual(['Look away from the screen.', 'Let your eyes soften for a moment.'])
  })

  it('packs long English clauses without dropping punctuation', () => {
    expect(
      splitBreakPromptLines(
        'Stand up, roll your shoulders back, and breathe slowly before sitting down again.',
        'en',
        'hero',
      ),
    ).toEqual(['Stand up, roll your shoulders back,', 'and breathe slowly before sitting down again.'])
  })
})
