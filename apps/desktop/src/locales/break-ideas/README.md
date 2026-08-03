# Break ideas content guide

`messages/*.json` is the only source of user-visible break-idea copy. The generated registry is a build artifact, and `batches/*.json` contains governance metadata only: IDs, editorial categories, count targets, and quality limits.

## Content boundaries

- `miniBreakIdeas[id]` contains exactly `{ "text": "..." }`.
- `longBreakIdeas[id]` contains exactly `{ "title": "...", "text": "..." }`.
- Official locales (`zh-CN`, `zh-TW`, and `en`) must have identical ordered ID sets and entry shapes.
- Legacy locale bundles remain valid fallbacks and are not required to match the official pool.
- IDs are three lowercase base-26 letters. Add a batch after the current last ID; never reuse or reorder an existing ID.

## Batch manifest

Each `batches/<batch-id>.json` records one append-only editorial batch. It must declare:

- manifest schema version (`1` at present);
- source and official languages;
- before / added / after counts and the first / last ID for both idea kinds;
- locale-specific title and body length limits;
- every new ID and one internal editorial category;
- the maximum allowed run of one category in source order;
- phrases that are disallowed in that batch because they tend to produce unsupported health claims or canned, synthetic-sounding copy.

A manifest never contains titles or bodies. Categories are review aids, not runtime labels or selection weights.

## Voice checklist

The goal is a small human interruption that makes standing up feel easy.

- Keep some lines plain. A joke does not need a lesson attached.
- Mix workplace teasing, body observations, modest health facts, everyday scenes, emotional care, and tiny missions.
- Vary rhythm and point of view. Do not make every line follow “notice → permission → uplifting conclusion”.
- Prefer ordinary speech over slogans: short fragments, a dry aside, or an unfinished-feeling thought are welcome.
- Microbreak copy should land quickly and suggest at most one or two actions.
- A full-break entry may take a small scene or paragraph, but it should still sound spoken rather than written for a poster.
- Use only stable, modest health guidance. Avoid diagnoses, guaranteed outcomes, precise risk percentages, and unsourced “research shows” claims.
- Localize rather than transliterate. `zh-TW` uses natural Taiwan vocabulary; English should sound written in English, not translated line by line.

## Category set

- `workday-banter`: meetings, tabs, code, inboxes, deadlines, and office absurdity.
- `body-banter`: neck, shoulders, back, hands, eyes, legs, and posture talking back.
- `health-note`: restrained, actionable facts about blinking, distance viewing, movement, hydration, and posture changes.
- `emotional-care`: pressure, frustration, boundaries, fatigue, and self-kindness without motivational-poster language.
- `daily-life`: windows, cups, plants, hallways, weather, snacks, and other screen-free details.
- `mini-mission`: one concrete, low-friction thing to do during the break.

The manifest interleaves categories because full-break prompts rotate in source order from a stable starting point.

## Editing workflow

1. Read the current final IDs and choose the next continuous ranges.
2. Add or update the batch manifest without copying any prompt text into it.
3. Write all three official locale entries with the same ordered IDs.
4. Run `python3 scripts/sync_desktop_break_ideas.py`; fix every reported source or manifest error.
5. Run the targeted content tests, full tests, typecheck, and desktop build.
6. Review the shortest and longest entries, category distribution, and a random cross-language sample aloud.

Do not edit `registry.generated.json` by hand.
