# ChatHeads

![ChatHeads](docs/icon.png)

A player's 8x8 skin face to the left of every chat line they wrote, for
[Horizon](https://github.com/horizon-rust/horizon). A native mod (a horizon-mixins bundle), built into
the game by the launcher.

## What it does

- The face comes from the skin the game resolved for that profile; with none resolved, a stable default
  skin picked by the name, so a player always keeps the same head.
- System messages, `/me` lines and wrapped continuations have no `<Name>` prefix and get no head.
- The head fades with its line, and the chat's own rules hold: F1 hides it, an open chat shows it over
  its input.

## Settings

**Options > Mods > ChatHeads > Settings** — one value: chat heads on or off.

## Known limit

The chat's hit-test does not know about the 10 px indent, so clicking a link in chat lands 10 px left of
its pixels. horizon-mixins refuses the injection that would fix it (`E_UNSUPPORTED_CONTROL_FLOW`), so it
needs a game-side hook.

## Building

The bundle is `mixins/chatheads`; `runtime.rs` is also the standalone crate for its tests:

```sh
cargo test
```

The game build is the launcher's own:

```sh
horizon-launcher native-build "…/mixins/chatheads" --trust-native-code
```

It pins Horizon commit `a46ce6898bd72016e061aaec1afc1375b384deac` and hooks `Game::draw_chat` (HEAD and
RETURN) and `Chat::draw` (a variable), so it builds on that commit's source.

## Publishing

See [PUBLISH.md](PUBLISH.md).
