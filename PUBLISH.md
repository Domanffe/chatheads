# Publishing

The artifact is built and ready. Only the submit form is left, and it needs a GitHub sign-in, which
only you can do.

## The file

`E:\Projects\hz-mods\dist\chatheads-0.1.0.hnmod` — 8366 bytes
SHA-256 `c20605c24e29a762b9aea85cba1b0cc7955955a40a031efae70a5b01e4e79df4`

It holds `horizon-native.json` (game 26.3, schema 1, pinned to Horizon `a46ce68` and horizon-mixins
`3f99a5c6`) and `mixins/chatheads/`.

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
- **File**: `chatheads-0.1.0.hnmod`
- **Changelog**: First release: native mod for Horizon 26.3.
- Public key and signature: leave empty (unsigned).

Known limit to mention if a reviewer asks: the chat's own hit-test does not know about the 10 px
indent, so clicking a link in chat lands 10 px left of its pixels. horizon-mixins refuses the
injection that would fix it (`E_UNSUPPORTED_CONTROL_FLOW`), so it needs a game-side hook.
