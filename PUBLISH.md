# Publishing

The artifact is built and ready. Only the submit form is left, and it needs a GitHub sign-in, which
only you can do.

## The file

`C:\Users\world\Documents\HermesWorkspace\mods\chatheads\dist\chatheads-0.1.1.hnmod` — 8440 bytes
SHA-256 `16f5e1db33255c550c087fcf3ed982a4be9abf61b547b470a08f471d309ad1fe`

It holds `horizon-native.json` (game 26.3, schema 1, hooks 2, pinned to Horizon `4aa6854` and
horizon-mixins `a2af5c0`) and `mixins/chatheads/`.

## The form

https://playhorizon.dev/mods/submit — GitHub sign-in, then:

- **Project id**: `chatheads` (free text; the existing id is what makes this an update)
- **Name**: ChatHeads
- **Summary** (≤160 chars): A player's head left of every chat line they wrote.
- **Description** (first paragraph only is kept): each chat line from a player gets their 8x8 skin face
  in the strip to its left; lines without a player stay as they are, and the face fades with its line.
  Settings in Options > Mods.
- **Authors**: Alexander (Domanffe)
- **Licence**: MIT
- **Source**: https://github.com/Domanffe/chatheads
- **File**: `chatheads-0.1.1.hnmod`
- **Changelog**: Rebuilt on the 2026-11-01 Nightly (Horizon `4aa6854`). The three hook points it shares
  (mods, settings, settings_click) are named now instead of pinned to a source target, so a Horizon
  update that moves them no longer breaks the bundle.
- Public key and signature: leave empty (unsigned).

Known limit to mention if a reviewer asks: the chat's own hit-test does not know about the 10 px
indent, so clicking a link in chat lands 10 px left of its pixels. horizon-mixins refuses the
injection that would fix it (`E_UNSUPPORTED_CONTROL_FLOW`), so it needs a game-side hook.
